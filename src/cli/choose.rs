use crate::command::normalize_command;
use crate::config::{parse_picker_mode, resolve_picker_mode};
use crate::error::{cli_error, CliResult};
use crate::model::{ContextFilter, ExecutionMode, PickerMode};
use crate::picker::{choose_candidate, choose_candidate_tui_only, SelectionAction};

use super::args::ArgCursor;
use super::{execute_command_in_dir, maybe_auto_sync_quietly};

#[derive(Debug)]
struct ChooseArgs {
    command: String,
    include_failed: bool,
    print_dir: bool,
    print_selection: bool,
    print_widget_selection: bool,
    mode: ExecutionMode,
    picker_mode: Option<PickerMode>,
    context_filter: ContextFilter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChoosePickerMode {
    WithPlainFallback,
    TuiOnly,
}

pub(super) fn choose_command(args: &[String]) -> CliResult<i32> {
    let args = parse_choose_args(args)?;
    let picker_mode = args.picker_mode.unwrap_or(resolve_picker_mode()?);
    maybe_auto_sync_quietly();
    let selection = match choose_picker_mode(&args) {
        ChoosePickerMode::WithPlainFallback => choose_candidate(
            &args.command,
            args.include_failed,
            picker_mode,
            args.context_filter,
        ),
        ChoosePickerMode::TuiOnly => choose_candidate_tui_only(
            &args.command,
            args.include_failed,
            picker_mode,
            args.context_filter,
        ),
    }?;
    let Some(selection) = selection else {
        return Ok(1);
    };
    let candidate = &selection.candidate;

    if args.print_dir {
        println!("{}", candidate.cwd);
        return Ok(0);
    }

    if args.print_selection {
        println!("{}", candidate.cwd);
        println!("{}", legacy_print_selection_command(&selection));
        return Ok(0);
    }

    if args.print_widget_selection {
        println!(
            "{}",
            match selection.action {
                SelectionAction::Run => "run",
                SelectionAction::CdOnly => "cd",
            }
        );
        println!("{}", candidate.cwd);
        println!("{}", candidate.command);
        println!("{}", selection.query);
        return Ok(0);
    }

    match selection.action {
        SelectionAction::Run => {
            execute_command_in_dir(&candidate.command, &candidate.cwd, args.mode)
        }
        SelectionAction::CdOnly => {
            println!("{}", candidate.cwd);
            Ok(0)
        }
    }
}

pub(super) fn run_command(args: &[String]) -> CliResult<i32> {
    let command = parse_command_text(args)?;
    let picker_mode = resolve_picker_mode()?;
    maybe_auto_sync_quietly();
    let Some(selection) = choose_candidate(&command, false, picker_mode, ContextFilter::All)?
    else {
        return Ok(1);
    };
    if matches!(selection.action, SelectionAction::CdOnly) {
        println!("{}", selection.candidate.cwd);
        return Ok(0);
    }

    execute_command_in_dir(
        &selection.candidate.command,
        &selection.candidate.cwd,
        ExecutionMode::Stay,
    )
}

fn choose_picker_mode(args: &ChooseArgs) -> ChoosePickerMode {
    if args.print_widget_selection {
        ChoosePickerMode::TuiOnly
    } else {
        ChoosePickerMode::WithPlainFallback
    }
}

fn legacy_print_selection_command(selection: &crate::picker::PickerSelection) -> &str {
    if matches!(selection.action, SelectionAction::CdOnly) {
        // Older zsh integrations used --print-selection and then always
        // accepted `cd -- dir && command`. They cannot represent Tab's cd-only
        // action, so send a no-op instead of accidentally running the command.
        ":"
    } else {
        &selection.candidate.command
    }
}

fn parse_choose_args(args: &[String]) -> CliResult<ChooseArgs> {
    let mut command = None;
    let mut include_failed = false;
    let mut print_dir = false;
    let mut print_selection = false;
    let mut print_widget_selection = false;
    let mut mode = ExecutionMode::Stay;
    let mut picker_mode = None;
    let mut context_filter = ContextFilter::All;
    let mut rest = Vec::new();
    let mut cursor = ArgCursor::new(args);

    while let Some(arg) = cursor.next() {
        match arg {
            "--command" | "-c" => {
                command = Some(cursor.next_value("--command")?.to_string());
            }
            "--include-failed" => include_failed = true,
            "--mode" => {
                let value = cursor.next_value("--mode")?;
                mode = parse_execution_mode(value)?;
            }
            "--picker" => {
                let value = cursor.next_value("--picker")?;
                picker_mode = Some(parse_picker_mode(value)?);
            }
            "--context" => {
                let value = cursor.next_value("--context")?;
                context_filter = parse_context_filter(value)?;
            }
            "--stay" => mode = ExecutionMode::Stay,
            "--restore" => mode = ExecutionMode::Restore,
            "--print-dir" => print_dir = true,
            "--print-selection" => print_selection = true,
            "--print-widget-selection" => print_widget_selection = true,
            "--" => {
                let remaining = cursor.remaining_joined();
                if !remaining.is_empty() {
                    rest.push(remaining);
                }
                break;
            }
            value => rest.push(value.to_string()),
        }
    }

    let command = command.unwrap_or_else(|| rest.join(" "));
    let command = normalize_command(&command);

    Ok(ChooseArgs {
        command,
        include_failed,
        print_dir,
        print_selection,
        print_widget_selection,
        mode,
        picker_mode,
        context_filter,
    })
}

fn parse_command_text(args: &[String]) -> CliResult<String> {
    let mut command = None;
    let mut rest = Vec::new();
    let mut cursor = ArgCursor::new(args);

    while let Some(arg) = cursor.next() {
        match arg {
            "--command" | "-c" => {
                command = Some(cursor.next_value("--command")?.to_string());
            }
            "--" => {
                let remaining = cursor.remaining_joined();
                if !remaining.is_empty() {
                    rest.push(remaining);
                }
                break;
            }
            value => rest.push(value.to_string()),
        }
    }

    let command = command.unwrap_or_else(|| rest.join(" "));
    let command = normalize_command(&command);

    if command.is_empty() {
        return Err(cli_error("missing command"));
    }

    Ok(command)
}

fn parse_execution_mode(value: &str) -> CliResult<ExecutionMode> {
    match value {
        "stay" => Ok(ExecutionMode::Stay),
        "restore" => Ok(ExecutionMode::Restore),
        unknown => Err(cli_error(format!(
            "unknown execution mode `{unknown}`; expected `stay` or `restore`"
        ))),
    }
}

fn parse_context_filter(value: &str) -> CliResult<ContextFilter> {
    match value {
        "all" | "global" => Ok(ContextFilter::All),
        "directory" | "dir" | "cwd" => Ok(ContextFilter::Directory),
        "workspace" | "repo" | "git" => Ok(ContextFilter::Workspace),
        unknown => Err(cli_error(format!(
            "unknown context filter `{unknown}`; expected `all`, `directory`, or `workspace`"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_choose_args_accepts_restore_mode() {
        let args = parse_choose_args(&[
            "--mode".to_string(),
            "restore".to_string(),
            "--command".to_string(),
            "cargo build".to_string(),
        ])
        .unwrap();

        assert_eq!(args.mode, ExecutionMode::Restore);
    }

    #[test]
    fn parse_choose_args_rejects_unknown_mode() {
        let error = parse_choose_args(&[
            "--mode".to_string(),
            "teleport".to_string(),
            "--command".to_string(),
            "cargo build".to_string(),
        ])
        .unwrap_err();

        assert!(error.to_string().contains("unknown execution mode"));
    }

    #[test]
    fn widget_selection_requires_tui_picker() {
        let args = parse_choose_args(&[
            "--print-widget-selection".to_string(),
            "--command".to_string(),
            "cargo build".to_string(),
        ])
        .unwrap();

        assert_eq!(choose_picker_mode(&args), ChoosePickerMode::TuiOnly);
    }

    #[test]
    fn normal_choose_keeps_plain_fallback() {
        let args =
            parse_choose_args(&["--command".to_string(), "cargo build".to_string()]).unwrap();

        assert_eq!(
            choose_picker_mode(&args),
            ChoosePickerMode::WithPlainFallback
        );
    }

    #[test]
    fn parse_choose_args_accepts_picker_mode() {
        let args = parse_choose_args(&[
            "--picker".to_string(),
            "fullscreen".to_string(),
            "--command".to_string(),
            "cargo build".to_string(),
        ])
        .unwrap();

        assert_eq!(args.picker_mode, Some(PickerMode::Fullscreen));
    }

    #[test]
    fn parse_choose_args_accepts_context_filter() {
        let args = parse_choose_args(&[
            "--context".to_string(),
            "workspace".to_string(),
            "--command".to_string(),
            "cargo build".to_string(),
        ])
        .unwrap();

        assert_eq!(args.context_filter, ContextFilter::Workspace);
    }

    #[test]
    fn parse_choose_args_preserves_command_after_double_dash() {
        let args = parse_choose_args(&[
            "--".to_string(),
            "cargo".to_string(),
            "test".to_string(),
            "--".to_string(),
            "--nocapture".to_string(),
        ])
        .unwrap();

        assert_eq!(args.command, "cargo test -- --nocapture");
    }

    #[test]
    fn parse_command_text_preserves_command_after_double_dash() {
        let command = parse_command_text(&[
            "--".to_string(),
            "cargo".to_string(),
            "test".to_string(),
            "--".to_string(),
            "--nocapture".to_string(),
        ])
        .unwrap();

        assert_eq!(command, "cargo test -- --nocapture");
    }

    #[test]
    fn legacy_print_selection_uses_noop_for_cd_only_selection() {
        use crate::model::{Candidate, CandidateSource};
        use crate::picker::PickerSelection;

        let selection = PickerSelection {
            action: SelectionAction::CdOnly,
            candidate: Candidate {
                cwd: "/project".to_string(),
                command: "cargo install --path . --force".to_string(),
                timestamp: 1,
                status: 0,
                source: CandidateSource::Local,
                run_count: 1,
                success_count: 1,
            },
            query: "cargo install --path . --force".to_string(),
        };

        assert_eq!(legacy_print_selection_command(&selection), ":");
    }
}
