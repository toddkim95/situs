mod db;
mod import;
mod sync;

use std::path::PathBuf;

pub(crate) use import::{import_atuin_db, AtuinImportSummary};
pub(crate) use sync::{maybe_auto_sync_atuin, parse_sync_mode, AtuinSyncMode};

pub(crate) fn default_atuin_db_path() -> Option<PathBuf> {
    crate::doctor::default_atuin_db_path()
}
