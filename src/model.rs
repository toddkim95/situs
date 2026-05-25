#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Record {
    pub(crate) timestamp: u64,
    pub(crate) status: i32,
    pub(crate) cwd: String,
    pub(crate) command: String,
    pub(crate) source: HistorySource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Candidate {
    pub(crate) cwd: String,
    pub(crate) command: String,
    pub(crate) timestamp: u64,
    pub(crate) status: i32,
    pub(crate) source: CandidateSource,
    pub(crate) run_count: usize,
    pub(crate) success_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HistorySource {
    Local,
    Atuin,
}

impl HistorySource {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Atuin => "atuin",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CandidateSource {
    Local,
    Atuin,
    Mixed,
}

impl CandidateSource {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Atuin => "atuin",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceFilter {
    All,
    Local,
    Atuin,
}

impl SourceFilter {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Local => "local",
            Self::Atuin => "atuin",
        }
    }

    pub(crate) fn next(self) -> Self {
        match self {
            Self::All => Self::Local,
            Self::Local => Self::Atuin,
            Self::Atuin => Self::All,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ContextFilter {
    All,
    Directory,
    Workspace,
}

impl ContextFilter {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Directory => "directory",
            Self::Workspace => "workspace",
        }
    }

    pub(crate) fn next(self) -> Self {
        match self {
            Self::All => Self::Directory,
            Self::Directory => Self::Workspace,
            Self::Workspace => Self::All,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MatchFilters<'a> {
    pub(crate) source: SourceFilter,
    pub(crate) context: ContextFilter,
    pub(crate) current_dir: Option<&'a str>,
    pub(crate) workspace_root: Option<&'a str>,
}

impl Default for MatchFilters<'_> {
    fn default() -> Self {
        Self {
            source: SourceFilter::All,
            context: ContextFilter::All,
            current_dir: None,
            workspace_root: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MatchScope {
    pub(crate) words: usize,
    pub(crate) total_words: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionMode {
    Stay,
    Restore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PickerMode {
    Inline,
    Fullscreen,
}
