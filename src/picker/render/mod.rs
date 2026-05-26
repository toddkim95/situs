use std::time::Duration;

mod candidate;
mod footer;
mod header;
pub(super) mod helpers;
mod inspect;

use crate::history::now_seconds;
use crate::i18n::{I18n, MessageKey};
use crate::model::{Candidate, ContextFilter, MatchScope, SourceFilter};
use crate::picker::input::PickerState;

use super::path::common_directory_prefix;
use super::style::{dim, green, yellow};
use candidate::{candidate_header_line, candidate_line};
use footer::{help_line, help_overlay_lines, query_cursor_column, query_line};
use header::{header_line, HeaderState};
use helpers::format_elapsed;
use inspect::inspect_lines;

pub(super) fn loading_lines(
    command: &str,
    spinner: &str,
    elapsed: Duration,
    max_rows: usize,
    width: usize,
) -> Vec<String> {
    let i18n = I18n::from_env();
    let mut lines = vec![
        header_line(
            HeaderState {
                visible_count: 0,
                candidate_count: 0,
                include_failed: false,
                inspect: false,
                source_filter: SourceFilter::All,
                context_filter: ContextFilter::All,
                message: None,
            },
            width,
            i18n,
        ),
        format!(
            "  {}  {}",
            green(spinner),
            format!(
                "{} {}",
                i18n.text(MessageKey::PickerLoadingHistory),
                dim(&format_elapsed(elapsed))
            )
        ),
    ];
    while lines.len() + 2 < max_rows {
        lines.push(String::new());
    }
    lines.push(query_line(command, width, i18n));
    lines.push(help_line(width, i18n));
    lines
}

pub(super) fn empty_history_lines(command: &str, max_rows: usize, width: usize) -> Vec<String> {
    let i18n = I18n::from_env();
    let mut lines = vec![
        header_line(
            HeaderState {
                visible_count: 0,
                candidate_count: 0,
                include_failed: false,
                inspect: false,
                source_filter: SourceFilter::All,
                context_filter: ContextFilter::All,
                message: None,
            },
            width,
            i18n,
        ),
        format!("  {}", yellow(i18n.text(MessageKey::PickerNoHistory))),
        format!("  {}", dim(i18n.text(MessageKey::PickerNoHistoryHint))),
    ];
    while lines.len() + 2 < max_rows {
        lines.push(String::new());
    }
    lines.push(query_line(command, width, i18n));
    lines.push(help_line(width, i18n));
    lines
}

pub(super) fn picker_lines(
    _command: &str,
    all_candidates: &[Candidate],
    visible: &[(usize, &Candidate)],
    _scope: MatchScope,
    state: &PickerState,
    max_rows: usize,
    width: usize,
) -> Vec<String> {
    let i18n = I18n::from_env();
    let mut lines = Vec::new();
    let now = now_seconds();

    let body_capacity = max_rows.saturating_sub(3);
    let has_candidate_header = !state.inspect && !visible.is_empty();
    let max_candidates = body_capacity
        .saturating_sub(usize::from(has_candidate_header))
        .max(1);
    let visible_len = visible.len();

    let (start, end) = if visible_len <= max_candidates {
        (0, visible_len)
    } else {
        let half = max_candidates / 2;
        if state.selected <= half {
            (0, max_candidates)
        } else if state.selected + half >= visible_len {
            (visible_len - max_candidates, visible_len)
        } else {
            (
                state.selected - half,
                state.selected - half + max_candidates,
            )
        }
    };

    lines.push(header_line(
        HeaderState {
            visible_count: visible.len(),
            candidate_count: all_candidates.len(),
            include_failed: state.include_failed,
            inspect: state.inspect,
            source_filter: state.source_filter,
            context_filter: state.context_filter,
            message: state.message.as_deref(),
        },
        width,
        i18n,
    ));

    let mut body_lines = Vec::new();
    if state.show_help {
        body_lines.extend(help_overlay_lines(width, i18n));
    } else if state.inspect {
        if let Some((_, candidate)) = visible.get(state.selected) {
            body_lines.extend(inspect_lines(candidate, now, i18n));
        } else {
            body_lines.push(format!(
                "  {}",
                dim(i18n.text(MessageKey::PickerNoSelected))
            ));
        }
    } else if visible.is_empty() {
        body_lines.push(format!("  {}", dim(i18n.text(MessageKey::PickerNoMatches))));
    } else {
        let rendered = visible[start..end].iter().map(|(_, candidate)| *candidate);
        let common_cwd_prefix = common_directory_prefix(rendered);
        if has_candidate_header {
            body_lines.push(candidate_header_line(width, i18n));
        }
        for display_index in (start..end).rev() {
            if let Some((_, candidate)) = visible.get(display_index) {
                body_lines.push(candidate_line(
                    display_index,
                    candidate,
                    state.selected,
                    common_cwd_prefix.as_deref(),
                    width,
                    now,
                ));
            }
        }
    }

    body_lines.truncate(body_capacity);
    while lines.len() + body_lines.len() + 2 < max_rows {
        lines.push(String::new());
    }
    lines.extend(body_lines);
    lines.push(query_line(&state.query, width, i18n));
    lines.push(help_line(width, i18n));

    lines
}

