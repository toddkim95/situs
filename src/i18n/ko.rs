use super::MessageKey;

pub(super) fn ko(key: MessageKey) -> &'static str {
    match key {
        MessageKey::KeymapTitle => "Situs 키맵",
        MessageKey::KeymapPicker => "선택기",
        MessageKey::KeymapViews => "보기",
        MessageKey::KeymapHistory => "히스토리",
        MessageKey::KeymapUpDown => "히스토리 행 선택",
        MessageKey::KeymapPage => "행 단위로 크게 이동",
        MessageKey::KeymapLeftRight => "하단 query 커서 이동",
        MessageKey::KeymapHomeEnd => "query 처음이나 끝으로 이동",
        MessageKey::KeymapTab => "선택한 디렉터리로 cd하고 query를 zsh에 남김",
        MessageKey::KeymapEnter => "선택한 디렉터리로 cd하고 선택한 히스토리 명령 실행",
        MessageKey::KeymapPut => "cd나 실행 없이 선택한 명령을 붙여넣기",
        MessageKey::KeymapEsc => "종료하고 원래 shell 입력 유지",
        MessageKey::KeymapHelp => "도움말 전환",
        MessageKey::KeymapFailed => "실패 히스토리 표시/숨김",
        MessageKey::KeymapInspect => "선택한 히스토리 자세히 보기",
        MessageKey::KeymapSource => "source 필터 순환: all, local, atuin",
        MessageKey::KeymapContext => "context 필터 순환: all, directory, workspace",
        MessageKey::KeymapCopy => "선택한 명령 복사",
        MessageKey::KeymapDelete => "situs 히스토리에서 선택 행 삭제",
        MessageKey::SetupTitle => "Situs 설정",
        MessageKey::SetupPickerUi => "선택기 UI:",
        MessageKey::SetupInline => "  1) inline      프롬프트 아래 compact 선택기",
        MessageKey::SetupFullscreen => "  2) fullscreen  alternate-screen TUI",
        MessageKey::SetupChoose => "선택 [1]: ",
        MessageKey::SetupPickerModeSetPrefix => "선택기 모드 설정",
        MessageKey::SetupAtuinFound => "Atuin 히스토리를 찾았습니다. Atuin 자동 동기화를 켤까요?",
        MessageKey::SetupAtuinImportFound => "Atuin 히스토리를 Situs로 가져올까요?",
        MessageKey::SetupAtuinSetPrefix => "Atuin 자동 동기화 설정",
        MessageKey::SetupZshrcHint => "~/.zshrc에 아직 없다면 이 줄을 추가하세요:",
        MessageKey::DoctorTitle => "Situs 진단",
        MessageKey::DoctorHistoryPath => "히스토리 경로",
        MessageKey::DoctorHistoryRecords => "히스토리 레코드",
        MessageKey::DoctorKeyBinding => "키 바인딩",
        MessageKey::DoctorMode => "모드",
        MessageKey::DoctorPickerMode => "선택기 모드",
        MessageKey::DoctorAtuinSync => "Atuin 동기화",
        MessageKey::DoctorZshIntegration => "zsh 연동",
        MessageKey::DoctorAtuinDb => "Atuin DB",
        MessageKey::DoctorConfigured => "설정됨",
        MessageKey::DoctorNotFound => "찾지 못함",
        MessageKey::PickerSearch => "검색",
        MessageKey::PickerInspect => "상세",
        MessageKey::PickerKeyboard => "키보드",
        MessageKey::PickerLoadingHistory => "명령 히스토리 로딩",
        MessageKey::PickerNoHistory => "디렉터리 히스토리가 없습니다",
        MessageKey::PickerNoHistoryHint => "올바른 디렉터리에서 한 번 실행한 뒤 다시 시도하세요.",
        MessageKey::PickerNoMatches => "현재 query와 맞는 디렉터리가 없습니다.",
        MessageKey::PickerNoSelected => "자세히 볼 히스토리가 선택되지 않았습니다.",
        MessageKey::PickerCandidateCommand => "명령",
        MessageKey::PickerCandidateDirectory => "디렉터리",
        MessageKey::PickerCandidateStatus => "상태",
        MessageKey::PickerCandidateWhen => "시간",
        MessageKey::PickerCandidateCompactHeader => "명령 / 디렉터리",
        MessageKey::PickerResultSingular => "결과",
        MessageKey::PickerResultPlural => "결과",
        MessageKey::PickerSuccessfulHistory => "성공만",
        MessageKey::PickerAllHistory => "전체 히스토리",
        MessageKey::PickerHelpQuit => "종료",
        MessageKey::PickerHelpSelect => "선택",
        MessageKey::PickerHelpEdit => "수정",
        MessageKey::PickerHelpCd => "cd",
        MessageKey::PickerHelpRun => "실행",
        MessageKey::PickerHelpCopy => "복사",
        MessageKey::PickerHelpDelete => "삭제",
        MessageKey::PickerHelpSource => "source",
        MessageKey::PickerHelpContext => "context",
        MessageKey::PickerHelpHelp => "도움말",
        MessageKey::PickerHelpSelectPrevious => "이전 명령 선택",
        MessageKey::PickerHelpEditQuery => "고정 하단 query 수정",
        MessageKey::PickerHelpCdKeepQuery => "선택한 디렉터리로 cd하고 query 유지",
        MessageKey::PickerHelpRunSelected => "그 디렉터리에서 선택 명령 실행",
        MessageKey::PickerHelpPasteCommand => "cd나 실행 없이 선택한 명령 붙여넣기",
        MessageKey::PickerHelpCopyCommand => "선택한 명령 복사",
        MessageKey::PickerHelpDeleteRow => "선택한 local 히스토리 행 삭제",
        MessageKey::PickerHelpCycleSource => "source 필터 순환",
        MessageKey::PickerHelpCycleContext => "context 필터 순환",
        MessageKey::PickerHelpShowHideFailed => "실패 명령 표시/숨김",
        MessageKey::PickerInspectCommand => "명령",
        MessageKey::PickerInspectCwd => "cwd",
        MessageKey::PickerInspectStatus => "상태",
        MessageKey::PickerInspectSource => "source",
        MessageKey::PickerInspectRuns => "실행",
        MessageKey::PickerInspectWhen => "언제",
        MessageKey::PickerInspectEnter => "이 디렉터리에서 명령 실행",
        MessageKey::PickerInspectTab => "여기로 cd하고 query 유지",
        MessageKey::PickerMessageCopied => "명령 복사됨",
        MessageKey::PickerMessageCopyFailed => "복사 실패",
        MessageKey::PickerMessageNothingSelected => "선택된 항목 없음",
        MessageKey::PickerMessageDeletedRows => "히스토리",
        MessageKey::PickerMessageNothingDeleted => "삭제된 항목 없음",
        MessageKey::PickerMessageSource => "소스",
        MessageKey::PickerMessageContext => "컨텍스트",
        MessageKey::PickerMessageShowingFailed => "실패 히스토리 표시 중",
        MessageKey::PickerMessageHidingFailed => "실패 히스토리 숨김",
        MessageKey::StatsTitle => "Situs 통계",
        MessageKey::StatsRecords => "레코드",
        MessageKey::StatsSuccessful => "성공",
        MessageKey::StatsFailed => "실패",
        MessageKey::StatsLocal => "local",
        MessageKey::StatsAtuin => "atuin",
        MessageKey::StatsTopCommands => "상위 명령",
        MessageKey::StatsTopDirectories => "상위 디렉터리",
        MessageKey::StatsNone => "없음",
        MessageKey::SetupTuiTitle => "Situs CLI 설정 (TUI)",
        MessageKey::SetupTuiHelp => {
            "위/아래: 이동 | 좌/우/Space/Enter: 값 변경 | S: 저장 | Esc/Q: 취소"
        }
        MessageKey::SetupTuiPickerMode => "선택기 UI 모드",
        MessageKey::SetupTuiAtuinSync => "Atuin 자동 동기화",
        MessageKey::SetupTuiLanguage => "표시 언어(Language)",
        MessageKey::SetupTuiSaveBtn => "[ 설정 저장 ]",
        MessageKey::SetupTuiCancelBtn => "[ 취소 ]",
        MessageKey::SetupTuiSavedMessage => "설정이 성공적으로 저장되었습니다!",
        MessageKey::SetupTuiWidgetKey => "단축키 바인딩",
        MessageKey::SetupTuiShellInit => "쉘 설정파일에 자동 등록",
        MessageKey::SetupTuiAtuinImport => "Atuin 히스토리 1회 가져오기",
    }
}

