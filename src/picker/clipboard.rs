use std::io::Write;
use std::process::{Command, Stdio};

use crate::error::CliResult;

pub(super) fn copy_text_to_clipboard(text: &str) -> CliResult<()> {
    // Try pbcopy (macOS)
    match spawn_and_copy("pbcopy", &[], text) {
        Ok(()) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(crate::error::cli_error(format!("pbcopy failed: {e}"))),
    }

    // Try xclip (Linux X11)
    match spawn_and_copy("xclip", &["-selection", "clipboard"], text) {
        Ok(()) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(crate::error::cli_error(format!("xclip failed: {e}"))),
    }

    // Try wl-copy (Linux Wayland)
    match spawn_and_copy("wl-copy", &[], text) {
        Ok(()) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(crate::error::cli_error(format!("wl-copy failed: {e}"))),
    }

    Err(crate::error::cli_error(
        "no clipboard tool found (expected pbcopy, xclip, or wl-copy)",
    ))
}

fn spawn_and_copy(cmd: &str, args: &[&str], text: &str) -> std::io::Result<()> {
    let mut child = Command::new(cmd).args(args).stdin(Stdio::piped()).spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "{cmd} exited with non-zero status"
        )))
    }
}
