pub(crate) fn visible_width(value: &str) -> usize {
    let mut width = 0;
    let mut chars = value.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            for next in chars.by_ref() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            width += char_display_width(character);
        }
    }

    width
}

pub(super) fn truncate_visible(value: &str, width: usize) -> String {
    let mut visible = 0;
    let mut output = String::new();
    let mut chars = value.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '\x1b' {
            output.push(character);
            for next in chars.by_ref() {
                output.push(next);
                if next == 'm' {
                    break;
                }
            }
            continue;
        }

        let character_width = char_display_width(character);
        if visible + character_width > width {
            break;
        }

        output.push(character);
        visible += character_width;
    }

    output
}

pub(super) fn take_visible_prefix(value: &str, width: usize) -> String {
    let mut visible = 0;
    let mut output = String::new();

    for character in value.chars() {
        let character_width = char_display_width(character);
        if visible + character_width > width {
            break;
        }
        output.push(character);
        visible += character_width;
    }

    output
}

pub(super) fn take_visible_suffix(value: &str, width: usize) -> String {
    let mut visible = 0;
    let mut output = Vec::new();

    for character in value.chars().rev() {
        let character_width = char_display_width(character);
        if visible + character_width > width {
            break;
        }
        output.push(character);
        visible += character_width;
    }

    output.into_iter().rev().collect()
}

fn char_display_width(character: char) -> usize {
    if character == '\0'
        || character.is_control()
        || matches!(
            character as u32,
            0x0300..=0x036F
                | 0x1AB0..=0x1AFF
                | 0x1DC0..=0x1DFF
                | 0x20D0..=0x20FF
                | 0xFE00..=0xFE0F
                | 0xFE20..=0xFE2F
        )
    {
        return 0;
    }

    if is_wide_character(character) {
        2
    } else {
        1
    }
}

fn is_wide_character(character: char) -> bool {
    matches!(
        character as u32,
        0x1100..=0x115F
            | 0x2329..=0x232A
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7A3
            | 0xF900..=0xFAFF
            | 0xFE10..=0xFE19
            | 0xFE30..=0xFE6F
            | 0xFF00..=0xFF60
            | 0xFFE0..=0xFFE6
            | 0x1F300..=0x1FAFF
    )
}
