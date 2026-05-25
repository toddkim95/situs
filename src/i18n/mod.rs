use std::env;

mod en;
mod es;
mod ja;
mod ko;
mod zh_hans;

use en::{en, EN_HELP_TEXT};
use es::{es, ES_HELP_TEXT};
use ja::{ja, JA_HELP_TEXT};
use ko::{ko, KO_HELP_TEXT};
use zh_hans::{zh_hans, ZH_HANS_HELP_TEXT};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Locale {
    En,
    Ko,
    ZhHans,
    Es,
    Ja,
}

impl Locale {
    const ENV_ORDER: &'static [&'static str] = &["SITUS_LANG", "LC_ALL", "LC_MESSAGES", "LANG"];

    pub(crate) fn from_env() -> Self {
        for name in Self::ENV_ORDER {
            let Ok(value) = env::var(name) else {
                continue;
            };
            if let Some(locale) = Self::parse(&value) {
                return locale;
            }
        }

        if let Ok(Some(value)) = crate::config::read_configured_language() {
            if let Some(locale) = Self::parse(&value) {
                return locale;
            }
        }

        Self::En
    }

    #[cfg(test)]
    pub(crate) fn from_env_pairs<I, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (&'static str, V)>,
        V: AsRef<str>,
    {
        let mut values = Vec::new();
        for (name, value) in pairs {
            if Self::ENV_ORDER.contains(&name) {
                values.push((name, value.as_ref().to_string()));
            }
        }

        for name in Self::ENV_ORDER {
            if let Some((_, value)) = values.iter().find(|(candidate, _)| candidate == name) {
                if let Some(locale) = Self::parse(value) {
                    return locale;
                }
            }
        }

        Self::En
    }

    pub(crate) fn parse(value: &str) -> Option<Self> {
        let normalized = normalize_locale(value);
        if normalized.is_empty() {
            return None;
        }

        match normalized.as_str() {
            "c" | "posix" | "en" => Some(Self::En),
            value if value.starts_with("en-") => Some(Self::En),
            "ko" => Some(Self::Ko),
            value if value.starts_with("ko-") => Some(Self::Ko),
            "zh" | "zh-cn" | "zh-sg" | "zh-hans" | "zh-hans-cn" | "zh-hans-sg" => {
                Some(Self::ZhHans)
            }
            "es" => Some(Self::Es),
            value if value.starts_with("es-") => Some(Self::Es),
            "ja" => Some(Self::Ja),
            value if value.starts_with("ja-") => Some(Self::Ja),
            _ => None,
        }
    }
}

fn normalize_locale(value: &str) -> String {
    let value = value.trim();
    let value = value.split('.').next().unwrap_or(value);
    let value = value.split('@').next().unwrap_or(value);
    value.replace('_', "-").to_ascii_lowercase()
}

macro_rules! message_keys {
    ($($key:ident),+ $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        #[allow(dead_code)]
        pub(crate) enum MessageKey {
            $($key),+
        }

        impl MessageKey {
            #[cfg(test)]
            pub(crate) const ALL: &'static [MessageKey] = &[
                $(MessageKey::$key),+
            ];
        }
    };
}

