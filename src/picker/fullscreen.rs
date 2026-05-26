use std::io;

use super::input::PickerState;
use super::path::{common_directory_prefix, masked_cwd};
use super::render::helpers::{truncate_end, truncate_start};
use super::session::PickerSession;
use crate::context::CommandContext;
use crate::i18n::{I18n, MessageKey};
use crate::model::Candidate;
use crate::picker::width::visible_width;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

pub(super) fn render_ratatui_fullscreen(
    session: &mut PickerSession,
    _candidates: &[Candidate],
    visible: &[(usize, &Candidate)],
    state: &PickerState,
    _command_context: &CommandContext,
) -> io::Result<()> {
    let backend = CrosstermBackend::new(&mut session.output);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.area();

        // 1. Root Vertical Layout: Main Content, fixed query line, fixed footer.
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(size);

        // 2. Horizontal Main Content Layout: Sidebar Results, Detail/Help Panel
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);

        // Left Sidebar: Candidates List
        let visible_cands: Vec<&Candidate> = visible.iter().map(|(_, c)| *c).collect();
        let common_prefix = common_directory_prefix(visible_cands.iter().copied());

        let list_items: Vec<ListItem> = visible
            .iter()
            .enumerate()
            .map(|(i, &(_, cand))| {
                let is_selected = i == state.selected;
                let indicator = if is_selected { "▶ " } else { "  " };
                let cwd = masked_cwd(&cand.cwd, common_prefix.as_deref());

                let cmd_style = if is_selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let cwd_style = if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                let text_style = if is_selected {
                    Style::default().bg(Color::Rgb(40, 44, 52))
                } else {
                    Style::default()
                };

                let spans = vec![
                    Span::styled(indicator, Style::default().fg(Color::Yellow)),
                    Span::styled(truncate_end(&cand.command, 30), cmd_style),
                    Span::raw("  "),
                    Span::styled(truncate_start(&cwd, 30), cwd_style),
                ];

                ListItem::new(Line::from(spans)).style(text_style)
            })
            .collect();

        let list_title = if visible.is_empty() {
            " Command History (0 results) ".to_string()
        } else {
            format!(
                " Command History ({} / {}) ",
                state.selected + 1,
                visible.len()
            )
        };

        let list = List::new(list_items).block(
            Block::default()
                .title(list_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

        f.render_widget(list, main_chunks[0]);

        // Right Detail/Help Panel
        let right_panel_widget = if state.show_help {
            let i18n = I18n::from_env();
            let text = vec![
                Line::from(Span::styled(
                    "Situs Shortcut Keymap",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Up / Down      ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapUpDown)),
                ]),
                Line::from(vec![
                    Span::styled("  PageUp/Down    ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapPage)),
                ]),
                Line::from(vec![
                    Span::styled("  Left / Right   ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapLeftRight)),
                ]),
                Line::from(vec![
                    Span::styled("  Home / End     ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapHomeEnd)),
                ]),
                Line::from(vec![
                    Span::styled("  Tab            ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapTab)),
                ]),
                Line::from(vec![
                    Span::styled("  Enter          ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapEnter)),
                ]),
                Line::from(vec![
                    Span::styled("  Esc / Ctrl-C   ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapEsc)),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Views & Filters",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Ctrl-/ / F1    ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapHelp)),
                ]),
                Line::from(vec![
                    Span::styled("  Ctrl-F         ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapFailed)),
                ]),
                Line::from(vec![
                    Span::styled("  Ctrl-O         ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapInspect)),
                ]),
                Line::from(vec![
                    Span::styled("  F2             ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapSource)),
                ]),
                Line::from(vec![
                    Span::styled("  F3             ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapContext)),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "History Actions",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Ctrl-Y         ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapCopy)),
                ]),
                Line::from(vec![
                    Span::styled("  Ctrl-D         ", Style::default().fg(Color::Cyan)),
                    Span::raw(i18n.text(MessageKey::KeymapDelete)),
                ]),
            ];
            Paragraph::new(text).block(
                Block::default()
                    .title(" Keyboard Help ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
        } else if let Some((_, cand)) = visible.get(state.selected) {
            let status_span = if cand.status == 0 {
                Span::styled(
                    "ok",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    format!("exit {}", cand.status),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
            };

            let source_span = match cand.source.as_str() {
                "atuin" => Span::styled("ATUIN", Style::default().fg(Color::Magenta)),
                "mixed" => Span::styled("MIXED", Style::default().fg(Color::Magenta)),
                _ => Span::styled("LOCAL", Style::default().fg(Color::Blue)),
            };

            let now = crate::history::now_seconds();
            let age_str = crate::history::human_age_at(now, cand.timestamp);
            let i18n = I18n::from_env();
            let command_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectCommand));
            let cwd_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectCwd));
            let status_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectStatus));
            let source_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectSource));
            let runs_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectRuns));
            let when_label = format!("{:<12}", i18n.text(MessageKey::PickerInspectWhen));

            let text = vec![
                Line::from(vec![
                    Span::styled(command_label, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &cand.command,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(cwd_label, Style::default().fg(Color::DarkGray)),
                    Span::styled(&cand.cwd, Style::default().fg(Color::Cyan)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled(status_label, Style::default().fg(Color::DarkGray)),
                    status_span,
                ]),
                Line::from(vec![
                    Span::styled(source_label, Style::default().fg(Color::DarkGray)),
                    source_span,
                ]),
                Line::from(vec![
                    Span::styled(runs_label, Style::default().fg(Color::DarkGray)),
                    Span::raw(format!(
                        "{} total, {} successful",
                        cand.run_count, cand.success_count
                    )),
                ]),
                Line::from(vec![
                    Span::styled(when_label, Style::default().fg(Color::DarkGray)),
                    Span::raw(age_str),
                ]),
            ];

            Paragraph::new(text).wrap(Wrap { trim: true }).block(
                Block::default()
                    .title(" Inspection Details ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
        } else {
            Paragraph::new(vec![Line::from("No candidate selected.")]).block(
                Block::default()
                    .title(" Inspection Details ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
        };

        f.render_widget(right_panel_widget, main_chunks[1]);

        // Bottom query line
        let mut search_spans = vec![
            Span::styled("Search", Style::default().fg(Color::Cyan)),
            Span::raw(" > "),
            Span::raw(&state.query),
        ];
        if let Some(ref msg) = state.message {
            search_spans.push(Span::raw("   "));
            search_spans.push(Span::styled(msg, Style::default().fg(Color::Yellow)));
        }
        let search_block = Paragraph::new(Line::from(search_spans));

        f.render_widget(search_block, chunks[1]);

        // Footer shortcut bar
        let mut footer_spans = Vec::new();
        for (key, action) in fullscreen_footer_items() {
            footer_spans.push(Span::styled(
                format!(" {key}"),
                Style::default()
                    .bg(Color::Rgb(50, 50, 50))
                    .fg(Color::Yellow),
            ));
            footer_spans.push(Span::raw(format!(" {action}  ")));
        }

        let footer =
            Paragraph::new(Line::from(footer_spans)).style(Style::default().fg(Color::DarkGray));

        f.render_widget(footer, chunks[2]);
    })?;

    let size = terminal.size()?;
    let (cursor_row, cursor_col) = fullscreen_query_cursor_position(
        &state.query,
        state.query_cursor,
        usize::from(size.width),
        usize::from(size.height),
    );
    terminal.set_cursor_position((cursor_col, cursor_row))?;
    terminal.show_cursor()?;

    Ok(())
}

fn fullscreen_query_cursor_position(
    query: &str,
    cursor: usize,
    width: usize,
    height: usize,
) -> (u16, u16) {
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
    let row = height.saturating_sub(2);
    let column = visible_width("Search > ") + visible_width(&query[..cursor]);

    (
        row.min(usize::from(u16::MAX)) as u16,
        column.min(width.saturating_sub(2)) as u16,
    )
}

fn fullscreen_footer_items() -> &'static [(&'static str, &'static str)] {
    &[
        ("Enter", "cd & run"),
        ("Tab", "cd"),
        ("Ctrl-Y", "copy"),
        ("Ctrl-D", "delete"),
        ("F2", "source"),
        ("F3", "context"),
        ("Ctrl-/", "help"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_cursor_position_stays_on_second_to_last_line() {
        let (row, column) = fullscreen_query_cursor_position("cargo", "cargo".len(), 80, 24);

        assert_eq!(row, 22);
        assert_eq!(column, 14);
    }

    #[test]
    fn query_cursor_position_counts_wide_characters() {
        let (row, column) = fullscreen_query_cursor_position("한글", "한글".len(), 80, 10);

        assert_eq!(row, 8);
        assert_eq!(column, 13);
    }

    #[test]
    fn footer_lists_only_supported_fullscreen_actions() {
        let labels = fullscreen_footer_items();

        assert!(labels.iter().any(|(key, _)| *key == "Enter"));
        assert!(labels.iter().any(|(key, _)| *key == "Tab"));
        assert!(!labels.iter().any(|(key, _)| key.contains("Alt")));
    }
}
