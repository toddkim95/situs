use std::env;

use crate::command::{normalize_command, should_ignore_command};
use crate::error::{cli_error, CliResult};
use crate::history::{append_record, history_path, now_seconds};
use crate::model::{HistorySource, Record};

#[derive(Debug)]
struct RecordArgs {
    cwd: String,
    status: i32,
    command: String,
}

pub(super) fn record_command(args: &[String]) -> CliResult<i32> {
    let args = parse_record_args(args)?;
    let normalized = normalize_command(&args.command);

    if normalized.is_empty() || should_ignore_command(&normalized) {
        return Ok(0);
    }

    append_record(
        &history_path()?,
        &Record {
            timestamp: now_seconds(),
            status: args.status,
            cwd: args.cwd,
            command: normalized,
            source: HistorySource::Local,
        },
    )?;

    Ok(0)
}

fn parse_record_args(args: &[String]) -> CliResult<RecordArgs> {
    let mut cwd = None;
    let mut status = 0;
    let mut command_parts = Vec::new();
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--cwd" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err(cli_error("missing value for --cwd"));
                };
                cwd = Some(value.clone());
            }
            "--status" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err(cli_error("missing value for --status"));
                };
                status = value.parse::<i32>()?;
            }
            "--" => {
                command_parts.extend(args[index + 1..].iter().cloned());
                break;
            }
            value => command_parts.push(value.to_string()),
        }

        index += 1;
    }

    let cwd = match cwd {
        Some(value) => value,
        None => env::current_dir()?.to_string_lossy().into_owned(),
    };
    let command = normalize_command(&command_parts.join(" "));

    if command.is_empty() {
        return Err(cli_error("missing command to record"));
    }

    Ok(RecordArgs {
        cwd,
        status,
        command,
    })
}
