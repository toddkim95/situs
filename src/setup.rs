use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::tty::IsTty;
use crossterm::{execute, queue};

use crate::atuin::AtuinSyncMode;
use crate::config::{picker_mode_name, read_configured_picker_mode};
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
const PICKER_MODES: &[PickerMode] = &[PickerMode::Inline, PickerMode::Fullscreen];
const WIDGET_KEYS: &[&str] = &["^G", "^R", "^T", "None"];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ActiveSettingRow {
    PickerMode,
    Language,
    WidgetKey,
    ShellInit,
    AtuinImport,
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

fn shell_profile_path(shell: &str) -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    match shell {
        "zsh" => Some(PathBuf::from(&home).join(".zshrc")),
        "bash" => {
            let bash_profile = PathBuf::from(&home).join(".bash_profile");
            if bash_profile.exists() {
                Some(bash_profile)
            } else {
                Some(PathBuf::from(&home).join(".bashrc"))
            }
        }
        "fish" => Some(
            PathBuf::from(&home)
                .join(".config")
                .join("fish")
                .join("config.fish"),
        ),
        _ => None,
    }
}

fn profile_contains_situs(path: &Path, shell: &str) -> bool {
    let needle = match shell {
        "zsh" => "situs init zsh",
        "bash" => "situs init bash",
        "fish" => "situs init fish",
        _ => "situs init",
    };
    std::fs::read_to_string(path)
        .map(|contents| contents.contains(needle))
        .unwrap_or(false)
}

fn append_to_profile(path: &Path, shell: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let line = match shell {
        "zsh" => "\n# Situs integration\neval \"$(situs init zsh)\"\n",
        "bash" => "\n# Situs integration\neval \"$(situs init bash)\"\n",
        "fish" => "\n# Situs integration\nsitus init fish | source\n",
        _ => "",
    };
    if !line.is_empty() {
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

pub(crate) fn setup_command(args: &[String]) -> CliResult<i32> {
    if !args.is_empty() {
        return Err(cli_error("setup does not accept arguments yet"));
    }

    if !io::stdout().is_tty() || !io::stdin().is_tty() {
        return setup_cli_fallback();
    }

    let shell_path = std::env::var("SHELL").unwrap_or_default();
    let shell_name = if shell_path.contains("zsh") {
        "zsh"
    } else if shell_path.contains("bash") {
        "bash"
    } else if shell_path.contains("fish") {
        "fish"
    } else {
        "zsh"
    };

    let profile_path = shell_profile_path(shell_name);
    let already_configured = profile_path
        .as_ref()
        .map(|p| profile_contains_situs(p, shell_name))
        .unwrap_or(false);

    // Existing settings
    let initial_picker_mode = read_configured_picker_mode()?.unwrap_or(PickerMode::Inline);
    let initial_bindkey =
        crate::config::read_configured_bindkey()?.unwrap_or_else(|| "^G".to_string());
    let initial_locale = Locale::from_env();

    let mut picker_mode = initial_picker_mode;
    let mut bindkey = initial_bindkey;
    let mut shell_init = if already_configured {
        "Skip"
    } else {
        "Auto-add"
    };
    let mut atuin_import = "Skip";
    let mut current_locale = initial_locale;
    let mut active_row = ActiveSettingRow::PickerMode;

    let atuin_db_path = crate::atuin::default_atuin_db_path();
    let atuin_db_found = atuin_db_path.as_ref().map(|p| p.exists()).unwrap_or(false);

    let _guard =
        RawModeGuard::new().map_err(|e| cli_error(format!("failed to start TUI raw mode: {e}")))?;
    let mut stdout = io::stdout();

    loop {
        let i18n = I18n::new(current_locale);
        draw_setup(
            &mut stdout,
            i18n,
            picker_mode,
            &bindkey,
            shell_init,
            atuin_import,
            atuin_db_found,
            current_locale,
            active_row,
        )?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => active_row = prev_row(active_row, atuin_db_found),
                    KeyCode::Down => active_row = next_row(active_row, atuin_db_found),
                    KeyCode::Left => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, false);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, false);
                        }
                        ActiveSettingRow::WidgetKey => {
                            bindkey = cycle_widget_key(&bindkey, false);
                        }
                        ActiveSettingRow::ShellInit => {
                            shell_init = cycle_shell_init(shell_init);
                        }
                        ActiveSettingRow::AtuinImport => {
                            atuin_import = cycle_atuin_import(atuin_import);
                        }
                        _ => {}
                    },
                    KeyCode::Right => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, true);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, true);
                        }
                        ActiveSettingRow::WidgetKey => {
                            bindkey = cycle_widget_key(&bindkey, true);
                        }
                        ActiveSettingRow::ShellInit => {
                            shell_init = cycle_shell_init(shell_init);
                        }
                        ActiveSettingRow::AtuinImport => {
                            atuin_import = cycle_atuin_import(atuin_import);
                        }
                        _ => {}
                    },
                    KeyCode::Char(' ') | KeyCode::Enter => match active_row {
                        ActiveSettingRow::PickerMode => {
                            picker_mode = cycle_picker_mode(picker_mode, true);
                        }
                        ActiveSettingRow::Language => {
                            current_locale = cycle_language(current_locale, true);
                        }
                        ActiveSettingRow::WidgetKey => {
                            bindkey = cycle_widget_key(&bindkey, true);
                        }
                        ActiveSettingRow::ShellInit => {
                            shell_init = cycle_shell_init(shell_init);
                        }
                        ActiveSettingRow::AtuinImport => {
                            atuin_import = cycle_atuin_import(atuin_import);
                        }
                        ActiveSettingRow::Save => {
                            drop(_guard);
                            save_settings(
                                picker_mode,
                                current_locale,
                                &bindkey,
                                shell_init,
                                atuin_import,
                                shell_name,
                                profile_path,
                                i18n,
                            )?;
                            return Ok(0);
                        }
                        ActiveSettingRow::Cancel => {
                            return Ok(0);
                        }
                    },
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        drop(_guard);
                        save_settings(
                            picker_mode,
                            current_locale,
                            &bindkey,
                            shell_init,
                            atuin_import,
                            shell_name,
                            profile_path,
                            i18n,
                        )?;
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