pub(super) fn query_cursor_position(
    query: &str,
    cursor: usize,
    max_rows: usize,
    width: usize,
) -> (usize, u16) {
    (
        max_rows.saturating_sub(2),
        query_cursor_column(query, cursor, width),
    )
}

#[cfg(test)]
mod tests {
    use super::footer::query_prefix_width;
    use super::helpers::{truncate_end, truncate_start};
    use super::*;
    use crate::model::CandidateSource;
    use crate::picker::width::visible_width;

    fn candidate(timestamp: u64, status: i32, cwd: &str, command: &str) -> Candidate {
        Candidate {
            timestamp,
            status,
            cwd: cwd.to_string(),
            command: command.to_string(),
            source: CandidateSource::Local,
            run_count: 1,
            success_count: usize::from(status == 0),
        }
    }

    fn state(query: &str) -> PickerState {
        PickerState {
            selected: 0,
            query: query.to_string(),
            query_cursor: query.len(),
            query_synced_to_selection: false,
            filter: String::new(),
            scope_anchor: query.to_string(),
            scope_words: 1,
            include_failed: false,
            inspect: false,
            show_help: false,
            source_filter: SourceFilter::All,
            context_filter: ContextFilter::All,
            message: None,
        }
    }

    #[test]
    fn picker_lines_render_header_before_candidates() {
        let candidates = vec![
            candidate(30, 0, "/latest", "source ~/.zshrc"),
            candidate(20, 0, "/middle", "source ~/.zshrc"),
            candidate(10, 0, "/old", "source ~/.zshrc"),
        ];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("source");
        state.scope_anchor = "source ~/.zshrc".to_string();
        let lines = picker_lines(
            "source",
            &candidates,
            &visible,
            MatchScope {
                words: 1,
                total_words: 2,
            },
            &state,
            8,
            120,
        );

        assert!(lines[0].contains("Situs"));
        assert!(lines[0].contains("3/3 results"));
        assert!(lines[2].contains("Command"));
        assert!(lines[3].contains("/old"));
        assert!(lines[4].contains("/middle"));
        assert!(lines[5].contains("/latest"));
        assert!(lines[5].contains(">"));
        assert!(lines[6].contains(">"));
        assert!(lines[6].contains("source"));
        assert!(!lines[6].contains("tab"));
        assert!(lines[7].contains("tab"));
        assert!(lines[7].contains("enter"));
        assert!(!lines.iter().any(|line| line.contains("query ")));
    }

    #[test]
    fn picker_lines_anchor_single_candidate_directly_above_footer() {
        let candidates = vec![candidate(
            30,
            0,
            "/project",
            "cargo install --path . --force",
        )];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let state = state("cargo");

        let lines = picker_lines(
            "cargo",
            &candidates,
            &visible,
            MatchScope {
                words: 1,
                total_words: 1,
            },
            &state,
            8,
            120,
        );

        assert!(lines[5].contains("cargo install --path . --force"));
        assert!(lines[6].contains(">"));
        assert!(lines[6].contains("cargo"));
        assert!(lines[7].contains("enter"));
        assert!(!lines[1..6]
            .iter()
            .take(4)
            .any(|line| line.contains("cargo install --path . --force")));
    }

    #[test]
    fn picker_lines_mask_common_directory_prefix_for_visible_rows() {
        let candidates = vec![
            candidate(30, 0, "/Users/me/work/apps/api", "cargo build"),
            candidate(20, 0, "/Users/me/work/apps/web", "cargo test"),
        ];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let state = state("cargo");

        let lines = picker_lines(
            "cargo",
            &candidates,
            &visible,
            MatchScope {
                words: 1,
                total_words: 1,
            },
            &state,
            8,
            140,
        );

        assert!(lines.iter().any(|line| line.contains("*/api")));
        assert!(lines.iter().any(|line| line.contains("*/web")));
        assert!(!lines
            .iter()
            .any(|line| line.contains("/Users/me/work/apps/api")));
    }

