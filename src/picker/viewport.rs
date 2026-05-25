use std::env;
use std::io::{self, Write};

use crossterm::cursor::{MoveToColumn, MoveUp};
use crossterm::execute;

use crate::model::PickerMode;
use crate::picker::width::truncate_visible;

pub(super) fn picker_rows(mode: PickerMode, terminal_height: u16) -> usize {
    match mode {
        PickerMode::Inline => inline_picker_rows(terminal_height),
        PickerMode::Fullscreen => fullscreen_picker_rows(terminal_height),
    }
}

fn inline_picker_rows(terminal_height: u16) -> usize {
    let requested = env::var("SITUS_INLINE_ROWS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(14);
    let available = usize::from(terminal_height.saturating_sub(2)).max(6);

    requested.clamp(6, available.min(22))
}

fn fullscreen_picker_rows(terminal_height: u16) -> usize {
    usize::from(terminal_height).clamp(8, 80)
}

pub(super) fn trim_ansi_line(line: &str, width: usize) -> String {
    let width = width.saturating_sub(1).max(1);
    let mut output = truncate_visible(line, width);
    output.push_str("\x1b[0m");
    output
}

pub(super) fn move_back_to_render_anchor(
    output: &mut impl Write,
    render_count: usize,
) -> io::Result<()> {
    let rows = render_anchor_move_rows(render_count);
    move_up_rows(output, rows)?;
    Ok(())
}

pub(super) fn move_up_rows(output: &mut impl Write, rows: u16) -> io::Result<()> {
    if rows > 0 {
        execute!(output, MoveUp(rows), MoveToColumn(0))?;
    } else {
        execute!(output, MoveToColumn(0))?;
    }
    Ok(())
}

pub(super) fn render_anchor_move_rows(render_count: usize) -> u16 {
    render_count.saturating_sub(1).min(usize::from(u16::MAX)) as u16
}

pub(super) fn cursor_row_to_anchor_move_rows(cursor_row: usize) -> u16 {
    cursor_row.min(usize::from(u16::MAX)) as u16
}

pub(super) fn last_line_to_cursor_row_move_rows(render_count: usize, cursor_row: usize) -> u16 {
    render_count
        .saturating_sub(1)
        .saturating_sub(cursor_row)
        .min(usize::from(u16::MAX)) as u16
}

pub(super) fn open_space_above_prompt(output: &mut impl Write, count: usize) -> io::Result<()> {
    let Some(sequence) = open_space_above_prompt_sequence(count) else {
        return Ok(());
    };
    output.write_all(sequence.as_bytes())
}

pub(super) fn close_space_above_prompt(output: &mut impl Write, count: usize) -> io::Result<()> {
    let Some(sequence) = close_space_above_prompt_sequence(count) else {
        return Ok(());
    };
    output.write_all(sequence.as_bytes())
}

fn insert_lines_sequence(count: usize) -> Option<String> {
    terminal_line_sequence(count, 'L')
}

fn delete_lines_sequence(count: usize) -> Option<String> {
    terminal_line_sequence(count, 'M')
}

fn open_space_above_prompt_sequence(count: usize) -> Option<String> {
    insert_lines_sequence(count).map(|insert| format!("\x1b[1A\x1b[1G{insert}"))
}

fn close_space_above_prompt_sequence(count: usize) -> Option<String> {
    delete_lines_sequence(count).map(|delete| format!("{delete}\x1b[1B"))
}

fn terminal_line_sequence(count: usize, command: char) -> Option<String> {
    if count == 0 {
        None
    } else {
        Some(format!(
            "\x1b[{}{command}",
            count.min(usize::from(u16::MAX))
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_ansi_line_does_not_pad_to_the_last_terminal_column() {
        let trimmed = trim_ansi_line("abc", 10);

        assert_eq!(visible_width(&trimmed), 3);
        assert_eq!(trimmed, "abc\x1b[0m");
    }

    #[test]
    fn trim_ansi_line_leaves_one_column_for_terminal_autowrap() {
        let trimmed = trim_ansi_line("abcdef", 4);

        assert_eq!(visible_width(&trimmed), 3);
        assert_eq!(trimmed, "abc\x1b[0m");
    }

    #[test]
    fn trim_ansi_line_counts_wide_unicode_columns() {
        let trimmed = trim_ansi_line("한글abc", 6);

        assert_eq!(visible_width(&trimmed), 5);
        assert_eq!(trimmed, "한글a\x1b[0m");
    }

    #[test]
    fn render_anchor_moves_back_to_first_rendered_line() {
        assert_eq!(render_anchor_move_rows(0), 0);
        assert_eq!(render_anchor_move_rows(1), 0);
        assert_eq!(render_anchor_move_rows(5), 4);
    }

    #[test]
    fn cursor_row_moves_are_relative_to_the_rendered_viewport() {
        assert_eq!(cursor_row_to_anchor_move_rows(0), 0);
        assert_eq!(cursor_row_to_anchor_move_rows(6), 6);
        assert_eq!(last_line_to_cursor_row_move_rows(8, 6), 1);
        assert_eq!(last_line_to_cursor_row_move_rows(8, 7), 0);
    }

    #[test]
    fn picker_rows_follow_inline_or_fullscreen_mode() {
        assert_eq!(picker_rows(PickerMode::Inline, 40), 14);
        assert_eq!(picker_rows(PickerMode::Fullscreen, 40), 40);
        assert_eq!(picker_rows(PickerMode::Fullscreen, 4), 8);
    }

    #[test]
    fn line_insert_and_delete_sequences_use_ansi_line_editing() {
        assert_eq!(insert_lines_sequence(0), None);
        assert_eq!(insert_lines_sequence(3).as_deref(), Some("\x1b[3L"));
        assert_eq!(delete_lines_sequence(3).as_deref(), Some("\x1b[3M"));
    }

    #[test]
    fn opening_space_moves_to_the_prompt_line_before_inserting_rows() {
        assert_eq!(open_space_above_prompt_sequence(0), None);
        assert_eq!(
            open_space_above_prompt_sequence(3).as_deref(),
            Some("\x1b[1A\x1b[1G\x1b[3L")
        );
    }

    #[test]
    fn closing_space_returns_to_the_original_prompt_cursor_row() {
        assert_eq!(close_space_above_prompt_sequence(0), None);
        assert_eq!(
            close_space_above_prompt_sequence(3).as_deref(),
            Some("\x1b[3M\x1b[1B")
        );
    }

    fn visible_width(value: &str) -> usize {
        super::super::width::visible_width(value)
    }
}
