use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{
    picker_mode_name, resolve_atuin_sync_mode, resolve_picker_mode, sync_mode_name,
};
use crate::error::CliResult;
use crate::history::{history_path, read_records};
use crate::i18n::{I18n, MessageKey};

pub(crate) struct DoctorReport {
    pub(crate) history_path: PathBuf,
    pub(crate) history_records: usize,
    pub(crate) key_binding: String,
    pub(crate) mode: String,
    pub(crate) picker_mode: String,
    pub(crate) atuin_sync: String,
    pub(crate) zshrc_has_init: bool,
    pub(crate) atuin_db: Option<PathBuf>,
}

#[cfg(test)]
pub(crate) fn format_doctor_report(report: &DoctorReport) -> String {
    format_doctor_report_with_i18n(report, I18n::english())
}

fn format_doctor_report_with_i18n(report: &DoctorReport, i18n: I18n) -> String {
    let atuin = report
        .atuin_db
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| i18n.text(MessageKey::DoctorNotFound).to_string());
    let zshrc = if report.zshrc_has_init {
        i18n.text(MessageKey::DoctorConfigured)
    } else {
        i18n.text(MessageKey::DoctorNotFound)
    };

    let mut output = format!("{}\n", i18n.text(MessageKey::DoctorTitle));
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorHistoryPath),
        &report.history_path.display().to_string(),
    );
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorHistoryRecords),
        &report.history_records.to_string(),
    );
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorKeyBinding),
        &report.key_binding,
    );
    push_label(&mut output, i18n.text(MessageKey::DoctorMode), &report.mode);
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorPickerMode),
        &report.picker_mode,
    );
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorAtuinSync),
        &report.atuin_sync,
    );
    push_label(
        &mut output,
        i18n.text(MessageKey::DoctorZshIntegration),
        zshrc,
    );
    push_label(&mut output, i18n.text(MessageKey::DoctorAtuinDb), &atuin);
    output
}

fn push_label(output: &mut String, label: &str, value: &str) {
    let width = crate::picker::width::visible_width(label);
    let padding = 16_usize.saturating_sub(width).max(1);
    output.push_str(label);
    for _ in 0..padding {
        output.push(' ');
    }
    output.push_str(value);
    output.push('\n');
}

pub(crate) fn doctor_report() -> CliResult<String> {
    let history_path = history_path()?;
    let history_records = read_records(&history_path)?.len();
    let key_binding = env::var("SITUS_BINDKEY").unwrap_or_else(|_| "^G".to_string());
    let mode = env::var("SITUS_MODE").unwrap_or_else(|_| "stay".to_string());
    let picker_mode = picker_mode_name(resolve_picker_mode()?).to_string();
    let atuin_sync = sync_mode_name(resolve_atuin_sync_mode()?).to_string();
    let zshrc_has_init = zshrc_path()
        .as_deref()
        .map(file_contains_situs_init)
        .unwrap_or(false);
    let atuin_db = default_atuin_db_path().filter(|path| path.exists());

    Ok(format_doctor_report_with_i18n(
        &DoctorReport {
            history_path,
            history_records,
            key_binding,
            mode,
            picker_mode,
            atuin_sync,
            zshrc_has_init,
            atuin_db,
        },
        I18n::from_env(),
    ))
}

pub(crate) fn default_atuin_db_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("ATUIN_DB") {
        return Some(PathBuf::from(path));
    }

    if let Ok(path) = env::var("XDG_DATA_HOME") {
        return Some(PathBuf::from(path).join("atuin").join("history.db"));
    }

    env::var("HOME").ok().map(|home| {
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("atuin")
            .join("history.db")
    })
}

fn zshrc_path() -> Option<PathBuf> {
    env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".zshrc"))
}

fn file_contains_situs_init(path: &Path) -> bool {
    fs::read_to_string(path)
        .map(|contents| contents.contains("situs init zsh"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_report_formats_core_status() {
        let report = DoctorReport {
            history_path: PathBuf::from("/tmp/situs/history.tsv"),
            history_records: 12,
            key_binding: "^G".to_string(),
            mode: "stay".to_string(),
            picker_mode: "fullscreen".to_string(),
            atuin_sync: "auto".to_string(),
            zshrc_has_init: true,
            atuin_db: Some(PathBuf::from("/tmp/atuin/history.db")),
        };

        let rendered = format_doctor_report(&report);

        assert!(rendered.contains("Situs Doctor"));
        assert!(rendered.contains("history records 12"));
        assert!(rendered.contains("key binding     ^G"));
        assert!(rendered.contains("picker mode     fullscreen"));
        assert!(rendered.contains("atuin sync      auto"));
        assert!(rendered.contains("zsh integration configured"));
        assert!(rendered.contains("/tmp/atuin/history.db"));
    }
}
