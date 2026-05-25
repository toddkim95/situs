use std::collections::HashMap;

use crate::command::{command_has_prefix_words, command_prefix_words, command_word_count};
#[cfg(test)]
use crate::model::SourceFilter;
use crate::model::{Candidate, CandidateSource, HistorySource, MatchFilters, MatchScope, Record};

use super::filters::{context_matches, source_matches, PreparedFilters};

#[cfg(test)]
pub(crate) fn matching_candidates(
    records: &[Record],
    command: &str,
    include_failed: bool,
) -> (Vec<Candidate>, MatchScope) {
    matching_candidates_with_source_filter(records, command, include_failed, SourceFilter::All)
}

#[cfg(test)]
pub(crate) fn matching_candidates_with_source_filter(
    records: &[Record],
    command: &str,
    include_failed: bool,
    source_filter: SourceFilter,
) -> (Vec<Candidate>, MatchScope) {
    matching_candidates_with_filters(
        records,
        command,
        include_failed,
        MatchFilters {
            source: source_filter,
            ..MatchFilters::default()
        },
    )
}

pub(crate) fn matching_candidates_with_filters(
    records: &[Record],
    command: &str,
    include_failed: bool,
    filters: MatchFilters<'_>,
) -> (Vec<Candidate>, MatchScope) {
    let total_words = command_word_count(command).max(1);
    let scope = best_match_scope_with_filters(records, command, include_failed, filters);
    (
        scoped_candidates_with_filters(records, command, include_failed, scope.words, filters),
        MatchScope {
            words: scope.words,
            total_words,
        },
    )
}

#[cfg(test)]
pub(crate) fn best_match_scope(
    records: &[Record],
    command: &str,
    include_failed: bool,
) -> MatchScope {
    best_match_scope_with_source_filter(records, command, include_failed, SourceFilter::All)
}

#[cfg(test)]
pub(crate) fn best_match_scope_with_source_filter(
    records: &[Record],
    command: &str,
    include_failed: bool,
    source_filter: SourceFilter,
) -> MatchScope {
    best_match_scope_with_filters(
        records,
        command,
        include_failed,
        MatchFilters {
            source: source_filter,
            ..MatchFilters::default()
        },
    )
}

pub(crate) fn best_match_scope_with_filters(
    records: &[Record],
    command: &str,
    include_failed: bool,
    filters: MatchFilters<'_>,
) -> MatchScope {
    let total_words = command_word_count(command).max(1);
    let filters = PreparedFilters::from(filters);
    let mut path_cache = HashMap::new();

    if command.is_empty() {
        return MatchScope {
            words: 1,
            total_words: 1,
        };
    }

    for words in (1..=total_words).rev() {
        if has_scoped_candidate(
            records,
            command,
            include_failed,
            words,
            &filters,
            &mut path_cache,
        ) {
            return MatchScope { words, total_words };
        }
    }

    MatchScope {
        words: total_words,
        total_words,
    }
}

#[cfg(test)]
pub(crate) fn scoped_candidates(
    records: &[Record],
    command: &str,
    include_failed: bool,
    scope_words: usize,
) -> Vec<Candidate> {
    scoped_candidates_with_source_filter(
        records,
        command,
        include_failed,
        scope_words,
        SourceFilter::All,
    )
}

#[cfg(test)]
pub(crate) fn scoped_candidates_with_source_filter(
    records: &[Record],
    command: &str,
    include_failed: bool,
    scope_words: usize,
    source_filter: SourceFilter,
) -> Vec<Candidate> {
    scoped_candidates_with_filters(
        records,
        command,
        include_failed,
        scope_words,
        MatchFilters {
            source: source_filter,
            ..MatchFilters::default()
        },
    )
}

pub(crate) fn scoped_candidates_with_filters(
    records: &[Record],
    command: &str,
    include_failed: bool,
    scope_words: usize,
    filters: MatchFilters<'_>,
) -> Vec<Candidate> {
    let prefix_words = command_prefix_words(command, scope_words);
    let is_empty_query = command.is_empty();
    if !is_empty_query && prefix_words.is_empty() {
        return Vec::new();
    }

    let mut by_command_and_cwd = HashMap::<(String, String), Candidate>::new();
    let filters = PreparedFilters::from(filters);
    let mut path_cache = HashMap::new();

    for record in records {
        if (!is_empty_query && !command_has_prefix_words(&record.command, &prefix_words))
            || (!include_failed && record.status != 0)
            || !source_matches(record.source, filters.source)
            || !context_matches(record, &filters, &mut path_cache)
        {
            continue;
        }

        let candidate = by_command_and_cwd
            .entry((record.command.clone(), record.cwd.clone()))
            .or_insert(Candidate {
                cwd: record.cwd.clone(),
                command: record.command.clone(),
                timestamp: record.timestamp,
                status: record.status,
                source: candidate_source(record.source),
                run_count: 0,
                success_count: 0,
            });

        candidate.run_count += 1;
        if record.status == 0 {
            candidate.success_count += 1;
        }
        candidate.source = merge_candidate_source(candidate.source, record.source);

        if record.timestamp >= candidate.timestamp {
            candidate.timestamp = record.timestamp;
            candidate.status = record.status;
        }
    }

    let mut candidates = by_command_and_cwd.into_values().collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .timestamp
            .div_euclid(300)
            .cmp(&left.timestamp.div_euclid(300))
            .then_with(|| right.success_count.cmp(&left.success_count))
            .then_with(|| right.run_count.cmp(&left.run_count))
            .then_with(|| right.timestamp.cmp(&left.timestamp))
            .then_with(|| left.command.cmp(&right.command))
            .then_with(|| left.cwd.cmp(&right.cwd))
    });
    candidates
}

