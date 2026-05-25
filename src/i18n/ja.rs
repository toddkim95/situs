use super::MessageKey;

pub(super) fn ja(key: MessageKey) -> &'static str {
    match key {
        MessageKey::KeymapTitle => "Situs キーマップ",
        MessageKey::KeymapPicker => "ピッカー",
        MessageKey::KeymapViews => "表示",
        MessageKey::KeymapHistory => "履歴",
        MessageKey::KeymapUpDown => "履歴の行を選択",
        MessageKey::KeymapPage => "行を大きく移動",
        MessageKey::KeymapLeftRight => "下部のクエリカーソルを移動",
        MessageKey::KeymapHomeEnd => "クエリの先頭または末尾に移動",
        MessageKey::KeymapTab => "選択したディレクトリにcdし、クエリをzshに残す",
        MessageKey::KeymapEnter => "選択したディレクトリにcdし、選択した履歴コマンドを実行",
        MessageKey::KeymapEsc => "終了して元のシェル入力を保持",
        MessageKey::KeymapHelp => "ヘルプの切り替え",
        MessageKey::KeymapFailed => "失敗した履歴の表示/非表示",
        MessageKey::KeymapInspect => "選択した履歴의 検査",
        MessageKey::KeymapSource => "ソースフィルタの切り替え: all, local, atuin",
        MessageKey::KeymapContext => "コンテキストフィルタの切り替え: all, directory, workspace",
        MessageKey::KeymapCopy => "選択したコマンドをコピー",
        MessageKey::KeymapDelete => "situsの履歴から選択した行を削除",
        MessageKey::SetupTitle => "Situs セットアップ",
        MessageKey::SetupPickerUi => "ピッカー UI:",
        MessageKey::SetupInline => "  1) inline      プロンプトの下に表示するコンパクトなピッカー",
        MessageKey::SetupFullscreen => "  2) fullscreen  代替画面のTUI",
        MessageKey::SetupChoose => "選択 [1]: ",
        MessageKey::SetupPickerModeSetPrefix => "ピッカーモードを設定しました:",
        MessageKey::SetupAtuinFound => {
            "Atuin の履歴が見つかりました。Atuin の自動同期を有効にしますか？"
        }
        MessageKey::SetupAtuinSetPrefix => "Atuin 自動同期を設定しました:",
        MessageKey::SetupZshrcHint => {
            "まだ追加されていない場合は、~/.zshrc に以下を追加してください:"
        }
        MessageKey::DoctorTitle => "Situs ドクター",
        MessageKey::DoctorHistoryPath => "履歴のパス",
        MessageKey::DoctorHistoryRecords => "履歴レコード",
        MessageKey::DoctorKeyBinding => "キーバインディング",
        MessageKey::DoctorMode => "モード",
        MessageKey::DoctorPickerMode => "ピッカーモード",
        MessageKey::DoctorAtuinSync => "atuin 同期",
        MessageKey::DoctorZshIntegration => "zsh 統合",
        MessageKey::DoctorAtuinDb => "atuin DB",
        MessageKey::DoctorConfigured => "設定済み",
        MessageKey::DoctorNotFound => "見つかりません",
        MessageKey::PickerSearch => "検索",
        MessageKey::PickerInspect => "詳細",
        MessageKey::PickerKeyboard => "キーボード",
        MessageKey::PickerLoadingHistory => "コマンド履歴を読み込んでいます",
        MessageKey::PickerNoHistory => "ディレクトリの履歴が見つかりません",
        MessageKey::PickerNoHistoryHint => {
            "正しいディレクトリで一度コマンドを実行してから、もう一度お試しください。"
        }
        MessageKey::PickerNoMatches => "現在のクエリに一致するディレクトリはありません。",
        MessageKey::PickerNoSelected => "検査する履歴アイテムが選択されていません。",
        MessageKey::PickerCandidateCommand => "コマンド",
        MessageKey::PickerCandidateDirectory => "ディレクトリ",
        MessageKey::PickerCandidateStatus => "ステータス",
        MessageKey::PickerCandidateWhen => "日時",
        MessageKey::PickerCandidateCompactHeader => "コマンド / ディレクトリ",
        MessageKey::PickerResultSingular => "件の結果",
        MessageKey::PickerResultPlural => "件の結果",
        MessageKey::PickerSuccessfulHistory => "成功のみ",
        MessageKey::PickerAllHistory => "すべての履歴",
        MessageKey::PickerHelpQuit => "終了",
        MessageKey::PickerHelpSelect => "選択",
        MessageKey::PickerHelpEdit => "編集",
        MessageKey::PickerHelpCd => "cd",
        MessageKey::PickerHelpRun => "実行",
        MessageKey::PickerHelpCopy => "コピー",
        MessageKey::PickerHelpDelete => "削除",
        MessageKey::PickerHelpSource => "ソース",
        MessageKey::PickerHelpContext => "コンテキスト",
        MessageKey::PickerHelpHelp => "ヘルプ",
        MessageKey::PickerHelpSelectPrevious => "前のコマンドを選択",
        MessageKey::PickerHelpEditQuery => "固定下部クエリを編集",
        MessageKey::PickerHelpCdKeepQuery => "選択したディレクトリにcdしてクエリを保持",
        MessageKey::PickerHelpRunSelected => "そのディレクトリで選択したコマンドを実行",
        MessageKey::PickerHelpCopyCommand => "選択したコマンドをコピー",
        MessageKey::PickerHelpDeleteRow => "選択したローカル履歴行を削除",
        MessageKey::PickerHelpCycleSource => "ソースフィルタの切り替え",
        MessageKey::PickerHelpCycleContext => "コンテキストフィルタの切り替え",
        MessageKey::PickerHelpShowHideFailed => "失敗したコマンドの表示/非表示",
        MessageKey::PickerInspectCommand => "コマンド",
        MessageKey::PickerInspectCwd => "cwd",
        MessageKey::PickerInspectStatus => "ステータス",
        MessageKey::PickerInspectSource => "ソース",
        MessageKey::PickerInspectRuns => "実行回数",
        MessageKey::PickerInspectWhen => "日時",
        MessageKey::PickerInspectEnter => "このディレクトリでコマンドを実行",
        MessageKey::PickerInspectTab => "ここにcdしてクエリを保持",
        MessageKey::PickerMessageCopied => "コマンドをコピーしました",
        MessageKey::PickerMessageCopyFailed => "コピーに失敗しました",
        MessageKey::PickerMessageNothingSelected => "選択されていません",
        MessageKey::PickerMessageDeletedRows => "履歴行",
        MessageKey::PickerMessageNothingDeleted => "削除されたものはありません",
        MessageKey::PickerMessageSource => "ソース",
        MessageKey::PickerMessageContext => "コンテキスト",
        MessageKey::PickerMessageShowingFailed => "失敗した履歴を表示中",
        MessageKey::PickerMessageHidingFailed => "失敗した履歴を非表示中",
        MessageKey::StatsTitle => "Situs 統計",
        MessageKey::StatsRecords => "レコード",
        MessageKey::StatsSuccessful => "成功",
        MessageKey::StatsFailed => "失敗",
        MessageKey::StatsLocal => "ローカル",
        MessageKey::StatsAtuin => "atuin",
        MessageKey::StatsTopCommands => "トップコマンド",
        MessageKey::StatsTopDirectories => "トップディレクトリ",
        MessageKey::StatsNone => "なし",
        MessageKey::SetupTuiTitle => "Situs CLI 設定 (TUI)",
        MessageKey::SetupTuiHelp => {
            "上/下: 移動 | 左/右/スペース/Enter: 切り替え | S: 保存 | Esc/Q: キャンセル"
        }
        MessageKey::SetupTuiPickerMode => "セレクター UI モード",
        MessageKey::SetupTuiAtuinSync => "Atuin 自動同期",
        MessageKey::SetupTuiLanguage => "表示言語",
        MessageKey::SetupTuiSaveBtn => "[ 設定を保存 ]",
        MessageKey::SetupTuiCancelBtn => "[ キャンセル ]",
        MessageKey::SetupTuiSavedMessage => "設定が正常に保存されました！",
    }
}

