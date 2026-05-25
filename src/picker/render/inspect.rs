use crate::history::human_age_at;
use crate::i18n::{I18n, MessageKey};
use crate::model::Candidate;
use crate::picker::style::{dim, green, muted_cyan, yellow};

use super::candidate::source_badge;

pub(super) fn inspect_lines(candidate: &Candidate, now: u64, i18n: I18n) -> Vec<String> {
    let status = if candidate.status == 0 {
        green("ok")
    } else {
        yellow(&format!("exit {}", candidate.status))
    };

    vec![
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectCommand)),
            candidate.command
        ),
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectCwd)),
            muted_cyan(&candidate.cwd)
        ),
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectStatus)),
            status
        ),
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectSource)),
            source_badge(candidate.source.as_str())
        ),
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectRuns)),
            i18n.picker_runs(candidate.run_count, candidate.success_count)
        ),
        format!(
            "  {} {}",
            dim(i18n.text(MessageKey::PickerInspectWhen)),
            human_age_at(now, candidate.timestamp)
        ),
        format!(
            "  {} {}",
            dim("enter"),
            i18n.text(MessageKey::PickerInspectEnter)
        ),
        format!(
            "  {} {}",
            dim("tab"),
            i18n.text(MessageKey::PickerInspectTab)
        ),
    ]
}
