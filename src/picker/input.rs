use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::command::command_word_count;
use crate::i18n::{I18n, MessageKey};
use crate::model::{Candidate, ContextFilter, SourceFilter};
use crate::picker::keymap::{key_intent, KeyIntent};

pub(super) struct PickerState {
    pub(super) selected: usize,
    pub(super) query: String,
    pub(super) query_cursor: usize,
    pub(super) query_synced_to_selection: bool,
    pub(super) filter: String,
    pub(super) scope_anchor: String,
    pub(super) scope_words: usize,
    pub(super) include_failed: bool,
    pub(super) inspect: bool,
    pub(super) show_help: bool,
    pub(super) source_filter: SourceFilter,
    pub(super) context_filter: ContextFilter,
    pub(super) message: Option<String>,
}

pub(super) enum PickerAction {
    KeepGoing,
    Run,
    CdOnly,
    PutOnly,
    CopySelected,
    DeleteSelected,
    Cancel,
}

pub(super) fn is_cancel_key(key: KeyEvent) -> bool {
    key.code == KeyCode::Esc
        || matches!(key.code, KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL))
}

pub(super) fn visible_candidates<'a>(
    candidates: &'a [Candidate],
    filter: &str,
) -> Vec<(usize, &'a Candidate)> {
    let filter = filter.trim();
    if filter.is_empty() {
        return candidates.iter().enumerate().collect();
    }

    candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| {
            contains_ignore_ascii_case(&candidate.cwd, filter)
                || contains_ignore_ascii_case(&candidate.command, filter)
        })
        .collect()
}

fn contains_ignore_ascii_case(value: &str, needle: &str) -> bool {
    if value.is_ascii() && needle.is_ascii() {
        let needle = needle.as_bytes();
        return value
            .as_bytes()
            .windows(needle.len())
            .any(|window| window.eq_ignore_ascii_case(needle));
    }

    value.to_lowercase().contains(&needle.to_lowercase())
}

pub(super) fn clamp_selection(state: &mut PickerState, visible_len: usize) {
    if visible_len == 0 {
        state.selected = 0;
    } else if state.selected >= visible_len {
        state.selected = visible_len - 1;
    }
}

pub(super) fn sync_query_to_candidate(state: &mut PickerState, candidate: &Candidate) {
    state.query = candidate.command.clone();
    state.query_cursor = state.query.len();
    state.query_synced_to_selection = true;
    state.filter.clear();
}

