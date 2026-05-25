use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::atuin::{parse_sync_mode, AtuinSyncMode};
use crate::error::CliResult;
use crate::model::PickerMode;

const ATUIN_SYNC_KEY: &str = "atuin_sync";
const PICKER_MODE_KEY: &str = "picker_mode";
const LANGUAGE_KEY: &str = "language";

pub(crate) fn config_path() -> PathBuf {
    if let Ok(path) = env::var("SITUS_CONFIG") {
        return PathBuf::from(path);
    }

    #[cfg(test)]
    {
        return PathBuf::from("/nonexistent-situs-test-config-path-xyz");
    }

    if let Ok(path) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(path).join("situs-cli").join("config");
    }

    env::var("HOME")
        .map(|home| {
            PathBuf::from(home)
                .join(".config")
                .join("situs-cli")
                .join("config")
        })
        .unwrap_or_else(|_| PathBuf::from("situs-config"))
}

pub(crate) fn read_configured_atuin_sync_mode() -> CliResult<Option<AtuinSyncMode>> {
    read_atuin_sync_config(&config_path())
}

pub(crate) fn write_configured_atuin_sync_mode(mode: AtuinSyncMode) -> CliResult<PathBuf> {
    let path = config_path();
    write_atuin_sync_config(&path, mode)?;
    Ok(path)
}

pub(crate) fn resolve_atuin_sync_mode() -> CliResult<AtuinSyncMode> {
    resolve_atuin_sync_mode_value(
        env::var("SITUS_ATUIN_SYNC").ok().as_deref(),
        read_configured_atuin_sync_mode()?,
    )
}

pub(crate) fn read_configured_picker_mode() -> CliResult<Option<PickerMode>> {
    read_picker_mode_config(&config_path())
}

pub(crate) fn write_configured_picker_mode(mode: PickerMode) -> CliResult<PathBuf> {
    let path = config_path();
    write_picker_mode_config(&path, mode)?;
    Ok(path)
}

pub(crate) fn resolve_picker_mode() -> CliResult<PickerMode> {
    resolve_picker_mode_value(
        env::var("SITUS_PICKER").ok().as_deref(),
        read_configured_picker_mode()?,
    )
}

pub(crate) fn read_configured_language() -> CliResult<Option<String>> {
    read_config_value(&config_path(), LANGUAGE_KEY)
}

pub(crate) fn write_configured_language(lang: &str) -> CliResult<PathBuf> {
    let path = config_path();
    write_config_value(&path, LANGUAGE_KEY, lang)?;
    Ok(path)
}

pub(crate) fn resolve_picker_mode_value(
    env_value: Option<&str>,
    config_value: Option<PickerMode>,
) -> CliResult<PickerMode> {
    match env_value {
        Some(value) => parse_picker_mode(value),
        None => Ok(config_value.unwrap_or(PickerMode::Inline)),
    }
}

pub(crate) fn resolve_atuin_sync_mode_value(
    env_value: Option<&str>,
    config_value: Option<AtuinSyncMode>,
) -> CliResult<AtuinSyncMode> {
    match env_value {
        Some(value) => parse_sync_mode(Some(value)),
        None => Ok(config_value.unwrap_or(AtuinSyncMode::Off)),
    }
}

fn read_atuin_sync_config(path: &Path) -> CliResult<Option<AtuinSyncMode>> {
    read_config_value(path, ATUIN_SYNC_KEY)?
        .map(|value| parse_sync_mode(Some(value.as_str())))
        .transpose()
}

fn write_atuin_sync_config(path: &Path, mode: AtuinSyncMode) -> CliResult<()> {
    write_config_value(path, ATUIN_SYNC_KEY, sync_mode_name(mode))
}

fn read_picker_mode_config(path: &Path) -> CliResult<Option<PickerMode>> {
    read_config_value(path, PICKER_MODE_KEY)?
        .map(|value| parse_picker_mode(&value))
        .transpose()
}

fn write_picker_mode_config(path: &Path, mode: PickerMode) -> CliResult<()> {
    write_config_value(path, PICKER_MODE_KEY, picker_mode_name(mode))
}

fn read_config_value(path: &Path, target_key: &str) -> CliResult<Option<String>> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Ok(None);
    };

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() == target_key {
            return Ok(Some(value.trim().to_string()));
        }
    }

    Ok(None)
}

