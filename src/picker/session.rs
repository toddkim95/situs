use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, RawFd};
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, MoveToColumn, Show};
use crossterm::event::KeyEvent;
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::model::PickerMode;
use crate::terminal::terminal_device_path;

use super::keys::{read_key_from_terminal, wait_for_input};
use super::viewport::{
    close_space_above_prompt, cursor_row_to_anchor_move_rows, last_line_to_cursor_row_move_rows,
    move_back_to_render_anchor, move_up_rows, open_space_above_prompt, picker_rows, trim_ansi_line,
};

pub(super) struct PickerSession {
    _raw_mode: RawMode,
    output: File,
    pub(super) width: usize,
    pub(super) max_rows: usize,
    cursor_row: Option<usize>,
    mode: PickerMode,
}

impl PickerSession {
    pub(super) fn start(mode: PickerMode) -> io::Result<Self> {
        let mut output = OpenOptions::new()
            .read(true)
            .write(true)
            .open(terminal_device_path())?;
        let (width, height) = terminal::size().unwrap_or((100, 24));
        let max_rows = picker_rows(mode, height);

        let raw_mode = RawMode::enable(&output)?;
        let start_result = match mode {
            PickerMode::Inline => execute!(output, Hide),
            PickerMode::Fullscreen => execute!(
                output,
                EnterAlternateScreen,
                Hide,
                MoveTo(0, 0),
                Clear(ClearType::All)
            ),
        };
        start_result?;

        if mode == PickerMode::Inline {
            open_space_above_prompt(&mut output, max_rows)?;
        }
        output.flush()?;

        Ok(Self {
            _raw_mode: raw_mode,
            output,
            width: usize::from(width),
            max_rows,
            cursor_row: None,
            mode,
        })
    }

    pub(super) fn render(
        &mut self,
        lines: &[String],
        cursor_position: Option<(usize, u16)>,
    ) -> io::Result<()> {
        if self.mode == PickerMode::Fullscreen {
            return self.render_fullscreen(lines, cursor_position);
        }

        if let Some(row) = self.cursor_row.take() {
            move_up_rows(&mut self.output, cursor_row_to_anchor_move_rows(row))?;
        }

        execute!(self.output, Hide, MoveToColumn(0))?;

        for index in 0..self.max_rows {
            if index > 0 {
                write!(self.output, "\r\n")?;
            }
            execute!(self.output, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
            let line = lines.get(index).map(String::as_str).unwrap_or("");
            write!(self.output, "{}", trim_ansi_line(line, self.width))?;
        }

        if let Some((row, column)) = cursor_position {
            let row = row.min(self.max_rows.saturating_sub(1));
            move_up_rows(
                &mut self.output,
                last_line_to_cursor_row_move_rows(self.max_rows, row),
            )?;
            execute!(self.output, MoveToColumn(column), Show)?;
            self.cursor_row = Some(row);
        } else {
            move_back_to_render_anchor(&mut self.output, self.max_rows)?;
            execute!(self.output, Show)?;
        }

        self.output.flush()
    }

    fn render_fullscreen(
        &mut self,
        lines: &[String],
        cursor_position: Option<(usize, u16)>,
    ) -> io::Result<()> {
        execute!(self.output, Hide, MoveTo(0, 0), Clear(ClearType::All))?;

        for index in 0..self.max_rows {
            if index > 0 {
                write!(self.output, "\r\n")?;
            }
            let line = lines.get(index).map(String::as_str).unwrap_or("");
            write!(self.output, "{}", trim_ansi_line(line, self.width))?;
        }

        if let Some((row, column)) = cursor_position {
            execute!(
                self.output,
                MoveTo(column, row.min(self.max_rows.saturating_sub(1)) as u16),
                Show
            )?;
        } else {
            execute!(self.output, MoveTo(0, 0), Show)?;
        }

        self.output.flush()
    }

    pub(super) fn read_key(&mut self) -> io::Result<KeyEvent> {
        read_key_from_terminal(&mut self.output)
    }

    pub(super) fn read_key_if_ready(&mut self, timeout: Duration) -> io::Result<Option<KeyEvent>> {
        if wait_for_input(self.output.as_raw_fd(), timeout)? {
            self.read_key().map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Drop for PickerSession {
    fn drop(&mut self) {
        match self.mode {
            PickerMode::Inline => {
                if let Some(row) = self.cursor_row.take() {
                    let _ = move_up_rows(&mut self.output, cursor_row_to_anchor_move_rows(row));
                }
                let _ = execute!(self.output, MoveToColumn(0));
                let _ = close_space_above_prompt(&mut self.output, self.max_rows);
                let _ = execute!(self.output, MoveToColumn(0), Show);
            }
            PickerMode::Fullscreen => {
                let _ = execute!(self.output, Show, LeaveAlternateScreen);
            }
        }
    }
}

struct RawMode {
    fd: RawFd,
    original: libc::termios,
}

impl RawMode {
    fn enable(file: &File) -> io::Result<Self> {
        let fd = file.as_raw_fd();
        let mut current = MaybeUninit::<libc::termios>::uninit();
        if unsafe { libc::tcgetattr(fd, current.as_mut_ptr()) } != 0 {
            return Err(io::Error::last_os_error());
        }

        let original = unsafe { current.assume_init() };
        let mut raw = original;
        unsafe {
            libc::cfmakeraw(&mut raw);
        }
        if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw) } != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { fd, original })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = unsafe { libc::tcsetattr(self.fd, libc::TCSANOW, &self.original) };
    }
}
