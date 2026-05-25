use crate::history::human_age_at;
use crate::i18n::{I18n, MessageKey};
use crate::model::Candidate;
use crate::picker::path::masked_cwd;
use crate::picker::style::{bold, dim, green, muted_cyan, selected_bar, yellow};

use super::helpers::{
    fit_line, pad_end_visible, pad_start_visible, terminal_usable_width, truncate_end,
    truncate_start,
};

pub(super) fn candidate_header_line(width: usize, i18n: I18n) -> String {
    let usable = terminal_usable_width(width);
    if usable < 72 {
        return fit_line(
            &format!(
                "  {}",
                dim(i18n.text(MessageKey::PickerCandidateCompactHeader))
            ),
            width,
        );
    }

    let layout = candidate_layout(width);
    let mut line = format!(
        "  {}  {}",
        pad_end_visible(
            &dim(i18n.text(MessageKey::PickerCandidateCommand)),
            layout.command_width
        ),
        pad_end_visible(
            &dim(i18n.text(MessageKey::PickerCandidateDirectory)),
            layout.cwd_width
        )
    );
    if layout.flex_gap > 0 {
        line.push_str(&" ".repeat(layout.flex_gap));
    }
    line.push_str(&format!(
        "  {}  {}",
        pad_start_visible(
            &dim(i18n.text(MessageKey::PickerCandidateStatus)),
            layout.status_width
        ),
        pad_start_visible(
            &dim(i18n.text(MessageKey::PickerCandidateWhen)),
            layout.age_width
        )
    ));
    fit_line(&line, width)
}

pub(super) fn candidate_line(
    index: usize,
    candidate: &Candidate,
    selected_index: usize,
    common_cwd_prefix: Option<&str>,
    width: usize,
    now: u64,
) -> String {
    let selected = index == selected_index;
    let usable = terminal_usable_width(width);
    let cwd = masked_cwd(&candidate.cwd, common_cwd_prefix);
    let row = if usable < 72 {
        compact_candidate_line(candidate, &cwd, selected, width, now)
    } else {
        spacious_candidate_line(candidate, &cwd, selected, width, now)
    };

    if selected {
        selected_bar(&fit_line(&row, width))
    } else {
        fit_line(&row, width)
    }
}

fn compact_candidate_line(
    candidate: &Candidate,
    display_cwd: &str,
    selected: bool,
    width: usize,
    now: u64,
) -> String {
    let usable = terminal_usable_width(width);
    let marker = marker(selected);
    let status = status_badge(candidate.status);
    let source = source_badge(candidate.source.as_str());
    let age = dim(&human_age_at(now, candidate.timestamp));
    let reserved =
        2 + 2 + visible_width(&status) + 1 + visible_width(&source) + 2 + visible_width(&age);
    let body_width = usable.saturating_sub(reserved).max(10);
    let command_width = (body_width * 2 / 3).max(8);
    let cwd_width = body_width.saturating_sub(command_width + 2).max(6);
    let command = truncate_end(&candidate.command, command_width);
    let cwd = truncate_start(display_cwd, cwd_width);
    let command = if selected { bold(&command) } else { command };

    format!(
        "{marker} {}  {}  {status} {source}  {age}",
        pad_end_visible(&command, command_width),
        muted_cyan(&cwd)
    )
}

fn spacious_candidate_line(
    candidate: &Candidate,
    display_cwd: &str,
    selected: bool,
    width: usize,
    now: u64,
) -> String {
    let marker = marker(selected);
    let layout = candidate_layout(width);

    let command = truncate_end(&candidate.command, layout.command_width);
    let cwd = truncate_start(display_cwd, layout.cwd_width);
    let status = format!(
        "{} {}",
        status_badge(candidate.status),
        source_badge(candidate.source.as_str())
    );
    let age = dim(&human_age_at(now, candidate.timestamp));
    let command = if selected { bold(&command) } else { command };

    let mut line = format!(
        "{marker} {}  {}",
        pad_end_visible(&command, layout.command_width),
        pad_end_visible(&muted_cyan(&cwd), layout.cwd_width)
    );
    if layout.flex_gap > 0 {
        line.push_str(&" ".repeat(layout.flex_gap));
    }
    line.push_str(&format!(
        "  {}  {}",
        pad_start_visible(&status, layout.status_width),
        pad_start_visible(&age, layout.age_width)
    ));
    line
}

struct CandidateLayout {
    command_width: usize,
    cwd_width: usize,
    status_width: usize,
    age_width: usize,
    flex_gap: usize,
}

fn candidate_layout(width: usize) -> CandidateLayout {
    let usable = terminal_usable_width(width);
    let status_width = 14;
    let age_width = 10;
    let fixed = 2 + 2 + 2 + status_width + 2 + age_width;
    let available = usable.saturating_sub(fixed);
    let command_width = (available * 5 / 12).clamp(18, 72);
    let cwd_width = (available * 4 / 12).clamp(16, 72);
    let used = fixed + command_width + cwd_width;
    let flex_gap = usable.saturating_sub(used);

    CandidateLayout {
        command_width,
        cwd_width,
        status_width,
        age_width,
        flex_gap,
    }
}

fn marker(selected: bool) -> String {
    if selected {
        green(">")
    } else {
        " ".to_string()
    }
}

fn status_badge(status: i32) -> String {
    if status == 0 {
        green("OK")
    } else {
        yellow(&format!("EXIT {status}"))
    }
}

pub(super) fn source_badge(source: &str) -> String {
    match source {
        "atuin" => muted_cyan("ATUIN"),
        "mixed" => muted_cyan("MIXED"),
        _ => dim("LOCAL"),
    }
}

fn visible_width(value: &str) -> usize {
    crate::picker::width::visible_width(value)
}
