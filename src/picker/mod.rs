mod clipboard;
mod fullscreen;
mod input;
mod keymap;
mod keys;
mod path;
mod plain;
mod render;
mod session;
mod style;
mod tui;
mod viewport;
pub(crate) mod width;

use std::env;

use crate::context::CommandContext;
use crate::error::{cli_error, CliResult};
use crate::model::{
    Candidate, CandidateSource, ContextFilter, HistorySource, MatchFilters, PickerMode,
    SourceFilter,
};
use crate::terminal::write_to_terminal;

use plain::choose_candidate_plain;
use session::PickerSession;
use tui::choose_candidate_inline;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SelectionAction {
    Run,
    CdOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PickerSelection {
    pub(crate) action: SelectionAction,
    pub(crate) candidate: Candidate,
    pub(crate) query: String,
}

pub(crate) fn choose_candidate(
    command: &str,
    include_failed: bool,
    picker_mode: PickerMode,
    context_filter: ContextFilter,
) -> CliResult<Option<PickerSelection>> {
    let command_context = CommandContext::capture();
    if should_use_tui() {
        match PickerSession::start(picker_mode) {
            Ok(mut session) => {
                return choose_candidate_inline(
                    &mut session,
                    command,
                    include_failed,
                    context_filter,
                    command_context,
                );
            }
            Err(error) => {
                write_to_terminal(format_args!(
                    "situs: interactive picker unavailable ({error}); using plain picker\n"
                ))?;
            }
        }
    }

    choose_candidate_plain(command, include_failed, context_filter, &command_context)
}

pub(crate) fn choose_candidate_tui_only(
    command: &str,
    include_failed: bool,
    picker_mode: PickerMode,
    context_filter: ContextFilter,
) -> CliResult<Option<PickerSelection>> {
    if !should_use_tui() {
        return Err(cli_error(
            "interactive picker is required for shell widget mode",
        ));
    }

    let mut session = PickerSession::start(picker_mode)
        .map_err(|error| cli_error(format!("interactive picker unavailable ({error})")))?;
    let command_context = CommandContext::capture();
    choose_candidate_inline(
        &mut session,
        command,
        include_failed,
        context_filter,
        command_context,
    )
}

fn selected_candidate(
    candidates: &[Candidate],
    visible: &[(usize, &Candidate)],
    state: &input::PickerState,
) -> Option<Candidate> {
    let (candidate_index, _) = visible.get(state.selected)?;
    candidates.get(*candidate_index).cloned()
}

fn match_filters<'a>(
    source_filter: SourceFilter,
    context_filter: ContextFilter,
    command_context: &'a CommandContext,
) -> MatchFilters<'a> {
    MatchFilters {
        source: source_filter,
        context: context_filter,
        current_dir: command_context.current_dir.as_deref(),
        workspace_root: command_context.workspace_root.as_deref(),
    }
}

fn selection_for_visible(
    candidates: &[Candidate],
    visible: &[(usize, &Candidate)],
    state: &input::PickerState,
    action: SelectionAction,
) -> Option<PickerSelection> {
    let (candidate_index, _) = visible.get(state.selected)?;
    let candidate = candidates.get(*candidate_index)?.clone();
    let query = if matches!(action, SelectionAction::CdOnly) && !state.query_synced_to_selection {
        candidate.command.clone()
    } else {
        state.query.clone()
    };
    Some(PickerSelection {
        action,
        candidate,
        query,
    })
}

fn delete_candidate_records(
    records: &mut Vec<crate::model::Record>,
    candidate: &Candidate,
) -> usize {
    let before = records.len();
    records.retain(|record| {
        !(record.cwd == candidate.cwd
            && record.command == candidate.command
            && record_source_is_in_candidate(record.source, candidate.source))
    });
    before - records.len()
}

fn record_source_is_in_candidate(source: HistorySource, candidate_source: CandidateSource) -> bool {
    match candidate_source {
        CandidateSource::Mixed => true,
        CandidateSource::Local => source == HistorySource::Local,
        CandidateSource::Atuin => source == HistorySource::Atuin,
    }
}

fn should_use_tui() -> bool {
    let term = env::var("TERM").ok();
    should_use_tui_from(env::var_os("SITUS_PLAIN").is_some(), term.as_deref())
}

fn should_use_tui_from(plain_mode_enabled: bool, term: Option<&str>) -> bool {
    !plain_mode_enabled && term.map(|term| term != "dumb").unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::command_word_count;
    use crate::model::Record;

    fn candidate(cwd: &str, command: &str) -> Candidate {
        Candidate {
            timestamp: 1,
            status: 0,
            cwd: cwd.to_string(),
            command: command.to_string(),
            source: CandidateSource::Local,
            run_count: 1,
            success_count: 1,
        }
    }

    fn state(query: &str) -> input::PickerState {
        input::PickerState {
            selected: 0,
            query: query.to_string(),
            query_cursor: query.len(),
            query_synced_to_selection: false,
            filter: String::new(),
            scope_anchor: query.to_string(),
            scope_words: command_word_count(query).max(1),
            include_failed: false,
            inspect: false,
            show_help: false,
            source_filter: SourceFilter::All,
            context_filter: ContextFilter::All,
            message: None,
        }
    }

    #[test]
    fn tui_detection_rejects_plain_mode_and_dumb_term() {
        assert!(!should_use_tui_from(true, Some("xterm-256color")));
        assert!(!should_use_tui_from(false, Some("dumb")));
        assert!(should_use_tui_from(false, Some("xterm-256color")));
        assert!(should_use_tui_from(false, None));
    }

    #[test]
    fn cd_only_selection_uses_candidate_command_when_query_is_only_search_text() {
        let candidates = vec![candidate("/project", "cargo install --path . --force")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let state = state("cargo");

        let selection =
            selection_for_visible(&candidates, &visible, &state, SelectionAction::CdOnly).unwrap();

        assert_eq!(selection.action, SelectionAction::CdOnly);
        assert_eq!(selection.candidate.cwd, "/project");
        assert_eq!(selection.query, "cargo install --path . --force");
    }

    #[test]
    fn cd_only_selection_preserves_synced_or_edited_query() {
        let candidates = vec![candidate("/project", "cargo install --path . --force")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("cargo install --path . --locked");
        state.query_synced_to_selection = true;
        state.scope_anchor = "cargo".to_string();
        state.scope_words = 1;

        let selection =
            selection_for_visible(&candidates, &visible, &state, SelectionAction::CdOnly).unwrap();

        assert_eq!(selection.query, "cargo install --path . --locked");
    }

    #[test]
    fn delete_candidate_records_respects_candidate_source() {
        let mut records = vec![
            Record {
                timestamp: 1,
                status: 0,
                cwd: "/project".to_string(),
                command: "cargo build".to_string(),
                source: HistorySource::Local,
            },
            Record {
                timestamp: 2,
                status: 0,
                cwd: "/project".to_string(),
                command: "cargo build".to_string(),
                source: HistorySource::Atuin,
            },
        ];
        let candidate = Candidate {
            source: CandidateSource::Local,
            ..candidate("/project", "cargo build")
        };

        let deleted = delete_candidate_records(&mut records, &candidate);

        assert_eq!(deleted, 1);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].source, HistorySource::Atuin);
    }
}
