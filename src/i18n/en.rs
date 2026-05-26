use super::MessageKey;

pub(super) fn en(key: MessageKey) -> &'static str {
    match key {
        MessageKey::KeymapTitle => "Situs Keymap",
        MessageKey::KeymapPicker => "Picker",
        MessageKey::KeymapViews => "Views",
        MessageKey::KeymapHistory => "History",
        MessageKey::KeymapUpDown => "select history rows",
        MessageKey::KeymapPage => "jump through rows",
        MessageKey::KeymapLeftRight => "move the fixed bottom query cursor",
        MessageKey::KeymapHomeEnd => "jump to query start or end",
        MessageKey::KeymapTab => "cd to selected directory and keep the query in zsh",
        MessageKey::KeymapEnter => "cd to selected directory and run the selected history command",
        MessageKey::KeymapPut => "paste selected command without cd or run",
        MessageKey::KeymapEsc => "quit and keep the original shell input",
        MessageKey::KeymapHelp => "toggle help",
        MessageKey::KeymapFailed => "show or hide failed history",
        MessageKey::KeymapInspect => "inspect selected history",
        MessageKey::KeymapSource => "cycle source filter: all, local, atuin",
        MessageKey::KeymapContext => "cycle context filter: all, directory, workspace",
        MessageKey::KeymapCopy => "copy selected command",
        MessageKey::KeymapDelete => "delete selected row from situs history",
        MessageKey::SetupTitle => "Situs setup",
        MessageKey::SetupPickerUi => "Picker UI:",
        MessageKey::SetupInline => "  1) inline      compact picker under your prompt",
        MessageKey::SetupFullscreen => "  2) fullscreen  alternate-screen TUI",
        MessageKey::SetupChoose => "Choose [1]: ",
        MessageKey::SetupPickerModeSetPrefix => "Picker mode set to",
        MessageKey::SetupAtuinFound => "Atuin history was found. Enable Atuin auto-sync?",
        MessageKey::SetupAtuinImportFound => "Import Atuin history into Situs now?",
        MessageKey::SetupAtuinSetPrefix => "Atuin auto-sync set to",
        MessageKey::SetupZshrcHint => "Add this to ~/.zshrc if it is not already there:",
        MessageKey::DoctorTitle => "Situs Doctor",
        MessageKey::DoctorHistoryPath => "history path",
        MessageKey::DoctorHistoryRecords => "history records",
        MessageKey::DoctorKeyBinding => "key binding",
        MessageKey::DoctorMode => "mode",
        MessageKey::DoctorPickerMode => "picker mode",
        MessageKey::DoctorAtuinSync => "atuin sync",
        MessageKey::DoctorZshIntegration => "zsh integration",
        MessageKey::DoctorAtuinDb => "atuin db",
        MessageKey::DoctorConfigured => "configured",
        MessageKey::DoctorNotFound => "not found",
        MessageKey::PickerSearch => "Search",
        MessageKey::PickerInspect => "Inspect",
        MessageKey::PickerKeyboard => "Keyboard",
        MessageKey::PickerLoadingHistory => "Loading command history",
        MessageKey::PickerNoHistory => "No directory history found",
        MessageKey::PickerNoHistoryHint => {
            "Run the command once in the right directory, then try again."
        }
        MessageKey::PickerNoMatches => "No directories match the current query.",
        MessageKey::PickerNoSelected => "No selected history item to inspect.",
        MessageKey::PickerCandidateCommand => "Command",
        MessageKey::PickerCandidateDirectory => "Directory",
        MessageKey::PickerCandidateStatus => "Status",
        MessageKey::PickerCandidateWhen => "When",
        MessageKey::PickerCandidateCompactHeader => "command / directory",
        MessageKey::PickerResultSingular => "result",
        MessageKey::PickerResultPlural => "results",
        MessageKey::PickerSuccessfulHistory => "successful",
        MessageKey::PickerAllHistory => "all history",
        MessageKey::PickerHelpQuit => "quit",
        MessageKey::PickerHelpSelect => "select",
        MessageKey::PickerHelpEdit => "edit",
        MessageKey::PickerHelpCd => "cd",
        MessageKey::PickerHelpRun => "run",
        MessageKey::PickerHelpCopy => "copy",
        MessageKey::PickerHelpDelete => "delete",
        MessageKey::PickerHelpSource => "source",
        MessageKey::PickerHelpContext => "context",
        MessageKey::PickerHelpHelp => "help",
        MessageKey::PickerHelpSelectPrevious => "select a previous command",
        MessageKey::PickerHelpEditQuery => "edit the fixed bottom query",
        MessageKey::PickerHelpCdKeepQuery => "cd to the selected directory and keep the query",
        MessageKey::PickerHelpRunSelected => "run the selected command in that directory",
        MessageKey::PickerHelpPasteCommand => "paste selected command without cd or run",
        MessageKey::PickerHelpCopyCommand => "copy selected command",
        MessageKey::PickerHelpDeleteRow => "delete selected local history row",
        MessageKey::PickerHelpCycleSource => "cycle source filter",
        MessageKey::PickerHelpCycleContext => "cycle context filter",
        MessageKey::PickerHelpShowHideFailed => "show or hide failed commands",
        MessageKey::PickerInspectCommand => "command",
        MessageKey::PickerInspectCwd => "cwd",
        MessageKey::PickerInspectStatus => "status",
        MessageKey::PickerInspectSource => "source",
        MessageKey::PickerInspectRuns => "runs",
        MessageKey::PickerInspectWhen => "when",
        MessageKey::PickerInspectEnter => "run command in this directory",
        MessageKey::PickerInspectTab => "cd here and keep the query",
        MessageKey::PickerMessageCopied => "copied command",
        MessageKey::PickerMessageCopyFailed => "copy failed",
        MessageKey::PickerMessageNothingSelected => "nothing selected",
        MessageKey::PickerMessageDeletedRows => "history rows",
        MessageKey::PickerMessageNothingDeleted => "nothing deleted",
        MessageKey::PickerMessageSource => "source",
        MessageKey::PickerMessageContext => "context",
        MessageKey::PickerMessageShowingFailed => "showing failed history",
        MessageKey::PickerMessageHidingFailed => "hiding failed history",
        MessageKey::StatsTitle => "Situs Stats",
        MessageKey::StatsRecords => "records",
        MessageKey::StatsSuccessful => "successful",
        MessageKey::StatsFailed => "failed",
        MessageKey::StatsLocal => "local",
        MessageKey::StatsAtuin => "atuin",
        MessageKey::StatsTopCommands => "Top Commands",
        MessageKey::StatsTopDirectories => "Top Directories",
        MessageKey::StatsNone => "none",
        MessageKey::SetupTuiTitle => "Situs CLI Setup (TUI)",
        MessageKey::SetupTuiHelp => {
            "Up/Down: Navigate | Left/Right/Space/Enter: Toggle | S: Save | Esc/Q: Cancel"
        }
        MessageKey::SetupTuiPickerMode => "Picker UI Mode",
        MessageKey::SetupTuiAtuinSync => "Atuin Auto-Sync",
        MessageKey::SetupTuiLanguage => "Display Language",
        MessageKey::SetupTuiSaveBtn => "[ Save Settings ]",
        MessageKey::SetupTuiCancelBtn => "[ Cancel ]",
        MessageKey::SetupTuiSavedMessage => "Settings successfully saved!",
        MessageKey::SetupTuiWidgetKey => "Widget Shortcut Key",
        MessageKey::SetupTuiShellInit => "Auto-add to Shell Profile",
        MessageKey::SetupTuiAtuinImport => "One-time Atuin Import",
    }
}