    #[test]
    fn picker_lines_show_failure_toggle_status() {
        let candidates = vec![candidate(30, 1, "/failed", "cargo build")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("cargo build");
        state.scope_words = 2;
        state.include_failed = true;

        let lines = picker_lines(
            "cargo build",
            &candidates,
            &visible,
            MatchScope {
                words: 2,
                total_words: 2,
            },
            &state,
            8,
            120,
        );

        assert!(lines[5].contains("EXIT 1"));
        assert!(lines[0].contains("all history"));
    }

    #[test]
    fn picker_lines_hide_raw_scope_status() {
        let candidates = vec![candidate(30, 0, "/project", "cargo build")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let state = state("cargo build");

        let lines = picker_lines(
            "cargo build",
            &candidates,
            &visible,
            MatchScope {
                words: 1,
                total_words: 2,
            },
            &state,
            8,
            120,
        );

        assert!(!lines.iter().any(|line| line.contains("scope")));
        assert!(!lines.iter().any(|line| line.contains("words")));
        assert!(!lines.iter().any(|line| line.contains("shown")));
    }

    #[test]
    fn picker_lines_show_selected_candidate_details_in_inspect_mode() {
        let candidates = vec![Candidate {
            source: CandidateSource::Atuin,
            run_count: 3,
            success_count: 1,
            ..candidate(30, 2, "/failed", "cargo build --release")
        }];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("cargo build");
        state.scope_words = 2;
        state.include_failed = true;
        state.inspect = true;

        let lines = picker_lines(
            "cargo build",
            &candidates,
            &visible,
            MatchScope {
                words: 2,
                total_words: 2,
            },
            &state,
            10,
            120,
        );

        assert!(lines[0].contains("Inspect"));
        assert!(lines.iter().any(|line| line.contains("command")));
        assert!(lines
            .iter()
            .any(|line| line.contains("cargo build --release")));
        assert!(lines.iter().any(|line| line.contains("/failed")));
        assert!(lines.iter().any(|line| line.contains("exit 2")));
        assert!(lines.iter().any(|line| line.contains("ATUIN")));
        assert!(lines.iter().any(|line| line.contains("3 total")));
    }

    #[test]
    fn picker_lines_do_not_panic_on_narrow_width() {
        let candidates = vec![candidate(30, 0, "/project", "cargo build")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("cargo build");
        state.scope_words = 2;

        let lines = picker_lines(
            "cargo build",
            &candidates,
            &visible,
            MatchScope {
                words: 2,
                total_words: 2,
            },
            &state,
            8,
            80,
        );

        assert!(lines[0].contains("Situs"));
        assert!(lines[6].contains(">"));
        assert!(lines[6].contains("cargo build"));
        assert!(lines[7].contains("enter"));
        assert!(lines[7].contains("run"));
    }

    #[test]
    fn picker_lines_show_help_overlay_and_source_filter_chip() {
        let candidates = vec![candidate(30, 0, "/project", "cargo build")];
        let visible = candidates.iter().enumerate().collect::<Vec<_>>();
        let mut state = state("cargo build");
        state.show_help = true;
        state.source_filter = SourceFilter::Atuin;
        state.context_filter = ContextFilter::Workspace;
        state.message = Some("source: atuin".to_string());

        let lines = picker_lines(
            "cargo build",
            &candidates,
            &visible,
            MatchScope {
                words: 2,
                total_words: 2,
            },
            &state,
            12,
            140,
        );

        assert!(lines[0].contains("atuin"));
        assert!(lines[0].contains("workspace"));
        assert!(lines.iter().any(|line| line.contains("Keyboard")));
        assert!(lines.iter().any(|line| line.contains("alt-enter")));
        assert!(lines.iter().any(|line| line.contains("ctrl-y")));
        assert!(lines.iter().any(|line| line.contains("ctrl-/")));
    }

    #[test]
    fn query_cursor_column_tracks_visible_query_position() {
        assert_eq!(query_cursor_column("cargo install", 0, 120), 12);
        assert_eq!(query_cursor_column("cargo install", 5, 120), 17);
        assert_eq!(query_cursor_column("cargo install", 13, 10), 8);
    }

    #[test]
    fn render_width_counts_common_wide_unicode_columns() {
        assert_eq!(visible_width("한글"), 4);
        assert_eq!(visible_width("cargo 한글"), 10);
        assert_eq!(query_cursor_column("한글", "한글".len(), 120), 16);
    }

    #[test]
    fn localized_picker_labels_account_for_wide_characters() {
        let i18n = I18n::new(crate::i18n::Locale::Ko);
        let header = header_line(
            HeaderState {
                visible_count: 1,
                candidate_count: 3,
                include_failed: false,
                inspect: false,
                source_filter: SourceFilter::All,
                context_filter: ContextFilter::All,
                message: None,
            },
            120,
            i18n,
        );
        let query = query_line("cargo", 120, i18n);
        let help = help_line(132, i18n);

        assert!(header.contains("검색"));
        assert!(query.contains("검색"));
        assert_eq!(query_prefix_width(i18n), 10);
        assert!(help.contains("종료"));
        assert!(help.contains("실행"));
    }

    #[test]
    fn truncation_respects_wide_unicode_columns() {
        assert_eq!(truncate_end("한글cargo", 5), "한...");
        assert_eq!(truncate_start("/tmp/한글/project", 13), "...글/project");
    }
}