pub(super) fn handle_picker_key(
    key: KeyEvent,
    state: &mut PickerState,
    visible_len: usize,
) -> PickerAction {
    match key_intent(key) {
        KeyIntent::Run => PickerAction::Run,
        KeyIntent::Cancel => PickerAction::Cancel,
        KeyIntent::CdOnly => PickerAction::CdOnly,
        KeyIntent::PutOnly => PickerAction::PutOnly,
        KeyIntent::ToggleHelp => {
            toggle_help(state);
            PickerAction::KeepGoing
        }
        KeyIntent::CycleSource => {
            state.source_filter = state.source_filter.next();
            state.selected = 0;
            let i18n = I18n::from_env();
            state.message = Some(format!(
                "{}: {}",
                i18n.text(MessageKey::PickerMessageSource),
                state.source_filter.as_str()
            ));
            PickerAction::KeepGoing
        }
        KeyIntent::CycleContext => {
            state.context_filter = state.context_filter.next();
            state.selected = 0;
            let i18n = I18n::from_env();
            state.message = Some(format!(
                "{}: {}",
                i18n.text(MessageKey::PickerMessageContext),
                state.context_filter.as_str()
            ));
            PickerAction::KeepGoing
        }
        KeyIntent::MoveSelectionPageUp => {
            move_selection_page_up(state, visible_len);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveSelectionPageDown => {
            move_selection_page_down(state, visible_len);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveQueryLeft => {
            move_query_cursor_left(state);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveQueryRight => {
            move_query_cursor_right(state);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveSelectionUp => {
            move_selection_up(state, visible_len);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveSelectionDown => {
            move_selection_down(state, visible_len);
            PickerAction::KeepGoing
        }
        KeyIntent::MoveQueryHome => {
            state.query_cursor = 0;
            state.message = None;
            PickerAction::KeepGoing
        }
        KeyIntent::MoveQueryEnd => {
            state.query_cursor = state.query.len();
            state.message = None;
            PickerAction::KeepGoing
        }
        KeyIntent::Backspace => {
            pop_query_char(state);
            PickerAction::KeepGoing
        }
        KeyIntent::Delete => {
            delete_query_char(state);
            PickerAction::KeepGoing
        }
        KeyIntent::DeleteSelected => PickerAction::DeleteSelected,
        KeyIntent::ToggleFailures => {
            state.include_failed = !state.include_failed;
            state.selected = 0;
            let i18n = I18n::from_env();
            state.message = Some(if state.include_failed {
                i18n.text(MessageKey::PickerMessageShowingFailed)
                    .to_string()
            } else {
                i18n.text(MessageKey::PickerMessageHidingFailed).to_string()
            });
            PickerAction::KeepGoing
        }
        KeyIntent::ToggleInspect => {
            state.inspect = !state.inspect;
            state.message = None;
            PickerAction::KeepGoing
        }
        KeyIntent::ClearQuery => {
            clear_query(state);
            PickerAction::KeepGoing
        }
        KeyIntent::CopySelected => PickerAction::CopySelected,
        KeyIntent::InsertChar(character) => {
            push_query_char(state, character);
            PickerAction::KeepGoing
        }
        KeyIntent::Ignore => PickerAction::KeepGoing,
    }
}

fn toggle_help(state: &mut PickerState) {
    state.show_help = !state.show_help;
    state.message = None;
}

fn move_selection_up(state: &mut PickerState, visible_len: usize) {
    if visible_len == 0 {
        state.selected = 0;
    } else {
        state.selected = (state.selected + 1) % visible_len;
    }
    state.message = None;
}

fn move_selection_down(state: &mut PickerState, visible_len: usize) {
    if visible_len == 0 {
        state.selected = 0;
    } else if state.selected == 0 {
        state.selected = visible_len - 1;
    } else {
        state.selected -= 1;
    }
    state.message = None;
}

fn move_selection_page_up(state: &mut PickerState, visible_len: usize) {
    if visible_len == 0 {
        state.selected = 0;
    } else {
        state.selected = (state.selected + 5).min(visible_len - 1);
    }
    state.message = None;
}

fn move_selection_page_down(state: &mut PickerState, visible_len: usize) {
    if visible_len == 0 {
        state.selected = 0;
    } else {
        state.selected = state.selected.saturating_sub(5);
    }
    state.message = None;
}

fn push_query_char(state: &mut PickerState, character: char) {
    state.query.insert(state.query_cursor, character);
    state.query_cursor += character.len_utf8();
    reset_query_scope(state);
}

fn pop_query_char(state: &mut PickerState) {
    if state.query_cursor == 0 {
        return;
    }

    let previous = previous_char_boundary(&state.query, state.query_cursor);
    state.query.drain(previous..state.query_cursor);
    state.query_cursor = previous;
    reset_query_scope(state);
}

fn delete_query_char(state: &mut PickerState) {
    if state.query_cursor >= state.query.len() {
        return;
    }

    let next = next_char_boundary(&state.query, state.query_cursor);
    state.query.drain(state.query_cursor..next);
    reset_query_scope(state);
}

fn clear_query(state: &mut PickerState) {
    state.query.clear();
    state.query_cursor = 0;
    reset_query_scope(state);
}

fn reset_query_scope(state: &mut PickerState) {
    state.filter.clear();
    state.scope_anchor = state.query.clone();
    state.scope_words = command_word_count(&state.query).max(1);
    state.selected = 0;
    state.query_synced_to_selection = false;
    state.message = None;
}

fn move_query_cursor_left(state: &mut PickerState) {
    state.query_cursor = previous_char_boundary(&state.query, state.query_cursor);
    state.message = None;
}

fn move_query_cursor_right(state: &mut PickerState) {
    state.query_cursor = next_char_boundary(&state.query, state.query_cursor);
    state.message = None;
}

fn previous_char_boundary(value: &str, cursor: usize) -> usize {
    value[..cursor]
        .char_indices()
        .last()
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn next_char_boundary(value: &str, cursor: usize) -> usize {
    if cursor >= value.len() {
        return value.len();
    }

    value[cursor..]
        .char_indices()
        .nth(1)
        .map(|(index, _)| cursor + index)
        .unwrap_or(value.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::CandidateSource;

    fn state(query: &str) -> PickerState {
        PickerState {
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

    fn candidate(timestamp: u64, cwd: &str, command: &str) -> Candidate {
        Candidate {
            timestamp,
            status: 0,
            cwd: cwd.to_string(),
            command: command.to_string(),
            source: CandidateSource::Local,
            run_count: 1,
            success_count: 1,
        }
    }

    #[test]
    fn handle_picker_key_toggles_failed_history_visibility() {
        let mut state = state("cargo build");
        state.selected = 2;

        let action = handle_picker_key(
            KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL),
            &mut state,
            3,
        );

        assert!(matches!(action, PickerAction::KeepGoing));
        assert!(state.include_failed);
        assert_eq!(state.selected, 0);
        assert_eq!(state.message.as_deref(), Some("showing failed history"));
    }

    #[test]
    fn handle_picker_key_toggles_inspect_mode() {
        let mut state = state("cargo build");

        let action = handle_picker_key(
            KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL),
            &mut state,
            1,
        );

        assert!(matches!(action, PickerAction::KeepGoing));
        assert!(state.inspect);
    }

    #[test]
    fn typing_updates_the_query_instead_of_a_hidden_filter() {
        let mut state = state("cargo");

        let first = handle_picker_key(
            KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE),
            &mut state,
            1,
        );
        let second = handle_picker_key(
            KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
            &mut state,
            1,
        );

        assert!(matches!(first, PickerAction::KeepGoing));
        assert!(matches!(second, PickerAction::KeepGoing));
        assert_eq!(state.query, "cargo b");
        assert_eq!(state.query_cursor, "cargo b".len());
        assert!(state.filter.is_empty());
        assert_eq!(state.scope_anchor, "cargo b");
        assert_eq!(state.scope_words, 2);
    }

    #[test]
    fn left_right_move_query_cursor_and_typing_inserts_at_cursor() {
        let mut state = state("cargo build");

        handle_picker_key(
            KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            &mut state,
            1,
        );
        handle_picker_key(
            KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
            &mut state,
            1,
        );
        handle_picker_key(
            KeyEvent::new(KeyCode::Char('X'), KeyModifiers::SHIFT),
            &mut state,
            1,
        );

        assert_eq!(state.query, "cargo buiXld");
        assert_eq!(state.query_cursor, "cargo buiX".len());
    }

    #[test]
    fn backspace_deletes_before_the_query_cursor() {
        let mut state = state("cargo buiXld");
        state.query_cursor = "cargo buiX".len();

        handle_picker_key(
            KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            &mut state,
            1,
        );

        assert_eq!(state.query, "cargo build");
        assert_eq!(state.query_cursor, "cargo bui".len());
        assert_eq!(state.scope_anchor, "cargo build");
    }

    #[test]
    fn delete_removes_character_at_the_query_cursor() {
        let mut state = state("cargo buiXld");
        state.query_cursor = "cargo bui".len();

        handle_picker_key(
            KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE),
            &mut state,
            1,
        );

        assert_eq!(state.query, "cargo build");
        assert_eq!(state.query_cursor, "cargo bui".len());
        assert_eq!(state.scope_anchor, "cargo build");
    }

    #[test]
    fn ctrl_a_and_ctrl_e_move_query_cursor() {
        let mut state = state("cargo build");
        state.query_cursor = "cargo".len();

        handle_picker_key(
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL),
            &mut state,
            1,
        );
        assert_eq!(state.query_cursor, 0);

        handle_picker_key(
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CONTROL),
            &mut state,
            1,
        );
        assert_eq!(state.query_cursor, "cargo build".len());
    }

    #[test]
    fn tab_returns_cd_only_and_enter_returns_run() {
        let mut state = state("cargo");

        let tab = handle_picker_key(
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            &mut state,
            1,
        );
        let enter = handle_picker_key(
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            &mut state,
            1,
        );

        assert!(matches!(tab, PickerAction::CdOnly));
        assert!(matches!(enter, PickerAction::Run));
    }

    #[test]
    fn up_down_follow_bottom_up_visual_order() {
        let mut state = state("cargo");

        handle_picker_key(
            KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            &mut state,
            4,
        );
        assert_eq!(state.selected, 1);

        handle_picker_key(
            KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            &mut state,
            4,
        );
        assert_eq!(state.selected, 0);

        handle_picker_key(
            KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
            &mut state,
            4,
        );
        assert_eq!(state.selected, 3);
    }

    #[test]
    fn sync_query_to_candidate_updates_visible_input_without_resetting_search_scope() {
        let mut state = state("cargo");
        let candidate = candidate(1, "/project", "cargo install --path . --force");

        sync_query_to_candidate(&mut state, &candidate);

        assert_eq!(state.query, "cargo install --path . --force");
        assert_eq!(state.query_cursor, "cargo install --path . --force".len());
        assert!(state.query_synced_to_selection);
        assert_eq!(state.scope_anchor, "cargo");
        assert_eq!(state.scope_words, 1);
    }

    #[test]
    fn visible_candidates_uses_ascii_case_insensitive_filtering() {
        let candidates = vec![
            candidate(1, "/Users/me/Projects/API", "Cargo Build"),
            candidate(2, "/Users/me/Projects/web", "npm test"),
        ];

        let visible = visible_candidates(&candidates, "cargo b");

        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].1.cwd, "/Users/me/Projects/API");
        assert!(contains_ignore_ascii_case("Cargo Build", "cargo b"));
        assert!(!contains_ignore_ascii_case("npm test", "cargo b"));
    }

    #[test]
    fn help_and_source_keys_update_picker_state() {
        let mut state = state("cargo");

        handle_picker_key(
            KeyEvent::new(KeyCode::Char('/'), KeyModifiers::CONTROL),
            &mut state,
            1,
        );
        assert!(state.show_help);
        handle_picker_key(
            KeyEvent::new(KeyCode::Char('_'), KeyModifiers::CONTROL),
            &mut state,
            1,
        );
        assert!(!state.show_help);

        handle_picker_key(
            KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE),
            &mut state,
            1,
        );
        assert_eq!(state.source_filter, SourceFilter::Local);
        assert_eq!(state.message.as_deref(), Some("source: local"));
        handle_picker_key(
            KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE),
            &mut state,
            1,
        );
        assert_eq!(state.source_filter, SourceFilter::Atuin);

        handle_picker_key(
            KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE),
            &mut state,
            1,
        );
        assert_eq!(state.context_filter, ContextFilter::Directory);
        assert_eq!(state.message.as_deref(), Some("context: directory"));
    }

    #[test]
    fn copy_delete_and_page_keys_have_explicit_actions() {
        let mut state = state("cargo");

        let copy = handle_picker_key(
            KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL),
            &mut state,
            10,
        );
        let delete = handle_picker_key(
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
            &mut state,
            10,
        );

        assert!(matches!(copy, PickerAction::CopySelected));
        assert!(matches!(delete, PickerAction::DeleteSelected));

        handle_picker_key(
            KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE),
            &mut state,
            10,
        );
        assert_eq!(state.selected, 5);
        handle_picker_key(
            KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
            &mut state,
            10,
        );
        assert_eq!(state.selected, 0);
    }
}
