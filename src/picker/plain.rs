use crate::command::command_prefix;
use crate::context::CommandContext;
use crate::error::CliResult;
use crate::history::{human_age, load_records};
use crate::matcher::matching_candidates_with_filters;
use crate::model::{Candidate, ContextFilter, MatchScope, SourceFilter};
use crate::terminal::{write_to_terminal, Terminal};

use super::{match_filters, PickerSelection, SelectionAction};

pub(super) fn choose_candidate_plain(
    command: &str,
    include_failed: bool,
    context_filter: ContextFilter,
    command_context: &CommandContext,
) -> CliResult<Option<PickerSelection>> {
    let records = load_records()?;
    let filters = match_filters(SourceFilter::All, context_filter, command_context);
    let (candidates, scope) =
        matching_candidates_with_filters(&records, command, include_failed, filters);
    if candidates.is_empty() {
        if !include_failed {
            let (failed_candidates, _) =
                matching_candidates_with_filters(&records, command, true, filters);
            if !failed_candidates.is_empty() {
                write_to_terminal(format_args!(
                    "situs: only failed history found for `{command}`; retry with --include-failed\n"
                ))?;
                return Ok(None);
            }
        }
        write_to_terminal(format_args!(
            "situs: no directory history for `{command}`\n"
        ))?;
        return Ok(None);
    }

    prompt_for_candidate_plain(command, &candidates, scope)
}

fn prompt_for_candidate_plain(
    command: &str,
    candidates: &[Candidate],
    scope: MatchScope,
) -> CliResult<Option<PickerSelection>> {
    let mut tty = Terminal::open();
    tty.write(format_args!("\nsitus: directories for `{command}`\n"))?;
    if scope.words < scope.total_words {
        tty.write(format_args!(
            "situs: no exact match, showing matches for `{}`\n",
            command_prefix(command, scope.words)
        ))?;
    }

    for (index, candidate) in candidates.iter().enumerate() {
        let status_note = if candidate.status == 0 {
            String::new()
        } else {
            format!(", exit {}", candidate.status)
        };
        tty.write(format_args!(
            "{:>2}. {}  [{}]  ({}{})\n",
            index + 1,
            candidate.command,
            candidate.cwd,
            human_age(candidate.timestamp),
            status_note
        ))?;
    }

    loop {
        tty.write(format_args!("Choose [1], or q to cancel: "))?;
        let input = tty.read_line()?;
        let input = input.trim();

        if input.is_empty() {
            return Ok(candidates
                .first()
                .cloned()
                .map(|candidate| PickerSelection {
                    action: SelectionAction::Run,
                    candidate,
                    query: command.to_string(),
                }));
        }

        if input.eq_ignore_ascii_case("q") {
            return Ok(None);
        }

        match input.parse::<usize>() {
            Ok(choice) if (1..=candidates.len()).contains(&choice) => {
                return Ok(candidates
                    .get(choice - 1)
                    .cloned()
                    .map(|candidate| PickerSelection {
                        action: SelectionAction::Run,
                        candidate,
                        query: command.to_string(),
                    }));
            }
            _ => tty.write(format_args!(
                "situs: enter a number from 1 to {}, or q\n",
                candidates.len()
            ))?,
        }
    }
}