fn candidate_source(source: HistorySource) -> CandidateSource {
    match source {
        HistorySource::Local => CandidateSource::Local,
        HistorySource::Atuin => CandidateSource::Atuin,
    }
}

fn merge_candidate_source(current: CandidateSource, incoming: HistorySource) -> CandidateSource {
    match (current, incoming) {
        (CandidateSource::Mixed, _) => CandidateSource::Mixed,
        (CandidateSource::Local, HistorySource::Local) => CandidateSource::Local,
        (CandidateSource::Atuin, HistorySource::Atuin) => CandidateSource::Atuin,
        _ => CandidateSource::Mixed,
    }
}

fn has_scoped_candidate<'a>(
    records: &'a [Record],
    command: &str,
    include_failed: bool,
    scope_words: usize,
    filters: &PreparedFilters,
    path_cache: &mut HashMap<&'a str, Vec<String>>,
) -> bool {
    if command.is_empty() {
        return records.iter().any(|record| {
            (include_failed || record.status == 0)
                && source_matches(record.source, filters.source)
                && context_matches(record, filters, path_cache)
        });
    }

    let prefix_words = command_prefix_words(command, scope_words);
    if prefix_words.is_empty() {
        return false;
    }

    records.iter().any(|record| {
        command_has_prefix_words(&record.command, &prefix_words)
            && (include_failed || record.status == 0)
            && source_matches(record.source, filters.source)
            && context_matches(record, filters, path_cache)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::ContextFilter;

    fn record(timestamp: u64, status: i32, cwd: &str, command: &str) -> Record {
        Record {
            timestamp,
            status,
            cwd: cwd.to_string(),
            command: command.to_string(),
            source: HistorySource::Local,
        }
    }

    #[test]
    fn candidates_prefer_successful_recent_records() {
        let records = vec![
            record(1, 0, "/old", "cargo build"),
            record(3, 1, "/wrong", "cargo build"),
            record(2, 0, "/new", "cargo build"),
        ];

        let (candidates, scope) = matching_candidates(&records, "cargo build", false);
        assert_eq!(scope.words, 2);
        assert_eq!(scope.total_words, 2);
        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.cwd.as_str())
                .collect::<Vec<_>>(),
            vec!["/new", "/old"]
        );
    }

    #[test]
    fn candidates_fall_back_to_command_signature() {
        let records = vec![record(1, 0, "/project", "cargo build")];

        let (candidates, scope) = matching_candidates(&records, "cargo build --release", false);
        assert_eq!(scope.words, 2);
        assert_eq!(scope.total_words, 3);
        assert_eq!(candidates[0].cwd, "/project");
        assert_eq!(candidates[0].command, "cargo build");
    }

    #[test]
    fn candidates_ignore_failed_records_by_default() {
        let records = vec![
            record(1, 1, "/wrong", "cargo install"),
            record(2, 0, "/project", "cargo install --path . --force"),
        ];

        let (candidates, scope) = matching_candidates(&records, "cargo install", false);
        assert_eq!(scope.words, 2);
        assert_eq!(scope.total_words, 2);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].cwd, "/project");
        assert_eq!(candidates[0].command, "cargo install --path . --force");
    }

    #[test]
    fn scoped_candidates_can_broaden_command_prefix() {
        let records = vec![
            record(1, 0, "/project", "cargo install --path . --force"),
            record(2, 0, "/project", "cargo build --release"),
        ];

        let cargo_candidates =
            scoped_candidates(&records, "cargo install --path . --force", false, 1);
        assert_eq!(cargo_candidates.len(), 2);

        let install_candidates =
            scoped_candidates(&records, "cargo install --path . --force", false, 2);
        assert_eq!(install_candidates.len(), 1);
        assert_eq!(
            install_candidates[0].command,
            "cargo install --path . --force"
        );
    }

    #[test]
    fn scoped_candidates_handles_large_history_smoke() {
        let mut records = Vec::new();
        for index in 0..10_000 {
            let command = if index % 2 == 0 {
                "cargo install --path . --force"
            } else {
                "npm test"
            };
            records.push(record(index, 0, &format!("/tmp/project-{index}"), command));
        }

        let candidates = scoped_candidates(&records, "cargo i", false, 2);

        assert_eq!(candidates.len(), 5_000);
        assert_eq!(candidates[0].command, "cargo install --path . --force");
    }

    #[test]
    fn best_match_scope_checks_existence_without_building_candidates() {
        let records = vec![
            record(1, 1, "/failed", "cargo build"),
            record(2, 0, "/project", "cargo install --path ."),
        ];
        let filters = PreparedFilters::from(MatchFilters::default());
        let mut path_cache = HashMap::new();

        assert!(!has_scoped_candidate(
            &records,
            "cargo build",
            false,
            2,
            &filters,
            &mut path_cache
        ));
        assert!(has_scoped_candidate(
            &records,
            "cargo build",
            true,
            2,
            &filters,
            &mut path_cache
        ));
        assert_eq!(
            best_match_scope(&records, "cargo install --path . --force", false).words,
            4
        );
    }

    #[test]
    fn candidates_can_filter_and_merge_sources() {
        let records = vec![
            record(1, 0, "/project", "cargo build"),
            Record {
                timestamp: 2,
                status: 0,
                cwd: "/project".to_string(),
                command: "cargo build".to_string(),
                source: HistorySource::Atuin,
            },
        ];

        let local = scoped_candidates_with_source_filter(
            &records,
            "cargo build",
            false,
            2,
            SourceFilter::Local,
        );
        let atuin = scoped_candidates_with_source_filter(
            &records,
            "cargo build",
            false,
            2,
            SourceFilter::Atuin,
        );
        let all = scoped_candidates_with_source_filter(
            &records,
            "cargo build",
            false,
            2,
            SourceFilter::All,
        );

        assert_eq!(local[0].source, CandidateSource::Local);
        assert_eq!(atuin[0].source, CandidateSource::Atuin);
        assert_eq!(all[0].source, CandidateSource::Mixed);
        assert_eq!(all[0].run_count, 2);
    }

    #[test]
    fn candidates_can_filter_by_directory_or_workspace_context() {
        let records = vec![
            record(1, 0, "/repo/api", "cargo test"),
            record(2, 0, "/repo/web", "cargo test"),
            record(3, 0, "/other", "cargo test"),
        ];

        let directory = scoped_candidates_with_filters(
            &records,
            "cargo test",
            false,
            2,
            MatchFilters {
                context: ContextFilter::Directory,
                current_dir: Some("/repo/api"),
                ..MatchFilters::default()
            },
        );
        let workspace = scoped_candidates_with_filters(
            &records,
            "cargo test",
            false,
            2,
            MatchFilters {
                context: ContextFilter::Workspace,
                workspace_root: Some("/repo"),
                ..MatchFilters::default()
            },
        );

        assert_eq!(directory.len(), 1);
        assert_eq!(directory[0].cwd, "/repo/api");
        assert_eq!(
            workspace
                .iter()
                .map(|candidate| candidate.cwd.as_str())
                .collect::<Vec<_>>(),
            vec!["/repo/web", "/repo/api"]
        );
    }

    #[test]
    fn workspace_context_handles_large_repeated_cwd_history() {
        let mut records = Vec::new();
        for index in 0..10_000 {
            let cwd = if index % 2 == 0 {
                "/repo/api"
            } else {
                "/elsewhere/api"
            };
            records.push(record(index, 0, cwd, "cargo test"));
        }

        let candidates = scoped_candidates_with_filters(
            &records,
            "cargo test",
            false,
            2,
            MatchFilters {
                context: ContextFilter::Workspace,
                workspace_root: Some("/repo"),
                ..MatchFilters::default()
            },
        );

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].cwd, "/repo/api");
        assert_eq!(candidates[0].run_count, 5_000);
    }

    #[test]
    fn empty_query_matches_all_recent_records() {
        let records = vec![
            record(1, 0, "/old", "cargo build"),
            record(2, 0, "/new", "npm test"),
        ];

        let (candidates, scope) = matching_candidates(&records, "", false);
        assert_eq!(scope.words, 1);
        assert_eq!(scope.total_words, 1);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].cwd, "/new");
        assert_eq!(candidates[0].command, "npm test");
        assert_eq!(candidates[1].cwd, "/old");
        assert_eq!(candidates[1].command, "cargo build");
    }
}
