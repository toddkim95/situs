mod args;
mod choose;
mod import;
mod record;

use std::env;
use std::process::Command;

use crate::atuin::AtuinSyncMode;
use crate::config::{
    config_path, read_configured_atuin_sync_mode, resolve_atuin_sync_mode, sync_mode_name,
    write_configured_atuin_sync_mode,
};
use crate::doctor::doctor_report;
use crate::error::{cli_error, CliResult};
use crate::history::{history_path, record_executed_command};
use crate::i18n::I18n;
use crate::model::ExecutionMode;
use crate::setup::setup_command;
use crate::shell::print_zsh_init;
use crate::stats::stats_report;

pub(crate) fn run() -> CliResult<i32> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let Some(command) = args.first().cloned() else {
        print_help();
        return Ok(0);
    };
    args.remove(0);

    match command.as_str() {
        "atuin" => atuin_command(&args),
        "choose" => choose::choose_command(&args),
        "help" | "--help" | "-h" => {
            print_help();
            Ok(0)
        }
        "doctor" => doctor_command(&args),
        "import" => import::import_command(&args),
        "init" => init_command(&args),
        "keymap" => keymap_command(&args),
        "record" => record::record_command(&args),
        "run" => choose::run_command(&args),
        "setup" => setup_command(&args),
        "stats" => stats_command(&args),
        unknown => Err(cli_error(format!("unknown command `{unknown}`"))),
    }
}

fn keymap_command(args: &[String]) -> CliResult<i32> {
    if !args.is_empty() {
        return Err(cli_error("keymap does not accept arguments yet"));
    }

    print!("{}", I18n::from_env().keymap_text());
    Ok(0)
}

fn atuin_command(args: &[String]) -> CliResult<i32> {
    match args {
        [] => atuin_status_command(),
        [command] if command == "status" => atuin_status_command(),
        [command] if command == "enable" => set_atuin_sync_command(AtuinSyncMode::Auto),
        [command, flag] if command == "enable" && flag == "--always" => {
            set_atuin_sync_command(AtuinSyncMode::Always)
        }
        [command] if command == "disable" => set_atuin_sync_command(AtuinSyncMode::Off),
        [command] => Err(cli_error(format!(
            "unknown atuin command `{command}`; try `situs atuin enable`"
        ))),
        _ => Err(cli_error(
            "unknown atuin arguments; try `situs atuin enable`, `disable`, or `status`",
        )),
    }
}

fn set_atuin_sync_command(mode: AtuinSyncMode) -> CliResult<i32> {
    let path = write_configured_atuin_sync_mode(mode)?;
    println!(
        "Atuin auto-sync set to `{}` in {}",
        sync_mode_name(mode),
        path.display()
    );
    Ok(0)
}

fn atuin_status_command() -> CliResult<i32> {
    let resolved = resolve_atuin_sync_mode()?;
    let configured = read_configured_atuin_sync_mode()?
        .map(sync_mode_name)
        .unwrap_or("not set");
    let env_override = env::var("SITUS_ATUIN_SYNC").ok();

    println!("Atuin auto-sync {}", sync_mode_name(resolved));
    println!("config {}", config_path().display());
    println!("configured {}", configured);
    if let Some(value) = env_override {
        println!("env override SITUS_ATUIN_SYNC={value}");
    }
    Ok(0)
}

fn doctor_command(args: &[String]) -> CliResult<i32> {
    if !args.is_empty() {
        return Err(cli_error("doctor does not accept arguments yet"));
    }

    print!("{}", doctor_report()?);
    Ok(0)
}

fn stats_command(args: &[String]) -> CliResult<i32> {
    if !args.is_empty() {
        return Err(cli_error("stats does not accept arguments yet"));
    }

    let records = crate::history::read_records(&history_path()?)?;
    print!("{}", stats_report(&records));
    Ok(0)
}

fn init_command(args: &[String]) -> CliResult<i32> {
    match args.first().map(String::as_str) {
        Some("zsh") => {
            print_zsh_init();
            Ok(0)
        }
        Some("bash") => {
            crate::shell::print_bash_init();
            Ok(0)
        }
        Some("fish") => {
            crate::shell::print_fish_init();
            Ok(0)
        }
        Some(shell) => Err(cli_error(format!(
            "`{shell}` is not supported yet; try `situs init zsh`, `bash`, or `fish`"
        ))),
        None => Err(cli_error(
            "missing shell name; try `situs init zsh`, `bash`, or `fish`",
        )),
    }
}

fn maybe_auto_sync_quietly() {
    if let Err(error) = crate::atuin::maybe_auto_sync_atuin() {
        eprintln!("situs: Atuin auto-sync failed: {error}");
    }
}

fn execute_command_in_dir(command: &str, cwd: &str, _mode: ExecutionMode) -> CliResult<i32> {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let status = Command::new(shell)
        .arg("-lc")
        .arg(command)
        .current_dir(cwd)
        .status()?;

    let code = status.code().unwrap_or(1);
    if let Err(error) = record_executed_command(cwd, code, command) {
        eprintln!("situs: failed to record command result: {error}");
    }

    Ok(code)
}

fn print_help() {
    print!("{}", I18n::from_env().help_text());
}