fn next_row(row: ActiveSettingRow, atuin_db_found: bool) -> ActiveSettingRow {
    match row {
        ActiveSettingRow::PickerMode => ActiveSettingRow::Language,
        ActiveSettingRow::Language => ActiveSettingRow::WidgetKey,
        ActiveSettingRow::WidgetKey => ActiveSettingRow::ShellInit,
        ActiveSettingRow::ShellInit => {
            if atuin_db_found {
                ActiveSettingRow::AtuinImport
            } else {
                ActiveSettingRow::Save
            }
        }
        ActiveSettingRow::AtuinImport => ActiveSettingRow::Save,
        ActiveSettingRow::Save => ActiveSettingRow::Cancel,
        ActiveSettingRow::Cancel => ActiveSettingRow::PickerMode,
    }
}

fn prev_row(row: ActiveSettingRow, atuin_db_found: bool) -> ActiveSettingRow {
    match row {
        ActiveSettingRow::PickerMode => ActiveSettingRow::Cancel,
        ActiveSettingRow::Language => ActiveSettingRow::PickerMode,
        ActiveSettingRow::WidgetKey => ActiveSettingRow::Language,
        ActiveSettingRow::ShellInit => ActiveSettingRow::WidgetKey,
        ActiveSettingRow::AtuinImport => ActiveSettingRow::ShellInit,
        ActiveSettingRow::Save => {
            if atuin_db_found {
                ActiveSettingRow::AtuinImport
            } else {
                ActiveSettingRow::ShellInit
            }
        }
        ActiveSettingRow::Cancel => ActiveSettingRow::Save,
    }
}

