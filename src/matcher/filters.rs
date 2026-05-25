use std::collections::HashMap;

use crate::context::{components_are_same_or_under, normalized_path_components};
use crate::model::{ContextFilter, HistorySource, MatchFilters, Record, SourceFilter};

pub(super) fn source_matches(source: HistorySource, filter: SourceFilter) -> bool {
    match filter {
        SourceFilter::All => true,
        SourceFilter::Local => source == HistorySource::Local,
        SourceFilter::Atuin => source == HistorySource::Atuin,
    }
}

pub(super) struct PreparedFilters {
    pub(super) source: SourceFilter,
    context: PreparedContextFilter,
}

enum PreparedContextFilter {
    All,
    Directory(Vec<String>),
    Workspace(Vec<String>),
    MissingContext,
}

impl PreparedFilters {
    pub(super) fn from(filters: MatchFilters<'_>) -> Self {
        let context = match filters.context {
            ContextFilter::All => PreparedContextFilter::All,
            ContextFilter::Directory => filters
                .current_dir
                .map(normalized_path_components)
                .map(PreparedContextFilter::Directory)
                .unwrap_or(PreparedContextFilter::MissingContext),
            ContextFilter::Workspace => filters
                .workspace_root
                .map(normalized_path_components)
                .map(PreparedContextFilter::Workspace)
                .unwrap_or(PreparedContextFilter::MissingContext),
        };

        Self {
            source: filters.source,
            context,
        }
    }
}

pub(super) fn context_matches<'a>(
    record: &'a Record,
    filters: &PreparedFilters,
    path_cache: &mut HashMap<&'a str, Vec<String>>,
) -> bool {
    match &filters.context {
        PreparedContextFilter::All => true,
        PreparedContextFilter::Directory(current_dir) => {
            cached_path_components(record, path_cache) == current_dir
        }
        PreparedContextFilter::Workspace(root) => {
            components_are_same_or_under(cached_path_components(record, path_cache), root)
        }
        PreparedContextFilter::MissingContext => false,
    }
}

fn cached_path_components<'records, 'cache>(
    record: &'records Record,
    path_cache: &'cache mut HashMap<&'records str, Vec<String>>,
) -> &'cache [String] {
    let cwd = record.cwd.as_str();
    path_cache
        .entry(cwd)
        .or_insert_with(|| normalized_path_components(cwd))
}