pub(super) const JA_HELP_TEXT: &str = "\
situs - 以前シェルコマンドが実行に成功したディレクトリを記憶します

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
  choose はディレクトリピッカーを開き、選択されたディレクトリでコマンドを実行します。
  --mode restore は zsh 統合が使用されている場合に元のシェルディレクトリに戻します。
  --include-failed は、成功したコマンド実行に加えて失敗したコマンド実行も表示します。
  --context directory は一致するものを現在のディレクトリに制限し、workspace は現在の git リポジトリに制限します。
  --print-dir はシェル統合用に選択されたディレクトリを出力します。
  --print-selection は選択されたディレクトリとコマンドを別々の行に出力します。
  --print-widget-selection はシェル統合用のアクション、ディレクトリ、コマンド、クエリを出力します。
  --print-widget-selection は TUI ピッカーを必要とし、シンプルなピッカーにはフォールバックしません。
  doctor はインストールと履歴の診断を出力します。
  keymap はピッカーのショートカットキーを出力します。
  stats は記憶されたコマンド、ディレクトリ、ソースの混在、および失敗を要約します。
  setup はピッカーモードとオプション of Atuin 自動同期動作を設定します。
  atuin enable は Atuin 自動同期の設定を situs の設定ファイルに保存します。
  import atuin は Atuin の SQLite 履歴を situs の履歴にインポートします。
  SITUS_PICKER=fullscreen を設定すると、設定済みのピッカーモードをオーバーライドします。
  SITUS_ATUIN_SYNC=auto を設定すると、設定済みの Atuin 同期モードをオーバーライドします。

Try:
  eval \"$(situs init zsh)\"
";
