use std::io::{self, Write};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::tty::IsTty;
use crossterm::{execute, queue};

use crate::atuin::AtuinSyncMode;
use crate::config::{
    picker_mode_name, read_configured_atuin_sync_mode, read_configured_picker_mode,
};
use crate::error::{cli_error, CliResult};
use crate::i18n::{I18n, Locale, MessageKey};
use crate::model::PickerMode;

const LOCALES: &[Locale] = &[
    Locale::En,
    Locale::Ko,
    Locale::ZhHans,
    Locale::Es,
    Locale::Ja,
];
const SYNC_MODES: &[AtuinSyncMode] = &[
    AtuinSyncMode::Off,
    AtuinSyncMode::Auto,
    AtuinSyncMode::Always,
];
const PICKER_MODES: &[PickerMode] = &[PickerMode::Inline, PickerMode::Fullscreen];

#[derive(Clone, Copy, PartialEq, Eq)]
enum ActiveSettingRow {
    PickerMode,
    AtuinSync,
    Language,
    Save,
    Cancel,
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

pub(crate) fn setup_command(args: &[String]) -> CliResult<i32> {
    if !args.is_empty() {
        return Err(cli_error("setup does not accept arguments yet"));
    }

    if !io::stdout().is_tty() || !io::stdin().is_tty() {
        return setup_cli_fallback();
    }

    // Existing settings
    let initial_picker_mode = read_configured_picker_mode()?.unwrap_or(PickerMode::Inline);
    let initial_atuin_sync = read_configured_atuin_sync_mode()?.unwrap_or(AtuinSyncMode::Off);
    let initial_locale = Locale::from_env();

    let mut picker_mode = initial_picker_mode;
    let mut atuin_sync = initial_atuin_sync;
    let mut current_locale = initial_locale;
    let mut active_row = ActiveSettingRow::PickerMode;

    let _guard =
        RawModeGuard::new().map_err(|e| cli_error(format!("failed to start TUI raw mode: {e}")))?;
    let mut stdout = io::stdout();

    loop {
        let i18n = I18n::new(current_locale);
        draw_setup(
            &mut stdout,
            i18n,
            picker_mode,
            atuin_sync,
            current_locale,
            active_row,
        )?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => active_row = prev_row(active_row),
                    KeyCode::Down => active_row = next_row(active_row),
                    KeyCode::Left => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, false);
                        }
                        ActiveSettingRow::AtuinSync => {
                            atuin_sync = cycle_atuin_sync(atuin_sync, false);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, false);
                        }
                        _ => {}
                    },
                    KeyCode::Right => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, true);
                        }
                        ActiveSettingRow::AtuinSync => {
                            atuin_sync = cycle_atuin_sync(atuin_sync, true);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, true);
                        }
                        _ => {}
                    },
                    KeyCode::Char(' ') | KeyCode::Enter => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, true);
                        }
                        ActiveSettingRow::AtuinSync => {
                            atuin_sync = cycle_atuin_sync(atuin_sync, true);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, true);
                        }
                        ActiveSettingRow::Save => {
                            drop(_guard);
                            save_settings(picker_mode, atuin_sync, current_locale, i18n)?;
                            return Ok(0);
                        }
                        ActiveSettingRow::Cancel => {
                            return Ok(0);
                        }
                    },
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        drop(_guard);
                        save_settings(picker_mode, atuin_sync, current_locale, i18n)?;
                        return Ok(0);
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(0);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn next_row(row: ActiveSettingRow) -> ActiveSettingRow {
    match row {
        ActiveSettingRow::PickerMode => ActiveSettingRow::AtuinSync,
        ActiveSettingRow::AtuinSync => ActiveSettingRow::Language,
        ActiveSettingRow::Language => ActiveSettingRow::Save,
        ActiveSettingRow::Save => ActiveSettingRow::Cancel,
        ActiveSettingRow::Cancel => ActiveSettingRow::PickerMode,
    }
}

fn prev_row(row: ActiveSettingRow) -> ActiveSettingRow {
    match row {
        ActiveSettingRow::PickerMode => ActiveSettingRow::Cancel,
        ActiveSettingRow::AtuinSync => ActiveSettingRow::PickerMode,
        ActiveSettingRow::Language => ActiveSettingRow::AtuinSync,
        ActiveSettingRow::Save => ActiveSettingRow::Language,
        ActiveSettingRow::Cancel => ActiveSettingRow::Save,
    }
}

fn cycle_picker_mode(current: PickerMode, _forward: bool) -> PickerMode {
    match current {
        PickerMode::Inline => PickerMode::Fullscreen,
        PickerMode::Fullscreen => PickerMode::Inline,
    }
}