message_keys! {
    KeymapTitle,
    KeymapPicker,
    KeymapViews,
    KeymapHistory,
    KeymapUpDown,
    KeymapPage,
    KeymapLeftRight,
    KeymapHomeEnd,
    KeymapTab,
    KeymapEnter,
    KeymapEsc,
    KeymapHelp,
    KeymapFailed,
    KeymapInspect,
    KeymapSource,
    KeymapContext,
    KeymapCopy,
    KeymapDelete,
    SetupTitle,
    SetupPickerUi,
    SetupInline,
    SetupFullscreen,
    SetupChoose,
    SetupPickerModeSetPrefix,
    SetupAtuinFound,
    SetupAtuinSetPrefix,
    SetupZshrcHint,
    DoctorTitle,
    DoctorHistoryPath,
    DoctorHistoryRecords,
    DoctorKeyBinding,
    DoctorMode,
    DoctorPickerMode,
    DoctorAtuinSync,
    DoctorZshIntegration,
    DoctorAtuinDb,
    DoctorConfigured,
    DoctorNotFound,
    PickerSearch,
    PickerInspect,
    PickerKeyboard,
    PickerLoadingHistory,
    PickerNoHistory,
    PickerNoHistoryHint,
    PickerNoMatches,
    PickerNoSelected,
    PickerCandidateCommand,
    PickerCandidateDirectory,
    PickerCandidateStatus,
    PickerCandidateWhen,
    PickerCandidateCompactHeader,
    PickerResultSingular,
    PickerResultPlural,
    PickerSuccessfulHistory,
    PickerAllHistory,
    PickerHelpQuit,
    PickerHelpSelect,
    PickerHelpEdit,
    PickerHelpCd,
    PickerHelpRun,
    PickerHelpCopy,
    PickerHelpDelete,
    PickerHelpSource,
    PickerHelpContext,
    PickerHelpHelp,
    PickerHelpSelectPrevious,
    PickerHelpEditQuery,
    PickerHelpCdKeepQuery,
    PickerHelpRunSelected,
    PickerHelpCopyCommand,
    PickerHelpDeleteRow,
    PickerHelpCycleSource,
    PickerHelpCycleContext,
    PickerHelpShowHideFailed,
    PickerInspectCommand,
    PickerInspectCwd,
    PickerInspectStatus,
    PickerInspectSource,
    PickerInspectRuns,
    PickerInspectWhen,
    PickerInspectEnter,
    PickerInspectTab,
    PickerMessageCopied,
    PickerMessageCopyFailed,
    PickerMessageNothingSelected,
    PickerMessageDeletedRows,
    PickerMessageNothingDeleted,
    PickerMessageSource,
    PickerMessageContext,
    PickerMessageShowingFailed,
    PickerMessageHidingFailed,
    StatsTitle,
    StatsRecords,
    StatsSuccessful,
    StatsFailed,
    StatsLocal,
    StatsAtuin,
    StatsTopCommands,
    StatsTopDirectories,
    StatsNone,
    SetupTuiTitle,
    SetupTuiHelp,
    SetupTuiPickerMode,
    SetupTuiAtuinSync,
    SetupTuiLanguage,
    SetupTuiSaveBtn,
    SetupTuiCancelBtn,
    SetupTuiSavedMessage,
    SetupTuiWidgetKey,
    SetupTuiShellInit,
    SetupTuiAtuinImport,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct I18n {
    locale: Locale,
}

impl I18n {
    pub(crate) fn from_env() -> Self {
        Self::new(Locale::from_env())
    }

    #[cfg(test)]
    pub(crate) fn english() -> Self {
        Self::new(Locale::En)
    }

    pub(crate) fn new(locale: Locale) -> Self {
        Self { locale }
    }

    pub(crate) fn text(self, key: MessageKey) -> &'static str {
        match self.locale {
            Locale::En => en(key),
            Locale::Ko => ko(key),
            Locale::ZhHans => zh_hans(key),
            Locale::Es => es(key),
            Locale::Ja => ja(key),
        }
    }

    pub(crate) fn keymap_text(self) -> String {
        format!(
            "\
{title}

{picker}
  Up / Down      {up_down}
  PageUp/Down    {page}
  Left / Right   {left_right}
  Home / End     {home_end}
  Tab            {tab}
  Enter          {enter}
  Esc / Ctrl-C   {esc}

{views}
  Ctrl-/ / F1    {help}
  Ctrl-F         {failed}
  Ctrl-O         {inspect}
  F2             {source}
  F3             {context}

{history}
  Ctrl-Y         {copy}
  Ctrl-D         {delete}
",
            title = self.text(MessageKey::KeymapTitle),
            picker = self.text(MessageKey::KeymapPicker),
            up_down = self.text(MessageKey::KeymapUpDown),
            page = self.text(MessageKey::KeymapPage),
            left_right = self.text(MessageKey::KeymapLeftRight),
            home_end = self.text(MessageKey::KeymapHomeEnd),
            tab = self.text(MessageKey::KeymapTab),
            enter = self.text(MessageKey::KeymapEnter),
            esc = self.text(MessageKey::KeymapEsc),
            views = self.text(MessageKey::KeymapViews),
            help = self.text(MessageKey::KeymapHelp),
            failed = self.text(MessageKey::KeymapFailed),
            inspect = self.text(MessageKey::KeymapInspect),
            source = self.text(MessageKey::KeymapSource),
            context = self.text(MessageKey::KeymapContext),
            history = self.text(MessageKey::KeymapHistory),
            copy = self.text(MessageKey::KeymapCopy),
            delete = self.text(MessageKey::KeymapDelete),
        )
    }

    pub(crate) fn help_text(self) -> &'static str {
        match self.locale {
            Locale::En => EN_HELP_TEXT,
            Locale::Ko => KO_HELP_TEXT,
            Locale::ZhHans => ZH_HANS_HELP_TEXT,
            Locale::Es => ES_HELP_TEXT,
            Locale::Ja => JA_HELP_TEXT,
        }
    }

    pub(crate) fn picker_runs(self, total: usize, successful: usize) -> String {
        match self.locale {
            Locale::En => format!("{total} total, {successful} successful"),
            Locale::Ko => format!("전체 {total}, 성공 {successful}"),
            Locale::ZhHans => format!("共 {total} 次, 成功 {successful} 次"),
            Locale::Es => format!("{total} en total, {successful} exitosas"),
            Locale::Ja => format!("計 {total} 回, 成功 {successful} 回"),
        }
    }

    pub(crate) fn picker_copy_failed(self, error: &str) -> String {
        format!(
            "{}: {error}",
            self.text(MessageKey::PickerMessageCopyFailed)
        )
    }

    pub(crate) fn picker_deleted_rows(self, count: usize) -> String {
        match self.locale {
            Locale::En => format!(
                "deleted {count} {}",
                self.text(MessageKey::PickerMessageDeletedRows)
            ),
            Locale::Ko => format!(
                "{} {count}개 삭제",
                self.text(MessageKey::PickerMessageDeletedRows)
            ),
            Locale::ZhHans => format!(
                "已删除 {count} {}",
                self.text(MessageKey::PickerMessageDeletedRows)
            ),
            Locale::Es => format!(
                "eliminadas {count} {}",
                self.text(MessageKey::PickerMessageDeletedRows)
            ),
            Locale::Ja => format!(
                "{count}行の{}を削除しました",
                self.text(MessageKey::PickerMessageDeletedRows)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_resolution_uses_expected_priority() {
        let locale = Locale::from_env_pairs([
            ("LANG", "ko_KR.UTF-8"),
            ("LC_MESSAGES", "zh_Hans_CN.UTF-8"),
            ("SITUS_LANG", "en_US.UTF-8"),
        ]);

        assert_eq!(locale, Locale::En);
    }

    #[test]
    fn locale_resolution_understands_supported_aliases() {
        assert_eq!(Locale::parse("C"), Some(Locale::En));
        assert_eq!(Locale::parse("POSIX"), Some(Locale::En));
        assert_eq!(Locale::parse("ko_KR.UTF-8"), Some(Locale::Ko));
        assert_eq!(Locale::parse("zh_CN.UTF-8"), Some(Locale::ZhHans));
        assert_eq!(Locale::parse("zh-Hans"), Some(Locale::ZhHans));
        assert_eq!(Locale::parse("es_ES.UTF-8"), Some(Locale::Es));
        assert_eq!(Locale::parse("es"), Some(Locale::Es));
        assert_eq!(Locale::parse("ja_JP.UTF-8"), Some(Locale::Ja));
        assert_eq!(Locale::parse("ja"), Some(Locale::Ja));
    }

    #[test]
    fn unsupported_locale_falls_back_to_english() {
        let locale = Locale::from_env_pairs([("SITUS_LANG", "zh_TW.UTF-8")]);

        assert_eq!(locale, Locale::En);
    }

    #[test]
    fn every_message_key_has_values_for_all_supported_locales() {
        for locale in [
            Locale::En,
            Locale::Ko,
            Locale::ZhHans,
            Locale::Es,
            Locale::Ja,
        ] {
            let i18n = I18n::new(locale);
            for key in MessageKey::ALL {
                assert!(
                    !i18n.text(*key).is_empty(),
                    "missing {key:?} for {locale:?}"
                );
            }
        }
    }

    #[test]
    fn protocol_words_remain_untranslated_in_help() {
        let ko = I18n::new(Locale::Ko).help_text();

        assert!(ko.contains("--print-widget-selection"));
        assert!(ko.contains("action, directory, command, query"));
        assert!(ko.contains("SITUS_PICKER=fullscreen"));
    }
}
