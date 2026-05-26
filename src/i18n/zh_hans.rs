use super::MessageKey;

pub(super) fn zh_hans(key: MessageKey) -> &'static str {
    match key {
        MessageKey::KeymapTitle => "Situs 键位",
        MessageKey::KeymapPicker => "选择器",
        MessageKey::KeymapViews => "视图",
        MessageKey::KeymapHistory => "历史",
        MessageKey::KeymapUpDown => "选择历史行",
        MessageKey::KeymapPage => "按页跳转",
        MessageKey::KeymapLeftRight => "移动底部 query 光标",
        MessageKey::KeymapHomeEnd => "跳到 query 开头或结尾",
        MessageKey::KeymapTab => "cd 到选中的目录，并在 zsh 保留 query",
        MessageKey::KeymapEnter => "cd 到选中的目录并运行选中的历史命令",
        MessageKey::KeymapPut => "不 cd、不运行，仅粘贴选中的命令",
        MessageKey::KeymapEsc => "退出并保留原始 shell 输入",
        MessageKey::KeymapHelp => "切换帮助",
        MessageKey::KeymapFailed => "显示或隐藏失败历史",
        MessageKey::KeymapInspect => "查看选中的历史",
        MessageKey::KeymapSource => "切换 source 过滤器: all, local, atuin",
        MessageKey::KeymapContext => "切换 context 过滤器: all, directory, workspace",
        MessageKey::KeymapCopy => "复制选中的命令",
        MessageKey::KeymapDelete => "从 situs 历史中删除选中行",
        MessageKey::SetupTitle => "Situs 设置",
        MessageKey::SetupPickerUi => "选择器 UI:",
        MessageKey::SetupInline => "  1) inline      在提示符下方显示 compact 选择器",
        MessageKey::SetupFullscreen => "  2) fullscreen  alternate-screen TUI",
        MessageKey::SetupChoose => "选择 [1]: ",
        MessageKey::SetupPickerModeSetPrefix => "选择器模式已设置为",
        MessageKey::SetupAtuinFound => "发现 Atuin 历史。启用 Atuin 自动同步吗?",
        MessageKey::SetupAtuinSetPrefix => "Atuin 自动同步已设置为",
        MessageKey::SetupZshrcHint => "如果 ~/.zshrc 里还没有, 请添加:",
        MessageKey::DoctorTitle => "Situs 诊断",
        MessageKey::DoctorHistoryPath => "历史路径",
        MessageKey::DoctorHistoryRecords => "历史记录",
        MessageKey::DoctorKeyBinding => "按键绑定",
        MessageKey::DoctorMode => "模式",
        MessageKey::DoctorPickerMode => "选择器模式",
        MessageKey::DoctorAtuinSync => "Atuin 同步",
        MessageKey::DoctorZshIntegration => "zsh 集成",
        MessageKey::DoctorAtuinDb => "Atuin DB",
        MessageKey::DoctorConfigured => "已配置",
        MessageKey::DoctorNotFound => "未找到",
        MessageKey::PickerSearch => "搜索",
        MessageKey::PickerInspect => "详情",
        MessageKey::PickerKeyboard => "键盘",
        MessageKey::PickerLoadingHistory => "正在加载命令历史",
        MessageKey::PickerNoHistory => "没有目录历史",
        MessageKey::PickerNoHistoryHint => "先在正确目录运行一次该命令, 然后再试。",
        MessageKey::PickerNoMatches => "没有目录匹配当前 query。",
        MessageKey::PickerNoSelected => "没有选中的历史可查看。",
        MessageKey::PickerCandidateCommand => "命令",
        MessageKey::PickerCandidateDirectory => "目录",
        MessageKey::PickerCandidateStatus => "状态",
        MessageKey::PickerCandidateWhen => "时间",
        MessageKey::PickerCandidateCompactHeader => "命令 / 目录",
        MessageKey::PickerResultSingular => "结果",
        MessageKey::PickerResultPlural => "结果",
        MessageKey::PickerSuccessfulHistory => "仅成功",
        MessageKey::PickerAllHistory => "全部历史",
        MessageKey::PickerHelpQuit => "退出",
        MessageKey::PickerHelpSelect => "选择",
        MessageKey::PickerHelpEdit => "编辑",
        MessageKey::PickerHelpCd => "cd",
        MessageKey::PickerHelpRun => "运行",
        MessageKey::PickerHelpCopy => "复制",
        MessageKey::PickerHelpDelete => "删除",
        MessageKey::PickerHelpSource => "source",
        MessageKey::PickerHelpContext => "context",
        MessageKey::PickerHelpHelp => "帮助",
        MessageKey::PickerHelpSelectPrevious => "选择上一条命令",
        MessageKey::PickerHelpEditQuery => "编辑固定底部 query",
        MessageKey::PickerHelpCdKeepQuery => "cd 到选中目录并保留 query",
        MessageKey::PickerHelpRunSelected => "在该目录运行选中的命令",
        MessageKey::PickerHelpPasteCommand => "不 cd、不运行，仅粘贴选中的命令",
        MessageKey::PickerHelpCopyCommand => "复制选中的命令",
        MessageKey::PickerHelpDeleteRow => "删除选中的 local 历史行",
        MessageKey::PickerHelpCycleSource => "切换 source 过滤器",
        MessageKey::PickerHelpCycleContext => "切换 context 过滤器",
        MessageKey::PickerHelpShowHideFailed => "显示或隐藏失败命令",
        MessageKey::PickerInspectCommand => "命令",
        MessageKey::PickerInspectCwd => "cwd",
        MessageKey::PickerInspectStatus => "状态",
        MessageKey::PickerInspectSource => "source",
        MessageKey::PickerInspectRuns => "运行",
        MessageKey::PickerInspectWhen => "时间",
        MessageKey::PickerInspectEnter => "在此目录运行命令",
        MessageKey::PickerInspectTab => "cd 到这里并保留 query",
        MessageKey::PickerMessageCopied => "命令已复制",
        MessageKey::PickerMessageCopyFailed => "复制失败",
        MessageKey::PickerMessageNothingSelected => "没有选中项",
        MessageKey::PickerMessageDeletedRows => "条历史",
        MessageKey::PickerMessageNothingDeleted => "没有删除项",
        MessageKey::PickerMessageSource => "来源",
        MessageKey::PickerMessageContext => "上下文",
        MessageKey::PickerMessageShowingFailed => "正在显示失败历史",
        MessageKey::PickerMessageHidingFailed => "正在隐藏失败历史",
        MessageKey::StatsTitle => "Situs 统计",
        MessageKey::StatsRecords => "记录",
        MessageKey::StatsSuccessful => "成功",
        MessageKey::StatsFailed => "失败",
        MessageKey::StatsLocal => "local",
        MessageKey::StatsAtuin => "atuin",
        MessageKey::StatsTopCommands => "热门命令",
        MessageKey::StatsTopDirectories => "热门目录",
        MessageKey::StatsNone => "无",
        MessageKey::SetupTuiTitle => "Situs CLI 设置 (TUI)",
        MessageKey::SetupTuiHelp => "上/下: 导航 | 左/右/空格/回车: 切换 | S: 保存 | Esc/Q: 取消",
        MessageKey::SetupTuiPickerMode => "选择器 UI 模式",
        MessageKey::SetupTuiAtuinSync => "Atuin 自动同步",
        MessageKey::SetupTuiLanguage => "显示语言",
        MessageKey::SetupTuiSaveBtn => "[ 保存设置 ]",
        MessageKey::SetupTuiCancelBtn => "[ 取消 ]",
        MessageKey::SetupTuiSavedMessage => "设置保存成功！",
    }
}

