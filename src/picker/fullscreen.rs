use std::io;
use std::path::PathBuf;

use crate::model::Candidate;
use crate::context::CommandContext;
use crate::i18n::{I18n, MessageKey};
use super::input::PickerState;
use super::session::PickerSession;
use super::render::helpers::{truncate_end, truncate_start};
use super::path::{common_directory_prefix, masked_cwd};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Wrap},
    Terminal,
};

pub(super) fn render_ratatui_fullscreen(
    session: &mut PickerSession,
    candidates: &[Candidate],
    visible: &[(usize, &Candidate)],
    state: &PickerState,
    _command_context: &CommandContext,
) -> io::Result<()> {
    let backend = CrosstermBackend::new(&mut session.output);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.area();

        // 1. Root Vertical Layout: Main Content, Search Input, Footer
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3), // Search Input Box
                Constraint::Length(1), // Shortcut Keys Footer
            ])
            .split(size);

        // 2. Horizontal Main Content Layout: Sidebar Results, Detail/Help Panel
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[0]);

        // Left Sidebar: Candidates List
        let visible_cands: Vec<&Candidate> = visible.iter().map(|(_, c)| *c).collect();
        let common_prefix = common_directory_prefix(visible_cands.iter().map(|c| *c));

        let list_items: Vec<ListItem> = visible
            .iter()
            .enumerate()
            .map(|(i, &(_, cand))| {
                let is_selected = i == state.selected;
                let indicator = if is_selected { "▶ " } else { "  " };
                let cwd = masked_cwd(&cand.cwd, common_prefix.as_deref());

                let cmd_style = if is_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let cwd_style = if is_selected {
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
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
            format!(" Command History ({} / {}) ", state.selected + 1, visible.len())
        };

        let list = List::new(list_items)
            .block(Block::default()
                .title(list_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)));

        f.render_widget(list, main_chunks[0]);

        // Right Detail/Help Panel
        let right_panel_widget = if state.show_help {
            let i18n = I18n::from_env();
            let text = vec![
                Line::from(Span::styled("Situs Shortcut Keymap", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
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
                Line::from(Span::styled("Views & Filters", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
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
                Line::from(Span::styled("History Actions", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
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
            Paragraph::new(text)
                .block(Block::default()
                    .title(" Keyboard Help ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)))
        } else if let Some((_, cand)) = visible.get(state.selected) {
            let status_span = if cand.status == 0 {
                Span::styled("ok", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(format!("exit {}", cand.status), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            };

            let source_span = match cand.source.as_str() {
                "atuin" => Span::styled("ATUIN", Style::default().fg(Color::Magenta)),
                "mixed" => Span::styled("MIXED", Style::default().fg(Color::Magenta)),
                _ => Span::styled("LOCAL", Style::default().fg(Color::Blue)),
            };

            let now = crate::history::now_seconds();
            let age_str = crate::history::human_age_at(now, cand.timestamp);
            let i18n = I18n::from_env();

            let text = vec![
                Line::from(vec![
                    Span::styled("Command:   ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&cand.command, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Directory: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(&cand.cwd, Style::default().fg(Color::Cyan)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Status:    ", Style::default().fg(Color::DarkGray)),
                    status_span,
                ]),
                Line::from(vec![
                    Span::styled("Source:    ", Style::default().fg(Color::DarkGray)),
                    source_span,
                ]),
                Line::from(vec![
                    Span::styled("Runs:      ", Style::default().fg(Color::DarkGray)),
                    Span::raw(format!("{} total, {} successful", cand.run_count, cand.success_count)),
                ]),
                Line::from(vec![
                    Span::styled("When:      ", Style::default().fg(Color::DarkGray)),
                    Span::raw(age_str),
                ]),
            ];

            Paragraph::new(text)
                .wrap(Wrap { trim: true })
                .block(Block::default()
                    .title(" Inspection Details ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)))
        } else {
            Paragraph::new(vec![Line::from("No candidate selected.")])
                .block(Block::default()
                    .title(" Inspection Details ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray)))
        };

        f.render_widget(right_panel_widget, main_chunks[1]);

        // Bottom Search Input
        let search_title = if let Some(ref msg) = state.message {
            format!(" Search | Message: {} ", msg)
        } else {
            " Search ".to_string()
        };

        let search_block = Paragraph::new(Line::from(vec![
            Span::styled("▶ ", Style::default().fg(Color::Yellow)),
            Span::raw(&state.query),
        ]))
        .block(Block::default()
            .title(search_title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)));

        f.render_widget(search_block, chunks[1]);

        // Footer shortcut bar
        let footer_spans = vec![
            Span::styled(" Enter", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" cd & run  "),
            Span::styled(" Tab", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" cd  "),
            Span::styled(" Alt-Enter", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" paste only  "),
            Span::styled(" Ctrl-Y", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" copy  "),
            Span::styled(" Ctrl-D", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" delete  "),
            Span::styled(" F2", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" source  "),
            Span::styled(" F3", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" context  "),
            Span::styled(" Ctrl-/", Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::Yellow)),
            Span::raw(" help "),
        ];

        let footer = Paragraph::new(Line::from(footer_spans))
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(footer, chunks[2]);
    });

    // Set cursor position inside the Search Input Box
    // Column should account for border (1) plus indicator "▶ " width (2) plus cursor offset.
    let size = terminal.size()?;
    let rect = ratatui::layout::Rect::new(0, 0, size.width, size.height);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3), // Search Input Box
            Constraint::Length(1), // Shortcut Keys Footer
        ])
        .split(rect);

    let cursor_col = chunks[1].x + 3 + state.query_cursor as u16;
    let cursor_row = chunks[1].y + 1;
    terminal.set_cursor_position((cursor_col, cursor_row))?;
    terminal.show_cursor()?;

    Ok(())
}
