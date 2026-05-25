mod encoding;
mod time;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use crate::command::{normalize_command, should_ignore_command};
use crate::error::CliResult;
use crate::model::{HistorySource, Record};

use encoding::{decode_field, encode_field};
pub(crate) use encoding::{
    decode_field as decode_storage_field, encode_field as encode_storage_field,
};
pub(crate) use time::{human_age, human_age_at, now_seconds};

pub(crate) fn load_records() -> CliResult<Vec<Record>> {
    Ok(read_records(&history_path()?)?)
}

pub(crate) fn record_executed_command(cwd: &str, status: i32, command: &str) -> io::Result<()> {
    let normalized = normalize_command(command);
    if normalized.is_empty() || should_ignore_command(&normalized) {
        return Ok(());
    }

    append_record(
        &history_path().map_err(|error| io::Error::other(error.to_string()))?,
        &Record {
            timestamp: now_seconds(),
            status,
            cwd: cwd.to_string(),
            command: normalized,
            source: HistorySource::Local,
        },
    )
}

pub(crate) fn read_records(path: &Path) -> io::Result<Vec<Record>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(record) = parse_record_line(&line) {
            records.push(record);
        }
    }

    Ok(records)
}

pub(crate) fn append_record(path: &Path, record: &Record) -> io::Result<()> {
    append_records(path, std::slice::from_ref(record))
}

pub(crate) fn append_records(path: &Path, records: &[Record]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut buffer = String::new();
    for record in records {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut buffer,
            "v2\t{}\t{}\t{}\t{}\t{}",
            record.timestamp,
            record.status,
            encode_field(&record.cwd),
            encode_field(&record.command),
            record.source.as_str()
        );
    }

    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

pub(crate) fn write_records(path: &Path, records: &[Record]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut buffer = String::new();
    for record in records {
        use std::fmt::Write as _;
        let _ = writeln!(
            &mut buffer,
            "v2\t{}\t{}\t{}\t{}\t{}",
            record.timestamp,
            record.status,
            encode_field(&record.cwd),
            encode_field(&record.command),
            record.source.as_str()
        );
    }

    let mut file = File::create(path)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

pub(crate) fn parse_record_line(line: &str) -> Option<Record> {
    let mut parts = line.split('\t');
    let version = parts.next()?;

    match version {
        "v1" => {
            let timestamp = parts.next()?.parse::<u64>().ok()?;
            let status = parts.next()?.parse::<i32>().ok()?;
            let cwd = decode_field(parts.next()?)?;
            let command = decode_field(parts.next()?)?;

            if parts.next().is_some() {
                return None;
            }

            Some(Record {
                timestamp,
                status,
                cwd,
                command,
                source: HistorySource::Local,
            })
        }
        "v2" => {
            let timestamp = parts.next()?.parse::<u64>().ok()?;
            let status = parts.next()?.parse::<i32>().ok()?;
            let cwd = decode_field(parts.next()?)?;
            let command = decode_field(parts.next()?)?;
            let source = parse_history_source(parts.next()?)?;

            if parts.next().is_some() {
                return None;
            }

            Some(Record {
                timestamp,
                status,
                cwd,
                command,
                source,
            })
        }
        _ => None,
    }
}

fn parse_history_source(value: &str) -> Option<HistorySource> {
    match value {
        "local" => Some(HistorySource::Local),
        "atuin" => Some(HistorySource::Atuin),
        _ => None,
    }
}

pub(crate) fn history_path() -> CliResult<PathBuf> {
    if let Ok(path) = env::var("SITUS_HISTORY") {
        return Ok(PathBuf::from(path));
    }

    if let Ok(path) = env::var("XDG_DATA_HOME") {
        return Ok(PathBuf::from(path).join("situs-cli").join("history.tsv"));
    }

    if let Ok(home) = env::var("HOME") {
        return Ok(PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("situs-cli")
            .join("history.tsv"));
    }

    Ok(PathBuf::from("situs-history.tsv"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_record_line_round_trip() {
        let record = Record {
            timestamp: 42,
            status: 0,
            cwd: "/tmp/project one".to_string(),
            command: "cargo build --release".to_string(),
            source: HistorySource::Atuin,
        };
        let line = format!(
            "v2\t{}\t{}\t{}\t{}\t{}",
            record.timestamp,
            record.status,
            encode_field(&record.cwd),
            encode_field(&record.command),
            record.source.as_str()
        );

        assert_eq!(parse_record_line(&line), Some(record));
    }

    #[test]
    fn parse_record_line_keeps_v1_history_as_local() {
        let record = parse_record_line("v1\t42\t0\t/tmp/app\tcargo build").unwrap();

        assert_eq!(record.source, HistorySource::Local);
        assert_eq!(record.cwd, "/tmp/app");
        assert_eq!(record.command, "cargo build");
    }
}
