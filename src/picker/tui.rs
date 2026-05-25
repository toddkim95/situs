use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::KeyCode;

use crate::command::command_word_count;
use crate::context::CommandContext;
use crate::error::{cli_error, CliResult};
use crate::history::{history_path, load_records, write_records};
use crate::i18n::{I18n, MessageKey};
use crate::matcher::{best_match_scope_with_filters, scoped_candidates_with_filters};
use crate::model::{ContextFilter, MatchScope, Record, SourceFilter};

use super::clipboard::copy_text_to_clipboard;
use super::input::{
    clamp_selection, handle_picker_key, is_cancel_key, sync_query_to_candidate, visible_candidates,
    PickerAction, PickerState,
};
use super::render::{empty_history_lines, loading_lines, picker_lines, query_cursor_position};
use super::session::PickerSession;
use super::{
    delete_candidate_records, match_filters, selected_candidate, selection_for_visible,
    PickerSelection, SelectionAction,
};

pub(super) fn choose_candidate_inline(
    session: &mut PickerSession,
    command: &str,
    include_failed: bool,
    context_filter: ContextFilter,
    command_context: CommandContext,
) -> CliResult<Option<PickerSelection>> {
    let loaded = load_records_with_spinner(session, command)?;
    let Some(mut records) = loaded else {
        return Ok(None);
    };

    prompt_for_candidate_inline(
        session,
        command,
        &mut records,
        include_failed,
        context_filter,
        command_context,
    )
}

pub(super) fn prompt_for_candidate_inline(
    session: &mut PickerSession,
    command: &str,
    records: &mut Vec<Record>,
    include_failed: bool,
    context_filter: ContextFilter,
    command_context: CommandContext,
) -> CliResult<Option<PickerSelection>> {
    let i18n = I18n::from_env();
    let initial_filters = match_filters(SourceFilter::All, context_filter, &command_context);
    let mut initial_scope =
        best_match_scope_with_filters(records, command, include_failed, initial_filters);
    let initial_candidates = scoped_candidates_with_filters(
        records,
        command,
        include_failed,
        initial_scope.words,
        initial_filters,
    );
    let failed_scope = best_match_scope_with_filters(records, command, true, initial_filters);
    let failed_candidates =
        scoped_candidates_with_filters(records, command, true, failed_scope.words, initial_filters);

    if initial_candidates.is_empty() && failed_candidates.is_empty() {
        wait_for_empty_history(session, command)?;
        return Ok(None);
    }

    if initial_candidates.is_empty() {
        initial_scope = failed_scope;
    }

    let mut state = PickerState {
        selected: 0,
        query: command.to_string(),
        query_cursor: command.len(),
        query_synced_to_selection: false,
        filter: String::new(),
        scope_anchor: command.to_string(),
        scope_words: initial_scope.words,
        include_failed,
        inspect: false,
        show_help: false,
        source_filter: SourceFilter::All,
        context_filter,
        message: None,
    };

    loop {
        let filters = match_filters(state.source_filter, state.context_filter, &command_context);
        let mut candidates = scoped_candidates_with_filters(
            records,
            &state.scope_anchor,
            state.include_failed,
            state.scope_words,
            filters,
        );
        if candidates.is_empty() && state.scope_words > 1 {
            let fallback =
                best_match_scope_with_filters(records, &state.query, state.include_failed, filters);
            state.scope_words = fallback.words;
            state.scope_anchor = state.query.clone();
            candidates = scoped_candidates_with_filters(
                records,
                &state.scope_anchor,
                state.include_failed,
                state.scope_words,
                filters,
            );
        }
        let visible = visible_candidates(&candidates, &state.filter);
        clamp_selection(&mut state, visible.len());
        let scope = MatchScope {
            words: state.scope_words,
            total_words: command_word_count(&state.scope_anchor).max(1),
        };

        session.render(
            &picker_lines(
                command,
                &candidates,
                &visible,
                scope,
                &state,
                session.max_rows,
                session.width,
            ),
            Some(query_cursor_position(
                &state.query,
                state.query_cursor,
                session.max_rows,
                session.width,
            )),
        )?;

        let key = session.read_key()?;
        match handle_picker_key(key, &mut state, visible.len()) {
            PickerAction::KeepGoing => {
                if matches!(key.code, KeyCode::Up | KeyCode::Down) {
                    if let Some((_, candidate)) = visible.get(state.selected) {
                        sync_query_to_candidate(&mut state, candidate);
                    }
                }
            }
            PickerAction::Cancel => return Ok(None),
            PickerAction::Run => {
                return Ok(selection_for_visible(
                    &candidates,
                    &visible,
                    &state,
                    SelectionAction::Run,
                ));
            }
            PickerAction::CdOnly => {
                return Ok(selection_for_visible(
                    &candidates,
                    &visible,
                    &state,
                    SelectionAction::CdOnly,
                ));
            }
            PickerAction::CopySelected => {
                if let Some(candidate) = selected_candidate(&candidates, &visible, &state) {
                    match copy_text_to_clipboard(&candidate.command) {
                        Ok(()) => {
                            state.message =
                                Some(i18n.text(MessageKey::PickerMessageCopied).to_string());
                        }
                        Err(error) => {
                            state.message = Some(i18n.picker_copy_failed(&error.to_string()));
                        }
                    }
                } else {
                    state.message = Some(
                        i18n.text(MessageKey::PickerMessageNothingSelected)
                            .to_string(),
                    );
                }
            }
            PickerAction::DeleteSelected => {
                if let Some(candidate) = selected_candidate(&candidates, &visible, &state) {
                    let deleted = delete_candidate_records(records, &candidate);
                    if deleted > 0 {
                        write_records(&history_path()?, records)?;
                        state.message = Some(i18n.picker_deleted_rows(deleted));
                        clamp_selection(&mut state, visible.len().saturating_sub(deleted));
                    } else {
                        state.message = Some(
                            i18n.text(MessageKey::PickerMessageNothingDeleted)
                                .to_string(),
                        );
                    }
                } else {
                    state.message = Some(
                        i18n.text(MessageKey::PickerMessageNothingSelected)
                            .to_string(),
                    );
                }
            }
        }
    }
}

