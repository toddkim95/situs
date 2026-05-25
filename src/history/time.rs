use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub(crate) fn human_age(timestamp: u64) -> String {
    human_age_at(now_seconds(), timestamp)
}

pub(crate) fn human_age_at(now: u64, timestamp: u64) -> String {
    let seconds = now.saturating_sub(timestamp);

    if seconds < 60 {
        "just now".to_string()
    } else if seconds < 60 * 60 {
        format!("{}m ago", seconds / 60)
    } else if seconds < 60 * 60 * 24 {
        format!("{}h ago", seconds / 60 / 60)
    } else {
        format!("{}d ago", seconds / 60 / 60 / 24)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_age_can_use_a_cached_now_value() {
        assert_eq!(human_age_at(100, 100), "just now");
        assert_eq!(human_age_at(160, 100), "1m ago");
        assert_eq!(human_age_at(60 * 60 + 100, 100), "1h ago");
        assert_eq!(human_age_at(60 * 60 * 24 + 100, 100), "1d ago");
    }
}