pub(super) const KO_HELP_TEXT: &str = "\
situs - shell 명령이 예전에 성공했던 디렉터리를 기억합니다

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
  choose는 디렉터리 선택기를 열고 선택한 디렉터리에서 명령을 실행합니다.
  --mode restore는 zsh 연동에서 원래 shell 디렉터리로 돌아옵니다.
  --include-failed는 성공 기록과 함께 실패한 명령 기록도 보여줍니다.
  --context directory는 현재 디렉터리로, workspace는 현재 git repo로 매치를 제한합니다.
  --print-dir는 shell 연동을 위해 선택한 디렉터리를 출력합니다.
  --print-selection은 선택한 디렉터리와 명령을 줄 단위로 출력합니다.
  --print-widget-selection은 action, directory, command, query를 출력합니다.
  --print-widget-selection은 TUI 선택기가 필요하며 plain 선택기로 fallback하지 않습니다.
  doctor는 설치와 히스토리 진단을 출력합니다.
  keymap은 선택기 단축키를 출력합니다.
  stats는 기억된 명령, 디렉터리, source, 실패 기록을 요약합니다.
  setup은 선택기 모드와 Atuin 자동 동기화를 설정합니다.
  atuin enable은 situs config에 Atuin 자동 동기화를 저장합니다.
  import atuin은 Atuin SQLite 히스토리를 situs 히스토리로 가져옵니다.
  SITUS_PICKER=fullscreen으로 설정된 선택기 모드를 덮어쓸 수 있습니다.
  SITUS_ATUIN_SYNC=auto로 설정된 Atuin 동기화 모드를 덮어쓸 수 있습니다.

Try:
  eval \"$(situs init zsh)\"
";