pub(super) const ZH_HANS_HELP_TEXT: &str = "\
situs - 记住 shell 命令曾经在哪个目录成功运行

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
  choose 会打开目录选择器, 然后在选中的目录运行命令。
  --mode restore 在 zsh 集成中会回到原来的 shell 目录。
  --include-failed 会同时显示失败 the 命令历史。
  --context directory 限制为当前目录, workspace 限制为当前 git repo。
  --print-dir 为 shell 集成输出选中的目录。
  --print-selection 分行输出选中的目录和命令。
  --print-widget-selection 输出 action, directory, command, query。
  --print-widget-selection 需要 TUI 选择器, 不会 fallback 到 plain 选择器。
  doctor 输出安装和历史诊断。
  keymap 输出选择器快捷键。
  stats 汇总已记住的命令、目录、source 和失败记录。
  setup 配置选择器模式和可选的 Atuin 自动同步。
  atuin enable 将 Atuin 自动同步保存到 situs config。
  import atuin 将 Atuin SQLite 历史导入 situs 历史。
  设置 SITUS_PICKER=fullscreen 可覆盖已配置的选择器模式。
  设置 SITUS_ATUIN_SYNC=auto 可覆盖已配置的 Atuin 同步模式。

Try:
  eval \"$(situs init zsh)\"
";
