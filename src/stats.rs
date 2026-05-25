use std::collections::HashMap;

use crate::i18n::{I18n, MessageKey};
use crate::model::{HistorySource, Record};

const DEFAULT_LIMIT: usize = 5;

pub(crate) fn stats_report(records: &[Record]) -> String {
    format_stats_report_with_i18n(records, DEFAULT_LIMIT, I18n::from_env())
}

#[cfg(test)]
pub(crate) fn format_stats_report(records: &[Record], limit: usize) -> String {
    format_stats_report_with_i18n(records, limit, I18n::english())
}

fn format_stats_report_with_i18n(records: &[Record], limit: usize, i18n: I18n) -> String {
    let successful = records.iter().filter(|record| record.status == 0).count();
    let failed = records.len().saturating_sub(successful);
    let local = records
        .iter()
        .filter(|record| record.source == HistorySource::Local)
        .count();
    let atuin = records
        .iter()
        .filter(|record| record.source == HistorySource::Atuin)
        .count();

    let top_commands = top_counts(records.iter().map(|record| record.command.as_str()), limit);
    let top_directories = top_counts(records.iter().map(|record| record.cwd.as_str()), limit);

    let mut output = format!(
        "\
{}
{:<12}{}
{:<12}{}
{:<12}{}
{:<12}{}
{:<12}{}
",
        i18n.text(MessageKey::StatsTitle),
        i18n.text(MessageKey::StatsRecords),
        records.len(),
        i18n.text(MessageKey::StatsSuccessful),
        successful,
        i18n.text(MessageKey::StatsFailed),
        failed,
        i18n.text(MessageKey::StatsLocal),
        local,
        i18n.text(MessageKey::StatsAtuin),
        atuin
    );

    output.push_str(&format!("\n{}\n", i18n.text(MessageKey::StatsTopCommands)));
    push_counts(&mut output, &top_commands, i18n);

    output.push_str(&format!(
        "\n{}\n",
        i18n.text(MessageKey::StatsTopDirectories)
    ));
    push_counts(&mut output, &top_directories, i18n);

    output
}

fn top_counts<'a>(values: impl Iterator<Item = &'a str>, limit: usize) -> Vec<(usize, String)> {
    let mut counts = HashMap::<String, usize>::new();
    for value in values {
        *counts.entry(value.to_string()).or_default() += 1;
    }

    let mut counts = counts
        .into_iter()
        .map(|(value, count)| (count, value))
        .collect::<Vec<_>>();
    counts.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));
    counts.truncate(limit);
    counts
}

fn push_counts(output: &mut String, counts: &[(usize, String)], i18n: I18n) {
    if counts.is_empty() {
        output.push_str(&format!("  {}\n", i18n.text(MessageKey::StatsNone)));
        return;
    }

    for (count, value) in counts {
        output.push_str(&format!("  {:>4}  {value}\n", count));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(status: i32, cwd: &str, command: &str, source: HistorySource) -> Record {
        Record {
            timestamp: 1,
            status,
            cwd: cwd.to_string(),
            command: command.to_string(),
            source,
        }
    }

    #[test]
    fn stats_report_summarizes_history() {
        let records = vec![
            record(0, "/repo", "cargo test", HistorySource::Local),
            record(1, "/repo", "cargo test", HistorySource::Atuin),
            record(0, "/repo/api", "cargo build", HistorySource::Local),
        ];

        let report = format_stats_report(&records, 2);

        assert!(report.contains("records     3"));
        assert!(report.contains("successful  2"));
        assert!(report.contains("failed      1"));
        assert!(report.contains("local       2"));
        assert!(report.contains("atuin       1"));
        assert!(report.contains("2  cargo test"));
        assert!(report.contains("2  /repo"));
    }
}
