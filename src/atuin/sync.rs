use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::config::resolve_atuin_sync_mode;
use crate::error::CliResult;
use crate::history::{decode_storage_field, encode_storage_field, history_path};

use super::{default_atuin_db_path, import_atuin_db, AtuinImportSummary};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AtuinSyncMode {
    Off,
    Auto,
    Always,
}

pub(crate) fn maybe_auto_sync_atuin() -> CliResult<Option<AtuinImportSummary>> {
    let mode = resolve_atuin_sync_mode()?;
    let Some(db_path) = default_atuin_db_path().filter(|path| path.exists()) else {
        return Ok(None);
    };
    let history_path = history_path()?;
    let state_path = sync_state_path(&history_path);

    maybe_auto_sync_atuin_with_paths(mode, &db_path, &history_path, &state_path)
}

pub(crate) fn parse_sync_mode(value: Option<&str>) -> CliResult<AtuinSyncMode> {
    match value.unwrap_or("").trim() {
        "" | "off" => Ok(AtuinSyncMode::Off),
        "auto" => Ok(AtuinSyncMode::Auto),
        "always" => Ok(AtuinSyncMode::Always),
        unknown => Err(crate::error::cli_error(format!(
            "unknown SITUS_ATUIN_SYNC `{unknown}`; expected off, auto, or always"
        ))),
    }
}

fn maybe_auto_sync_atuin_with_paths(
    mode: AtuinSyncMode,
    db_path: &Path,
    history_path: &Path,
    state_path: &Path,
) -> CliResult<Option<AtuinImportSummary>> {
    match mode {
        AtuinSyncMode::Off => Ok(None),
        AtuinSyncMode::Auto => {
            let current = SyncState::from_db(db_path)?;
            if history_path.exists()
                && read_sync_state(state_path)
                    .as_ref()
                    .map(|state| state == &current)
                    .unwrap_or(false)
            {
                return Ok(None);
            }

            let summary = import_atuin_db(db_path, history_path)?;
            write_sync_state(state_path, &current)?;
            Ok(Some(summary))
        }
        AtuinSyncMode::Always => {
            let current = SyncState::from_db(db_path)?;
            let summary = import_atuin_db(db_path, history_path)?;
            write_sync_state(state_path, &current)?;
            Ok(Some(summary))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SyncState {
    db_path: String,
    modified_secs: u64,
    modified_nanos: u32,
}

impl SyncState {
    fn from_db(db_path: &Path) -> CliResult<Self> {
        let modified = fs::metadata(db_path)?.modified().unwrap_or(UNIX_EPOCH);
        let duration = modified
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0));

        Ok(Self {
            db_path: db_path.to_string_lossy().into_owned(),
            modified_secs: duration.as_secs(),
            modified_nanos: duration.subsec_nanos(),
        })
    }
}

fn sync_state_path(history_path: &Path) -> PathBuf {
    history_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("atuin-sync-state")
}

fn read_sync_state(path: &Path) -> Option<SyncState> {
    let contents = fs::read_to_string(path).ok()?;
    let mut parts = contents.trim_end().split('\t');

    if parts.next()? != "v1" {
        return None;
    }

    Some(SyncState {
        db_path: decode_storage_field(parts.next()?)?,
        modified_secs: parts.next()?.parse().ok()?,
        modified_nanos: parts.next()?.parse().ok()?,
    })
}

fn write_sync_state(path: &Path, state: &SyncState) -> CliResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    writeln!(
        file,
        "v1\t{}\t{}\t{}",
        encode_storage_field(&state.db_path),
        state.modified_secs,
        state.modified_nanos
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rusqlite::Connection;

    use super::*;

    #[test]
    fn parses_sync_modes() {
        assert_eq!(parse_sync_mode(None).unwrap(), AtuinSyncMode::Off);
        assert_eq!(parse_sync_mode(Some("")).unwrap(), AtuinSyncMode::Off);
        assert_eq!(parse_sync_mode(Some("off")).unwrap(), AtuinSyncMode::Off);
        assert_eq!(parse_sync_mode(Some("auto")).unwrap(), AtuinSyncMode::Auto);
        assert_eq!(
            parse_sync_mode(Some("always")).unwrap(),
            AtuinSyncMode::Always
        );

        let error = parse_sync_mode(Some("sometimes")).unwrap_err();
        assert!(error.to_string().contains("SITUS_ATUIN_SYNC"));
    }

    #[test]
    fn auto_sync_skips_when_database_metadata_is_unchanged() {
        let dir = temp_test_dir("auto-sync-skip");
        let db_path = dir.join("history.db");
        let history_path = dir.join("history.tsv");
        let state_path = dir.join("atuin-sync-state");
        create_atuin_db(&db_path, "cargo build", "/tmp/app");

        let first = maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Auto,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap()
        .unwrap();
        let second = maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Auto,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap();

        assert_eq!(first.imported, 1);
        assert!(second.is_none());
    }

    #[test]
    fn always_sync_checks_database_even_when_metadata_is_unchanged() {
        let dir = temp_test_dir("always-sync");
        let db_path = dir.join("history.db");
        let history_path = dir.join("history.tsv");
        let state_path = dir.join("atuin-sync-state");
        create_atuin_db(&db_path, "cargo build", "/tmp/app");

        maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Always,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap();
        let second = maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Always,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap()
        .unwrap();

        assert_eq!(second.scanned, 1);
        assert_eq!(second.imported, 0);
        assert_eq!(second.skipped_existing, 1);
    }

    #[test]
    fn off_sync_does_not_touch_history_or_state() {
        let dir = temp_test_dir("off-sync");
        let db_path = dir.join("history.db");
        let history_path = dir.join("history.tsv");
        let state_path = dir.join("atuin-sync-state");
        create_atuin_db(&db_path, "cargo build", "/tmp/app");

        let result = maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Off,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap();

        assert!(result.is_none());
        assert!(!history_path.exists());
        assert!(!state_path.exists());
    }

    #[test]
    fn auto_sync_imports_when_history_is_missing_even_if_state_matches() {
        let dir = temp_test_dir("auto-sync-missing-history");
        let db_path = dir.join("history.db");
        let history_path = dir.join("history.tsv");
        let state_path = dir.join("atuin-sync-state");
        create_atuin_db(&db_path, "cargo build", "/tmp/app");

        maybe_auto_sync_atuin_with_paths(AtuinSyncMode::Auto, &db_path, &history_path, &state_path)
            .unwrap();
        fs::remove_file(&history_path).unwrap();
        let second = maybe_auto_sync_atuin_with_paths(
            AtuinSyncMode::Auto,
            &db_path,
            &history_path,
            &state_path,
        )
        .unwrap()
        .unwrap();

        assert_eq!(second.imported, 1);
        assert!(history_path.exists());
    }

    fn temp_test_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "situs-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn create_atuin_db(path: &Path, command: &str, cwd: &str) {
        let connection = Connection::open(path).unwrap();
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
                 VALUES ('1', 1700000000000000000, 0, 0, ?1, ?2, NULL)",
                [command, cwd],
            )
            .unwrap();
    }
}