fn write_config_value(path: &Path, target_key: &str, target_value: &str) -> CliResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut wrote_key = false;
    let mut output = String::new();

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            if line
                .split_once('=')
                .map(|(key, _)| key.trim() == target_key)
                .unwrap_or(false)
            {
                if !wrote_key {
                    output.push_str(&format!("{target_key}={target_value}\n"));
                    wrote_key = true;
                }
                continue;
            }

            output.push_str(line);
            output.push('\n');
        }
    }

    if !wrote_key {
        output.push_str(&format!("{target_key}={target_value}\n"));
    }

    fs::write(path, output)?;
    Ok(())
}

pub(crate) fn sync_mode_name(mode: AtuinSyncMode) -> &'static str {
    match mode {
        AtuinSyncMode::Off => "off",
        AtuinSyncMode::Auto => "auto",
        AtuinSyncMode::Always => "always",
    }
}

pub(crate) fn parse_picker_mode(value: &str) -> CliResult<PickerMode> {
    match value {
        "inline" => Ok(PickerMode::Inline),
        "fullscreen" | "full-screen" | "full" => Ok(PickerMode::Fullscreen),
        unknown => Err(crate::error::cli_error(format!(
            "unknown picker mode `{unknown}`; expected `inline` or `fullscreen`"
        ))),
    }
}

pub(crate) fn picker_mode_name(mode: PickerMode) -> &'static str {
    match mode {
        PickerMode::Inline => "inline",
        PickerMode::Fullscreen => "fullscreen",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn reads_atuin_sync_mode_from_config_file() {
        let dir = temp_test_dir("read-sync-config");
        let path = dir.join("config");
        fs::write(&path, "atuin_sync=auto\n").unwrap();

        let mode = read_atuin_sync_config(&path).unwrap();

        assert_eq!(mode, Some(AtuinSyncMode::Auto));
    }

    #[test]
    fn writes_atuin_sync_mode_and_preserves_other_config_lines() {
        let dir = temp_test_dir("write-sync-config");
        let path = dir.join("config");
        fs::write(
            &path,
            "theme=quiet\npicker_mode=fullscreen\natuin_sync=off\n",
        )
        .unwrap();

        write_atuin_sync_config(&path, AtuinSyncMode::Auto).unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("theme=quiet\n"));
        assert!(contents.contains("picker_mode=fullscreen\n"));
        assert!(contents.contains("atuin_sync=auto\n"));
        assert_eq!(contents.matches("atuin_sync=").count(), 1);
    }

    #[test]
    fn resolves_env_value_before_config_value() {
        assert_eq!(
            resolve_atuin_sync_mode_value(Some("off"), Some(AtuinSyncMode::Auto)).unwrap(),
            AtuinSyncMode::Off
        );
        assert_eq!(
            resolve_atuin_sync_mode_value(None, Some(AtuinSyncMode::Auto)).unwrap(),
            AtuinSyncMode::Auto
        );
        assert_eq!(
            resolve_atuin_sync_mode_value(None, None).unwrap(),
            AtuinSyncMode::Off
        );
    }

    #[test]
    fn reads_writes_and_resolves_picker_mode() {
        let dir = temp_test_dir("picker-mode-config");
        let path = dir.join("config");
        fs::write(&path, "atuin_sync=auto\npicker_mode=fullscreen\n").unwrap();

        assert_eq!(
            read_picker_mode_config(&path).unwrap(),
            Some(PickerMode::Fullscreen)
        );

        write_picker_mode_config(&path, PickerMode::Inline).unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("atuin_sync=auto\n"));
        assert!(contents.contains("picker_mode=inline\n"));

        assert_eq!(
            resolve_picker_mode_value(Some("fullscreen"), Some(PickerMode::Inline)).unwrap(),
            PickerMode::Fullscreen
        );
        assert_eq!(
            resolve_picker_mode_value(None, Some(PickerMode::Fullscreen)).unwrap(),
            PickerMode::Fullscreen
        );
        assert_eq!(
            resolve_picker_mode_value(None, None).unwrap(),
            PickerMode::Inline
        );
    }

    #[test]
    fn reads_writes_language_config() {
        let dir = temp_test_dir("lang-config");
        let path = dir.join("config");
        fs::write(&path, "picker_mode=inline\nlanguage=ko\n").unwrap();

        assert_eq!(
            read_config_value(&path, LANGUAGE_KEY).unwrap(),
            Some("ko".to_string())
        );

        write_config_value(&path, LANGUAGE_KEY, "zh-hans").unwrap();
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("picker_mode=inline\n"));
        assert!(contents.contains("language=zh-hans\n"));
    }

    fn temp_test_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "situs-config-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
