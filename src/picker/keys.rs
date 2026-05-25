use std::fs::File;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) fn read_key_from_terminal(input: &mut File) -> io::Result<KeyEvent> {
    let byte = read_byte(input)?;
    if byte == 0x1b {
        read_escape_key(input)
    } else if byte.is_ascii() {
        Ok(decode_single_byte_key(byte))
    } else {
        read_utf8_key(input, byte)
    }
}

fn read_escape_key(input: &mut File) -> io::Result<KeyEvent> {
    let Some(next) = read_byte_if_ready(input, Duration::from_millis(30))? else {
        return Ok(key(KeyCode::Esc));
    };

    match next {
        b'[' => read_csi_key(input),
        b'O' => read_ss3_key(input),
        _ => Ok(key(KeyCode::Esc)),
    }
}

fn read_csi_key(input: &mut File) -> io::Result<KeyEvent> {
    let mut sequence = Vec::new();
    loop {
        let byte = read_byte(input)?;
        sequence.push(byte);
        if (0x40..=0x7e).contains(&byte) {
            break;
        }
        if sequence.len() >= 16 {
            return Ok(key(KeyCode::Esc));
        }
    }

    Ok(decode_csi_key(&sequence))
}

fn read_ss3_key(input: &mut File) -> io::Result<KeyEvent> {
    read_byte(input).map(decode_ss3_key)
}

fn read_utf8_key(input: &mut File, first: u8) -> io::Result<KeyEvent> {
    let width = utf8_char_width(first);
    if width <= 1 {
        return Ok(key(KeyCode::Null));
    }

    let mut bytes = vec![first];
    for _ in 1..width {
        bytes.push(read_byte(input)?);
    }
    let character = std::str::from_utf8(&bytes)
        .ok()
        .and_then(|text| text.chars().next())
        .unwrap_or('\0');
    Ok(if character == '\0' {
        key(KeyCode::Null)
    } else {
        key(KeyCode::Char(character))
    })
}

fn decode_single_byte_key(byte: u8) -> KeyEvent {
    match byte {
        b'\r' | b'\n' => key(KeyCode::Enter),
        b'\t' => key(KeyCode::Tab),
        0x7f | 0x08 => key(KeyCode::Backspace),
        0x01 => control_key('a'),
        0x03 => control_key('c'),
        0x04 => control_key('d'),
        0x05 => control_key('e'),
        0x06 => control_key('f'),
        0x0f => control_key('o'),
        0x15 => control_key('u'),
        0x19 => control_key('y'),
        0x1f => control_key('_'),
        byte if byte.is_ascii_control() => key(KeyCode::Null),
        byte if byte.is_ascii() => key(KeyCode::Char(byte as char)),
        _ => key(KeyCode::Null),
    }
}

fn decode_csi_key(sequence: &[u8]) -> KeyEvent {
    match sequence {
        b"A" => key(KeyCode::Up),
        b"B" => key(KeyCode::Down),
        b"C" => key(KeyCode::Right),
        b"D" => key(KeyCode::Left),
        b"H" | b"1~" | b"7~" => key(KeyCode::Home),
        b"F" | b"4~" | b"8~" => key(KeyCode::End),
        b"3~" => key(KeyCode::Delete),
        b"5~" => key(KeyCode::PageUp),
        b"6~" => key(KeyCode::PageDown),
        b"Z" => key(KeyCode::BackTab),
        _ => key(KeyCode::Esc),
    }
}

fn decode_ss3_key(byte: u8) -> KeyEvent {
    match byte {
        b'P' => key(KeyCode::F(1)),
        b'Q' => key(KeyCode::F(2)),
        b'R' => key(KeyCode::F(3)),
        b'H' => key(KeyCode::Home),
        b'F' => key(KeyCode::End),
        b'A' => key(KeyCode::Up),
        b'B' => key(KeyCode::Down),
        b'C' => key(KeyCode::Right),
        b'D' => key(KeyCode::Left),
        _ => key(KeyCode::Esc),
    }
}

fn utf8_char_width(first: u8) -> usize {
    match first {
        0x00..=0x7f => 1,
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => 0,
    }
}

fn read_byte(input: &mut File) -> io::Result<u8> {
    let mut byte = [0];
    std::io::Read::read_exact(input, &mut byte)?;
    Ok(byte[0])
}

fn read_byte_if_ready(input: &mut File, timeout: Duration) -> io::Result<Option<u8>> {
    if wait_for_input(input.as_raw_fd(), timeout)? {
        read_byte(input).map(Some)
    } else {
        Ok(None)
    }
}

pub(super) fn wait_for_input(fd: RawFd, timeout: Duration) -> io::Result<bool> {
    let mut readfds = unsafe { std::mem::MaybeUninit::<libc::fd_set>::zeroed().assume_init() };
    unsafe {
        libc::FD_ZERO(&mut readfds);
        libc::FD_SET(fd, &mut readfds);
    }

    let mut timeval = libc::timeval {
        tv_sec: timeout.as_secs() as libc::time_t,
        tv_usec: timeout.subsec_micros() as libc::suseconds_t,
    };

    let result = unsafe {
        libc::select(
            fd + 1,
            &mut readfds,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeval,
        )
    };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result > 0)
    }
}

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn control_key(character: char) -> KeyEvent {
    KeyEvent::new(KeyCode::Char(character), KeyModifiers::CONTROL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_arrow_escape_sequences() {
        assert_eq!(decode_csi_key(b"A").code, KeyCode::Up);
        assert_eq!(decode_csi_key(b"B").code, KeyCode::Down);
        assert_eq!(decode_csi_key(b"C").code, KeyCode::Right);
        assert_eq!(decode_csi_key(b"D").code, KeyCode::Left);
    }

    #[test]
    fn decodes_shift_tab_and_function_sequences() {
        assert_eq!(decode_csi_key(b"Z").code, KeyCode::BackTab);
        assert_eq!(decode_ss3_key(b'P').code, KeyCode::F(1));
        assert_eq!(decode_ss3_key(b'Q').code, KeyCode::F(2));
        assert_eq!(decode_ss3_key(b'R').code, KeyCode::F(3));
    }

    #[test]
    fn decodes_control_and_utf8_keys() {
        assert_eq!(
            decode_single_byte_key(0x06).modifiers,
            KeyModifiers::CONTROL
        );
        assert_eq!(utf8_char_width(0xed), 3);
    }
}