pub(super) fn load_records_with_spinner(
    session: &mut PickerSession,
    command: &str,
) -> CliResult<Option<Vec<Record>>> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let result = load_records().map_err(|error| error.to_string());
        let _ = sender.send(result);
    });

    let started = Instant::now();
    let frames = ["|", "/", "-", "\\"];
    let mut frame_index = 0;

    loop {
        match receiver.try_recv() {
            Ok(Ok(result)) => return Ok(Some(result)),
            Ok(Err(message)) => return Err(cli_error(message)),
            Err(TryRecvError::Disconnected) => {
                return Err(cli_error("history loader stopped before finishing"));
            }
            Err(TryRecvError::Empty) => {}
        }

        session.render(
            &loading_lines(
                command,
                frames[frame_index % frames.len()],
                started.elapsed(),
                session.max_rows,
                session.width,
            ),
            Some(query_cursor_position(
                command,
                command.len(),
                session.max_rows,
                session.width,
            )),
        )?;
        frame_index += 1;

        if let Some(key) = session.read_key_if_ready(Duration::from_millis(80))? {
            if is_cancel_key(key) {
                return Ok(None);
            }
        }
    }
}

pub(super) fn wait_for_empty_history(session: &mut PickerSession, command: &str) -> CliResult<()> {
    session.render(
        &empty_history_lines(command, session.max_rows, session.width),
        Some(query_cursor_position(
            command,
            command.len(),
            session.max_rows,
            session.width,
        )),
    )?;

    let _ = session.read_key()?;
    Ok(())
}
