use std::collections::HashSet;
use std::path::Path;

use crate::error::CliResult;
use crate::history::{append_records, read_records};
use crate::model::Record;

use super::db::read_atuin_records;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AtuinImportSummary {
    pub(crate) scanned: usize,
    pub(crate) imported: usize,
    pub(crate) skipped_existing: usize,
}

pub(crate) fn import_atuin_db(
    db_path: &Path,
    history_path: &Path,
) -> CliResult<AtuinImportSummary> {
    let records = read_atuin_records(db_path)?;
    let existing = read_records(history_path)?;
    let mut seen = existing
        .iter()
        .map(record_key)
        .collect::<HashSet<(u64, i32, String, String)>>();

    let mut summary = AtuinImportSummary {
        scanned: records.len(),
        imported: 0,
        skipped_existing: 0,
    };

    let mut imported_records = Vec::new();
    for record in records {
        if !seen.insert(record_key(&record)) {
            summary.skipped_existing += 1;
            continue;
        }

        imported_records.push(record);
        summary.imported += 1;
    }
    if !imported_records.is_empty() {
        append_records(history_path, &imported_records)?;
    }

    Ok(summary)
}

fn record_key(record: &Record) -> (u64, i32, String, String) {
    (
        record.timestamp,
        record.status,
        record.cwd.clone(),
        record.command.clone(),
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusqlite::Connection;

    use super::*;
    use crate::history::read_records;
    use crate::model::HistorySource;

    #[test]
    fn imports_atuin_history_rows_into_situs_history() {
        let dir = std::env::temp_dir().join(format!(
            "situs-atuin-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let db_path = dir.join("history.db");
        let history_path = dir.join("history.tsv");

        let connection = Connection::open(&db_path).unwrap();
        connection
            .execute(
                "CREATE TABLE history (
                    id TEXT PRIMARY KEY,
                    timestamp INTEGER NOT NULL,
                    duration INTEGER NOT NULL,
                    exit INTEGER NOT NULL,
                    command TEXT NOT NULL,
                    cwd TEXT NOT NULL,
                    deleted_at INTEGER
                )",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO history (id, timestamp, duration, exit, command, cwd, deleted_at)
                 VALUES ('1', 1700000000000000000, 0, 0, 'cargo build', '/tmp/app', NULL)",
                [],
            )
            .unwrap();

        let summary = import_atuin_db(&db_path, &history_path).unwrap();
        let records = read_records(&history_path).unwrap();

        assert_eq!(
            summary,
            AtuinImportSummary {
                scanned: 1,
                imported: 1,
                skipped_existing: 0,
            }
        );
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].timestamp, 1_700_000_000);
        assert_eq!(records[0].status, 0);
        assert_eq!(records[0].command, "cargo build");
        assert_eq!(records[0].cwd, "/tmp/app");
        assert_eq!(records[0].source, HistorySource::Atuin);
    }
}