fn cycle_atuin_sync(current: AtuinSyncMode, forward: bool) -> AtuinSyncMode {
    let idx = SYNC_MODES.iter().position(|m| *m == current).unwrap_or(0);
    let next_idx = if forward {
        (idx + 1) % SYNC_MODES.len()
    } else {
        (idx + SYNC_MODES.len() - 1) % SYNC_MODES.len()
    };
    SYNC_MODES[next_idx]
}

fn cycle_language(current: Locale, forward: bool) -> Locale {
    let idx = LOCALES.iter().position(|m| *m == current).unwrap_or(0);
    let next_idx = if forward {
        (idx + 1) % LOCALES.len()
    } else {
        (idx + LOCALES.len() - 1) % LOCALES.len()
    };
    LOCALES[next_idx]
}

fn save_settings(
    picker_mode: PickerMode,
    atuin_sync: AtuinSyncMode,
    language: Locale,
    i18n: I18n,
) -> CliResult<()> {
    let _ = crate::config::write_configured_picker_mode(picker_mode)?;
    let _ = crate::config::write_configured_atuin_sync_mode(atuin_sync)?;
    let lang_str = match language {
        Locale::En => "en",
        Locale::Ko => "ko",
        Locale::ZhHans => "zh-hans",
        Locale::Es => "es",
        Locale::Ja => "ja",
    };
    let _ = crate::config::write_configured_language(lang_str)?;

    println!("{}", i18n.text(MessageKey::SetupTuiSavedMessage));
    Ok(())
}

fn draw_setup(
    stdout: &mut io::Stdout,
    i18n: I18n,
    picker_mode: PickerMode,
    atuin_sync: AtuinSyncMode,
    language: Locale,
    active_row: ActiveSettingRow,
) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let title = i18n.text(MessageKey::SetupTuiTitle);
    let help = i18n.text(MessageKey::SetupTuiHelp);

    let start_y = (height / 2).saturating_sub(6);
    let center_x = |text: &str| -> u16 {
        let text_len = text.chars().count() as u16;
        (width / 2).saturating_sub(text_len / 2)
    };

    // Title
    queue!(
        stdout,
        crossterm::cursor::MoveTo(center_x(title), start_y),
        SetForegroundColor(Color::Cyan),
        Print(title),
        ResetColor
    )?;

    // Border line
    let border = "═".repeat(title.chars().count() + 8);
    queue!(
        stdout,
        crossterm::cursor::MoveTo(center_x(&border), start_y + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(&border),
        ResetColor
    )?;

    // Help description
    queue!(
        stdout,
        crossterm::cursor::MoveTo(
            (width / 2).saturating_sub(help.chars().count() as u16 / 2),
            start_y + 3
        ),
        SetForegroundColor(Color::DarkGrey),
        Print(help),
        ResetColor
    )?;

    let content_start_x = (width / 2).saturating_sub(25).max(4);

    // 1. Picker Mode
    let picker_label = format!("{}: ", i18n.text(MessageKey::SetupTuiPickerMode));
    let picker_y = start_y + 5;
    queue!(stdout, crossterm::cursor::MoveTo(content_start_x, picker_y))?;
    if active_row == ActiveSettingRow::PickerMode {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(&picker_label))?;

    let option_align_x = content_start_x + 22;
    queue!(stdout, crossterm::cursor::MoveTo(option_align_x, picker_y))?;
    for mode in PICKER_MODES {
        let is_selected = *mode == picker_mode;
        let prefix = if is_selected { "● " } else { "○ " };
        let mode_str = match mode {
            PickerMode::Inline => "Inline    ",
            PickerMode::Fullscreen => "Fullscreen",
        };
        if is_selected {
            queue!(stdout, SetForegroundColor(Color::Green))?;
        }
        queue!(
            stdout,
            Print(prefix),
            Print(mode_str),
            ResetColor,
            Print("  ")
        )?;
    }

    // 2. Atuin Auto-Sync
    let sync_label = format!("{}: ", i18n.text(MessageKey::SetupTuiAtuinSync));
    let sync_y = start_y + 7;
    queue!(stdout, crossterm::cursor::MoveTo(content_start_x, sync_y))?;
    if active_row == ActiveSettingRow::AtuinSync {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(&sync_label))?;

    queue!(stdout, crossterm::cursor::MoveTo(option_align_x, sync_y))?;
    for mode in SYNC_MODES {
        let is_selected = *mode == atuin_sync;
        let prefix = if is_selected { "● " } else { "○ " };
        let sync_str = match mode {
            AtuinSyncMode::Off => "Off      ",
            AtuinSyncMode::Auto => "Auto     ",
            AtuinSyncMode::Always => "Always   ",
        };
        if is_selected {
            queue!(stdout, SetForegroundColor(Color::Green))?;
        }
        queue!(
            stdout,
            Print(prefix),
            Print(sync_str),
            ResetColor,
            Print("  ")
        )?;
    }

    // 3. Language
    let lang_label = format!("{}: ", i18n.text(MessageKey::SetupTuiLanguage));
    let lang_y = start_y + 9;
    queue!(stdout, crossterm::cursor::MoveTo(content_start_x, lang_y))?;
    if active_row == ActiveSettingRow::Language {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(&lang_label))?;

    queue!(stdout, crossterm::cursor::MoveTo(option_align_x, lang_y))?;
    for locale in LOCALES {
        let is_selected = *locale == language;
        let prefix = if is_selected { "● " } else { "○ " };
        let lang_name = match locale {
            Locale::En => "En  ",
            Locale::Ko => "Ko  ",
            Locale::ZhHans => "Zh  ",
            Locale::Es => "Es  ",
            Locale::Ja => "Ja  ",
        };
        if is_selected {
            queue!(stdout, SetForegroundColor(Color::Green))?;
        }
        queue!(
            stdout,
            Print(prefix),
            Print(lang_name),
            ResetColor,
            Print("  ")
        )?;
    }

    // 4. Save Button
    let save_btn = i18n.text(MessageKey::SetupTuiSaveBtn);
    let save_y = start_y + 12;
    queue!(
        stdout,
        crossterm::cursor::MoveTo(
            (width / 2).saturating_sub(save_btn.chars().count() as u16 / 2 + 2),
            save_y
        )
    )?;
    if active_row == ActiveSettingRow::Save {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            SetBackgroundColor(Color::DarkGrey)
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(save_btn), ResetColor)?;

    // 5. Cancel Button
    let cancel_btn = i18n.text(MessageKey::SetupTuiCancelBtn);
    let cancel_y = start_y + 13;
    queue!(
        stdout,
        crossterm::cursor::MoveTo(
            (width / 2).saturating_sub(cancel_btn.chars().count() as u16 / 2 + 2),
            cancel_y
        )
    )?;
    if active_row == ActiveSettingRow::Cancel {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            SetBackgroundColor(Color::DarkGrey)
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(cancel_btn), ResetColor)?;

    stdout.flush()?;
    Ok(())
}

