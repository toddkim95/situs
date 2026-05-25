use std::time::Duration;

use crate::picker::width::{
    take_visible_prefix, take_visible_suffix, truncate_visible, visible_width,
};

pub(super) fn join_sides(left: &str, right: &str, width: usize) -> String {
    let usable = terminal_usable_width(width);
    let left_width = visible_width(left);
    let right_width = visible_width(right);

    if left_width + 2 + right_width <= usable {
        format!(
            "{left}{}{right}",
            " ".repeat(usable - left_width - right_width)
        )
    } else if left_width <= usable {
        fit_line(left, width)
    } else {
        truncate_visible(left, usable)
    }
}

pub(super) fn fit_line(line: &str, width: usize) -> String {
    let usable = terminal_usable_width(width);
    let mut fitted = truncate_visible(line, usable);
    let fitted_width = visible_width(&fitted);
    if fitted_width < usable {
        fitted.push_str(&" ".repeat(usable - fitted_width));
    }
    fitted
}

pub(super) fn terminal_usable_width(width: usize) -> usize {
    width.saturating_sub(1).max(1)
}

pub(crate) fn truncate_start(value: &str, width: usize) -> String {
    if visible_width(value) <= width {
        return value.to_string();
    }
    if width <= 3 {
        return take_visible_prefix(value, width);
    }

    let suffix = take_visible_suffix(value, width - 3);
    format!("...{suffix}")
}

pub(crate) fn truncate_end(value: &str, width: usize) -> String {
    if visible_width(value) <= width {
        return value.to_string();
    }
    if width <= 3 {
        return take_visible_prefix(value, width);
    }

    let prefix = take_visible_prefix(value, width - 3);
    format!("{prefix}...")
}

pub(super) fn pad_end_visible(value: &str, width: usize) -> String {
    let mut value = truncate_visible(value, width);
    let value_width = visible_width(&value);
    if value_width < width {
        value.push_str(&" ".repeat(width - value_width));
    }
    value
}

pub(super) fn pad_start_visible(value: &str, width: usize) -> String {
    let value = truncate_visible(value, width);
    let value_width = visible_width(&value);
    if value_width >= width {
        value
    } else {
        format!("{}{}", " ".repeat(width - value_width), value)
    }
}

pub(super) fn format_elapsed(elapsed: Duration) -> String {
    if elapsed < Duration::from_secs(1) {
        format!("{}ms", elapsed.as_millis())
    } else {
        format!(
            "{}.{:01}s",
            elapsed.as_secs(),
            elapsed.subsec_millis() / 100
        )
    }
}
