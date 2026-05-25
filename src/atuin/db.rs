use std::path::Path;

use crate::error::CliResult;
use crate::model::{HistorySource, Record};

pub(super) fn read_atuin_records(db_path: &Path) -> CliResult<Vec<Record>> {
    let connection =
        rusqlite::Connection::open_with_flags(db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    connection.busy_timeout(std::time::Duration::from_millis(500))?;
    let columns = table_columns(&connection, "history")?;

    let command_column = require_column(&columns, &["command"])?;
    let cwd_column = require_column(&columns, &["cwd"])?;
    let status_column = require_column(&columns, &["exit", "exit_code", "status"])?;
    let timestamp_column = require_column(&columns, &["timestamp", "created_at"])?;
    let deleted_column = find_column(&columns, &["deleted_at"]);

    let mut query = format!(
        "SELECT {}, {}, {}, {} FROM history",
        quote_identifier(&timestamp_column),
        quote_identifier(&status_column),
        quote_identifier(&cwd_column),
        quote_identifier(&command_column)
    );
    if let Some(column) = deleted_column {
        query.push_str(&format!(" WHERE {} IS NULL", quote_identifier(&column)));
    }

    let mut statement = connection.prepare(&query)?;
    let mut rows = statement.query([])?;
    let mut records = Vec::new();

    while let Some(row) = rows.next()? {
        let timestamp: i64 = row.get(0)?;
        let status: i32 = row.get(1)?;
        let cwd: String = row.get(2)?;
        let command: String = row.get(3)?;

        if cwd.trim().is_empty() || command.trim().is_empty() {
            continue;
        }

        records.push(Record {
            timestamp: normalize_atuin_timestamp(timestamp),
            status,
            cwd,
            command,
            source: HistorySource::Atuin,
        });
    }

    Ok(records)
}

fn table_columns(connection: &rusqlite::Connection, table: &str) -> CliResult<Vec<String>> {
    let mut statement =
        connection.prepare(&format!("PRAGMA table_info({})", quote_identifier(table)))?;
    let rows = statement.query_map([], |row| row.get::<_, String>(1))?;
    let mut columns = Vec::new();
    for row in rows {
        columns.push(row?);
    }
    Ok(columns)
}

fn require_column(columns: &[String], names: &[&str]) -> CliResult<String> {
    find_column(columns, names).ok_or_else(|| {
        crate::error::cli_error(format!(
            "Atuin history table is missing required column: one of {}",
            names.join(", ")
        ))
    })
}

fn find_column(columns: &[String], names: &[&str]) -> Option<String> {
    columns
        .iter()
        .find(|column| names.iter().any(|name| column.eq_ignore_ascii_case(name)))
        .cloned()
}

fn quote_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn normalize_atuin_timestamp(timestamp: i64) -> u64 {
    let timestamp = timestamp.max(0) as u64;

    if timestamp >= 1_000_000_000_000_000_000 {
        timestamp / 1_000_000_000
    } else if timestamp >= 1_000_000_000_000_000 {
        timestamp / 1_000_000
    } else if timestamp >= 1_000_000_000_000 {
        timestamp / 1_000
    } else {
        timestamp
    }
}
