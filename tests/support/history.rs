use std::path::Path;

pub(crate) fn record_line(timestamp: &str, status: i32, cwd: &Path, command: &str) -> String {
    format!(
        "v1\t{timestamp}\t{status}\t{}\t{}\n",
        encode_field(&cwd.to_string_lossy()),
        encode_field(command)
    )
}

fn encode_field(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric()
            || matches!(byte, b' ' | b'/' | b'.' | b'_' | b'-' | b':' | b'+')
        {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(hex_digit(byte >> 4));
            encoded.push(hex_digit(byte & 0x0f));
        }
    }
    encoded
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'A' + value - 10) as char,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_line_escapes_tabs_and_newlines() {
        let line = record_line("7", 0, Path::new("/tmp/a\tb"), "echo hi\nthere");

        assert_eq!(line, "v1\t7\t0\t/tmp/a%09b\techo hi%0Athere\n");
    }
}
