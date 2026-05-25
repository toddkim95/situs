use crate::i18n::{I18n, MessageKey};
use crate::model::{ContextFilter, SourceFilter};
use crate::picker::style::{
    dim, header_bar, header_brand_badge, header_count_badge, header_mode_badge, muted_cyan,
};

use super::helpers::join_sides;

pub(super) struct HeaderState<'a> {
    pub(super) visible_count: usize,
    pub(super) candidate_count: usize,
    pub(super) include_failed: bool,
    pub(super) inspect: bool,
    pub(super) source_filter: SourceFilter,
    pub(super) context_filter: ContextFilter,
    pub(super) message: Option<&'a str>,
}

pub(super) fn header_line(state: HeaderState<'_>, width: usize, i18n: I18n) -> String {
    let mode = if state.inspect {
        i18n.text(MessageKey::PickerInspect)
    } else {
        i18n.text(MessageKey::PickerSearch)
    };
    let left = format!(
        " {} {} {}",
        header_brand_badge("Situs"),
        dim(&format!("v{}", env!("CARGO_PKG_VERSION"))),
        header_mode_badge(mode)
    );
    let noun = if state.visible_count == 1 {
        i18n.text(MessageKey::PickerResultSingular)
    } else {
        i18n.text(MessageKey::PickerResultPlural)
    };
    let history = if state.include_failed {
        i18n.text(MessageKey::PickerAllHistory)
    } else {
        i18n.text(MessageKey::PickerSuccessfulHistory)
    };
    let mut right_parts = vec![
        header_count_badge(&format!(
            "{}/{} {noun}",
            state.visible_count, state.candidate_count
        )),
        dim(history),
    ];
    if state.source_filter != SourceFilter::All {
        right_parts.push(header_mode_badge(state.source_filter.as_str()));
    }
    if state.context_filter != ContextFilter::All {
        right_parts.push(header_mode_badge(state.context_filter.as_str()));
    }
    if let Some(message) = state.message {
        right_parts.push(muted_cyan(message));
    }
    let right = right_parts.join(" ");
    header_bar(&join_sides(&left, &right, width))
}
