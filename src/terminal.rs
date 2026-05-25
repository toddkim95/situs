use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub(crate) fn terminal_device_path() -> PathBuf {
    std::env::var("SITUS_TTY")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/dev/tty"))
}

pub(crate) fn write_to_terminal(args: std::fmt::Arguments<'_>) -> io::Result<()> {
    let mut terminal = Terminal::open();
    terminal.write(args)
}

pub(crate) struct Terminal {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl Terminal {
    pub(crate) fn open() -> Self {
        if let Ok(file) = OpenOptions::new()
            .read(true)
            .write(true)
            .open(terminal_device_path())
        {
            if let Ok(output) = file.try_clone() {
                return Self {
                    input: Box::new(io::BufReader::new(file)),
                    output: Box::new(output),
                };
            }
        }

        Self {
            input: Box::new(io::BufReader::new(io::stdin())),
            output: Box::new(io::stderr()),
        }
    }

    pub(crate) fn write(&mut self, args: std::fmt::Arguments<'_>) -> io::Result<()> {
        self.output.write_fmt(args)?;
        self.output.flush()
    }

    pub(crate) fn read_line(&mut self) -> io::Result<String> {
        let mut value = String::new();
        self.input.read_line(&mut value)?;
        Ok(value)
    }
}