pub(super) const EN_HELP_TEXT: &str = "\
situs - remember where shell commands worked before

Usage:
  situs setup
  situs init zsh
  situs doctor
  situs keymap
  situs atuin enable|disable|status
  situs import atuin [--db <path>]
  situs record --cwd <dir> --status <code> -- <command>
  situs choose [--mode stay|restore] [--picker inline|fullscreen] [--context all|directory|workspace] --command <command>
  situs choose --print-dir --command <command>
  situs choose --print-selection --command <command>
  situs choose --print-widget-selection --command <command>
  situs run -- <command>
  situs stats

Notes:
  choose opens the directory picker, then runs the command in the selected directory.
  --mode restore returns to the original shell directory when used by zsh integration.
  --include-failed shows failed command runs in addition to successful ones.
  --context directory limits matches to the current directory; workspace limits to the current git repo.
  --print-dir prints the selected directory for shell integrations.
  --print-selection prints the selected directory and command on separate lines.
  --print-widget-selection prints action, directory, command, and query for shell integrations.
  --print-widget-selection requires a TUI picker and never falls back to the plain picker.
  doctor prints installation and history diagnostics.
  keymap prints the picker shortcuts.
  stats summarizes remembered commands, directories, source mix, and failures.
  setup configures picker mode and optional Atuin auto-sync.
  atuin enable stores Atuin auto-sync in situs's config file.
  import atuin reads Atuin's SQLite history into situs's history.
  Set SITUS_PICKER=fullscreen to override the configured picker mode.
  Set SITUS_ATUIN_SYNC=auto to override the configured Atuin sync mode.

Try:
  eval \"$(situs init zsh)\"
";
