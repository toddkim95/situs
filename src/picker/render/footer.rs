use crate::i18n::{I18n, MessageKey};
use crate::picker::style::{
    bold, help_bar, help_key_badge, query_bar, query_label_badge, query_prompt,
};
use crate::picker::width::visible_width;

use super::helpers::fit_line;

pub(super) fn query_prefix_width(i18n: I18n) -> usize {
    1 + visible_width(i18n.text(MessageKey::PickerSearch)) + 2 + 1 + visible_width("> ")
}

pub(super) fn query_line(query: &str, width: usize, i18n: I18n) -> String {
    let line = format!(
        " {} {}{}",
        query_label_badge(i18n.text(MessageKey::PickerSearch)),
        query_prompt("> "),
        query
    );

    query_bar(&fit_line(&line, width))
}

pub(super) fn query_cursor_column(query: &str, cursor: usize, width: usize) -> u16 {
    let bounded = cursor.min(query.len());
    let cursor = if query.is_char_boundary(bounded) {
        bounded
    } else {
        query
            .char_indices()
            .map(|(index, _)| index)
            .take_while(|index| *index < bounded)
            .last()
            .unwrap_or(0)
    };
    let column = query_prefix_width(I18n::from_env()) + visible_width(&query[..cursor]);
    column.min(width.saturating_sub(2)) as u16
}

pub(super) fn help_line(width: usize, i18n: I18n) -> String {
    let help = if width >= 132 {
        vec![
            ("esc", i18n.text(MessageKey::PickerHelpQuit)),
            ("up/down", i18n.text(MessageKey::PickerHelpSelect)),
            ("left/right", i18n.text(MessageKey::PickerHelpEdit)),
            ("tab", i18n.text(MessageKey::PickerHelpCd)),
            ("enter", i18n.text(MessageKey::PickerHelpRun)),
            ("ctrl-y", i18n.text(MessageKey::PickerHelpCopy)),
            ("ctrl-d", i18n.text(MessageKey::PickerHelpDelete)),
            ("f2", i18n.text(MessageKey::PickerHelpSource)),
            ("f3", i18n.text(MessageKey::PickerHelpContext)),
            ("ctrl-/", i18n.text(MessageKey::PickerHelpHelp)),
        ]
    } else if width >= 96 {
        vec![
            ("esc", i18n.text(MessageKey::PickerHelpQuit)),
            ("up/down", i18n.text(MessageKey::PickerHelpSelect)),
            ("tab", i18n.text(MessageKey::PickerHelpCd)),
            ("enter", i18n.text(MessageKey::PickerHelpRun)),
            ("ctrl-/", i18n.text(MessageKey::PickerHelpHelp)),
        ]
    } else if width >= 70 {
        vec![
            ("esc", i18n.text(MessageKey::PickerHelpQuit)),
            ("tab", i18n.text(MessageKey::PickerHelpCd)),
            ("enter", i18n.text(MessageKey::PickerHelpRun)),
        ]
    } else if width >= 42 {
        vec![("esc", ""), ("tab", ""), ("enter", "")]
    } else {
        Vec::new()
    };

    let mut line = String::from(" ");
    for (index, (key, action)) in help.iter().enumerate() {
        if index > 0 {
            line.push_str("  ");
        }
        line.push_str(&help_key_badge(key));
        if !action.is_empty() {
            line.push(' ');
            line.push_str(action);
        }
    }

    help_bar(&fit_line(&line, width))
}

pub(super) fn help_overlay_lines(width: usize, i18n: I18n) -> Vec<String> {
    let mut lines = vec![
        format!("  {}", bold(i18n.text(MessageKey::PickerKeyboard))),
        format!(
            "  {}   {}",
            help_key_badge("up/down"),
            i18n.text(MessageKey::PickerHelpSelectPrevious)
        ),
        format!(
            "  {}   {}",
            help_key_badge("left/right"),
            i18n.text(MessageKey::PickerHelpEditQuery)
        ),
        format!(
            "  {}   {}",
            help_key_badge("tab"),
            i18n.text(MessageKey::PickerHelpCdKeepQuery)
        ),
        format!(
            "  {}   {}",
            help_key_badge("enter"),
            i18n.text(MessageKey::PickerHelpRunSelected)
        ),
        format!(
            "  {} / {}   {}",
            help_key_badge("alt-enter"),
            help_key_badge("alt-y"),
            i18n.text(MessageKey::PickerHelpPasteCommand)
        ),
        format!(
            "  {}   {}",
            help_key_badge("ctrl-y"),
            i18n.text(MessageKey::PickerHelpCopyCommand)
        ),
        format!(
            "  {}   {}",
            help_key_badge("ctrl-d"),
            i18n.text(MessageKey::PickerHelpDeleteRow)
        ),
        format!(
            "  {}   {}",
            help_key_badge("f2"),
            i18n.text(MessageKey::PickerHelpCycleSource)
        ),
        format!(
            "  {}   {}",
            help_key_badge("f3"),
            i18n.text(MessageKey::PickerHelpCycleContext)
        ),
        format!(
            "  {}   {}",
            help_key_badge("ctrl-f"),
            i18n.text(MessageKey::PickerHelpShowHideFailed)
        ),
    ];

    lines
        .iter_mut()
        .for_each(|line| *line = fit_line(line, width));
    lines
}