fn cycle_picker_mode(current: PickerMode, _forward: bool) -> PickerMode {
    match current {
        PickerMode::Inline => PickerMode::Fullscreen,
        PickerMode::Fullscreen => PickerMode::Inline,
    }
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

fn cycle_widget_key(current: &str, forward: bool) -> String {
    let idx = WIDGET_KEYS.iter().position(|k| *k == current).unwrap_or(0);
    let next_idx = if forward {
        (idx + 1) % WIDGET_KEYS.len()
    } else {
        (idx + WIDGET_KEYS.len() - 1) % WIDGET_KEYS.len()
    };
    WIDGET_KEYS[next_idx].to_string()
}

fn cycle_shell_init(current: &str) -> &'static str {
    if current == "Auto-add" {
        "Skip"
    } else {
        "Auto-add"
    }
}

fn cycle_atuin_import(current: &str) -> &'static str {
    if current == "Run once" {
        "Skip"
    } else {
        "Run once"
    }
}

#[allow(clippy::too_many_arguments)]
fn save_settings(
    picker_mode: PickerMode,
    language: Locale,
    bindkey: &str,
    shell_init: &str,
    atuin_import: &str,
    shell_name: &str,
    profile_path: Option<PathBuf>,
    i18n: I18n,
) -> CliResult<()> {
    let _ = crate::config::write_configured_picker_mode(picker_mode)?;
    let _ = crate::config::write_configured_bindkey(bindkey)?;
    let lang_str = match language {
        Locale::En => "en",
        Locale::Ko => "ko",
        Locale::ZhHans => "zh-hans",
        Locale::Es => "es",
        Locale::Ja => "ja",
    };
    let _ = crate::config::write_configured_language(lang_str)?;
    let _ = crate::config::write_configured_atuin_sync_mode(AtuinSyncMode::Off)?;

    if shell_init == "Auto-add" {
        if let Some(ref path) = profile_path {
            if !profile_contains_situs(path, shell_name) {
                append_to_profile(path, shell_name).map_err(|e| {
                    cli_error(format!(
                        "Failed to write to profile ({}): {}",
                        path.display(),
                        e
                    ))
                })?;
                println!(
                    "Successfully added situs init command to {}.",
                    path.display()
                );
            }
        }
    }

    if atuin_import == "Run once" {
        if let Some(db_path) = crate::atuin::default_atuin_db_path() {
            if db_path.exists() {
                let h_path = crate::history::history_path()?;
                println!(
                    "Running one-time Atuin import from {}...",
                    db_path.display()
                );
                match crate::atuin::import_atuin_db(&db_path, &h_path) {
                    Ok(summary) => {
                        println!(
                            "Import finished: scanned {}, imported {}, skipped {}.",
                            summary.scanned, summary.imported, summary.skipped_existing
                        );
                    }
                    Err(e) => {
                        println!("Atuin import failed: {}", e);
                    }
                }
            }
        }
    }

    println!("{}", i18n.text(MessageKey::SetupTuiSavedMessage));
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn draw_setup(
    stdout: &mut io::Stdout,
    i18n: I18n,
    picker_mode: PickerMode,
    bindkey: &str,
    shell_init: &str,
    atuin_import: &str,
    atuin_db_found: bool,
    language: Locale,
    active_row: ActiveSettingRow,
) -> io::Result<()> {
    let (width, height) = terminal::size()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let title = i18n.text(MessageKey::SetupTuiTitle);
    let help = i18n.text(MessageKey::SetupTuiHelp);

    let start_y = (height / 2).saturating_sub(8);
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
    let option_align_x = content_start_x + 30;

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

    // 2. Language
    let lang_label = format!("{}: ", i18n.text(MessageKey::SetupTuiLanguage));
    let lang_y = start_y + 7;
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

    // 3. Widget Key
    let key_label = format!("{}: ", i18n.text(MessageKey::SetupTuiWidgetKey));
    let key_y = start_y + 9;
    queue!(stdout, crossterm::cursor::MoveTo(content_start_x, key_y))?;
    if active_row == ActiveSettingRow::WidgetKey {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(&key_label))?;

    queue!(stdout, crossterm::cursor::MoveTo(option_align_x, key_y))?;
    for k in WIDGET_KEYS {
        let is_selected = *k == bindkey;
        let prefix = if is_selected { "● " } else { "○ " };
        if is_selected {
            queue!(stdout, SetForegroundColor(Color::Green))?;
        }
        queue!(stdout, Print(prefix), Print(*k), ResetColor, Print("  "))?;
    }

    // 4. Shell Init
    let init_label = format!("{}: ", i18n.text(MessageKey::SetupTuiShellInit));
    let init_y = start_y + 11;
    queue!(stdout, crossterm::cursor::MoveTo(content_start_x, init_y))?;
    if active_row == ActiveSettingRow::ShellInit {
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print("▶ "),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print("  "))?;
    }
    queue!(stdout, Print(&init_label))?;

    queue!(stdout, crossterm::cursor::MoveTo(option_align_x, init_y))?;
    for opt in &["Auto-add", "Skip"] {
        let is_selected = *opt == shell_init;
        let prefix = if is_selected { "● " } else { "○ " };
        if is_selected {
            queue!(stdout, SetForegroundColor(Color::Green))?;
        }
        queue!(stdout, Print(prefix), Print(*opt), ResetColor, Print("  "))?;
    }

    // 5. Atuin Import (conditional)
    let next_y_offset = if atuin_db_found {
        let import_label = format!("{}: ", i18n.text(MessageKey::SetupTuiAtuinImport));
        let import_y = start_y + 13;
        queue!(stdout, crossterm::cursor::MoveTo(content_start_x, import_y))?;
        if active_row == ActiveSettingRow::AtuinImport {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("▶ "),
                ResetColor
            )?;
        } else {
            queue!(stdout, Print("  "))?;
        }
        queue!(stdout, Print(&import_label))?;

        queue!(stdout, crossterm::cursor::MoveTo(option_align_x, import_y))?;
        for opt in &["Run once", "Skip"] {
            let is_selected = *opt == atuin_import;
            let prefix = if is_selected { "● " } else { "○ " };
            if is_selected {
                queue!(stdout, SetForegroundColor(Color::Green))?;
            }
            queue!(stdout, Print(prefix), Print(*opt), ResetColor, Print("  "))?;
        }
        15
    } else {
        13
    };

    // 6. Save Button
    let save_btn = i18n.text(MessageKey::SetupTuiSaveBtn);
    let save_y = start_y + next_y_offset;
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

    // 7. Cancel Button
    let cancel_btn = i18n.text(MessageKey::SetupTuiCancelBtn);
    let cancel_y = start_y + next_y_offset + 1;
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

    // Save default bindkey and set sync mode to Off
    let _ = crate::config::write_configured_bindkey("^G")?;
    let _ = crate::config::write_configured_atuin_sync_mode(AtuinSyncMode::Off)?;

    if crate::atuin::default_atuin_db_path()
        .as_deref()
        .map(|path| path.exists())
        .unwrap_or(false)
        && prompt_yes_no_cli(i18n.text(MessageKey::SetupAtuinFound), true)?
    {
        // One-time import for CLI fallback
        if let Some(db_path) = crate::atuin::default_atuin_db_path() {
            let h_path = crate::history::history_path()?;
            println!(
                "Running one-time Atuin import from {}...",
                db_path.display()
            );
            let summary = crate::atuin::import_atuin_db(&db_path, &h_path)?;
            println!(
                "Import finished: scanned {}, imported {}, skipped {}.",
                summary.scanned, summary.imported, summary.skipped_existing
            );
        }
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