fn setup_cli_fallback() -> CliResult<i32> {
    let i18n = I18n::from_env();
    let picker_mode = prompt_picker_mode_cli(i18n)?;
    let picker_path = crate::config::write_configured_picker_mode(picker_mode)?;
    println!(
        "{} `{}` in {}",
        i18n.text(MessageKey::SetupPickerModeSetPrefix),
        picker_mode_name(picker_mode),
        picker_path.display()
    );

    if crate::atuin::default_atuin_db_path()
        .as_deref()
        .map(|path| path.exists())
        .unwrap_or(false)
        && prompt_yes_no_cli(i18n.text(MessageKey::SetupAtuinFound), true)?
    {
        let path = crate::config::write_configured_atuin_sync_mode(AtuinSyncMode::Auto)?;
        println!(
            "{} `auto` in {}",
            i18n.text(MessageKey::SetupAtuinSetPrefix),
            path.display()
        );
    }

    println!();
    println!("{}", i18n.text(MessageKey::SetupZshrcHint));
    println!("  eval \"$(situs init zsh)\"");
    Ok(0)
}

fn prompt_picker_mode_cli(i18n: I18n) -> CliResult<PickerMode> {
    println!("{}", i18n.text(MessageKey::SetupTitle));
    println!();
    println!("{}", i18n.text(MessageKey::SetupPickerUi));
    println!("{}", i18n.text(MessageKey::SetupInline));
    println!("{}", i18n.text(MessageKey::SetupFullscreen));
    print!("{}", i18n.text(MessageKey::SetupChoose));
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim() {
        "" | "1" | "inline" => Ok(PickerMode::Inline),
        "2" | "fullscreen" | "full-screen" | "full" => Ok(PickerMode::Fullscreen),
        value => Err(cli_error(format!(
            "unknown picker choice `{value}`; expected 1 or 2"
        ))),
    }
}

fn prompt_yes_no_cli(prompt: &str, default_yes: bool) -> CliResult<bool> {
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{prompt} {suffix}: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match input.trim().to_ascii_lowercase().as_str() {
        "" => Ok(default_yes),
        "y" | "yes" => Ok(true),
        "n" | "no" => Ok(false),
        value => Err(cli_error(format!(
            "unknown answer `{value}`; expected yes or no"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_rejects_arguments_for_now() {
        let error = setup_command(&["--unknown".to_string()]).unwrap_err();
        assert!(error.to_string().contains("setup does not accept"));
    }
}
