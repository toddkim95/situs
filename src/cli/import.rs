use std::path::PathBuf;

use crate::atuin::{default_atuin_db_path, import_atuin_db};
use crate::error::{cli_error, CliResult};
use crate::history::history_path;

pub(super) fn import_command(args: &[String]) -> CliResult<i32> {
    match args.first().map(String::as_str) {
        Some("atuin") => import_atuin_command(&args[1..]),
        Some(source) => Err(cli_error(format!(
            "`{source}` import is not supported yet; try `situs import atuin`"
        ))),
        None => Err(cli_error("missing import source; try `situs import atuin`")),
    }
}

fn import_atuin_command(args: &[String]) -> CliResult<i32> {
    let mut db_path = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--db" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err(cli_error("missing value for --db"));
                };
                db_path = Some(PathBuf::from(value));
            }
            value => {
                return Err(cli_error(format!(
                    "unknown import atuin option `{value}`; expected --db <path>"
                )));
            }
        }

        index += 1;
    }

    let db_path = match db_path.or_else(default_atuin_db_path) {
        Some(path) => path,
        None => {
            return Err(cli_error(
                "could not find Atuin history database; pass --db <path>",
            ));
        }
    };
    let summary = import_atuin_db(&db_path, &history_path()?)?;

    println!(
        "Imported {} Atuin history records ({} scanned, {} already existed)",
        summary.imported, summary.scanned, summary.skipped_existing
    );
    Ok(0)
}
