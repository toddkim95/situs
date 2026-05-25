use crate::model::Candidate;

pub(super) fn common_directory_prefix<'a>(
    candidates: impl IntoIterator<Item = &'a Candidate>,
) -> Option<String> {
    let mut paths = candidates
        .into_iter()
        .map(|candidate| candidate.cwd.as_str());
    let first = paths.next()?;
    let mut prefix_len = first.len();
    let mut count = 1;

    for path in paths {
        count += 1;
        prefix_len = common_prefix_len(&first[..prefix_len], path);
        if prefix_len == 0 {
            return None;
        }
    }

    if count < 2 {
        return None;
    }

    let prefix = &first[..prefix_len];
    let boundary = prefix.rfind('/')?;
    if boundary == 0 {
        return None;
    }

    Some(first[..=boundary].to_string())
}

pub(super) fn masked_cwd(cwd: &str, common_prefix: Option<&str>) -> String {
    let Some(prefix) = common_prefix else {
        return cwd.to_string();
    };
    let Some(suffix) = cwd.strip_prefix(prefix) else {
        return cwd.to_string();
    };
    if suffix.is_empty() {
        "*".to_string()
    } else {
        format!("*/{}", suffix.trim_start_matches('/'))
    }
}

fn common_prefix_len(left: &str, right: &str) -> usize {
    let mut len = 0;
    for ((left_index, left_char), (_, right_char)) in left.char_indices().zip(right.char_indices())
    {
        if left_char != right_char {
            break;
        }
        len = left_index + left_char.len_utf8();
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::CandidateSource;

    fn candidate(cwd: &str) -> Candidate {
        Candidate {
            cwd: cwd.to_string(),
            command: "cargo build".to_string(),
            timestamp: 1,
            status: 0,
            source: CandidateSource::Local,
            run_count: 1,
            success_count: 1,
        }
    }

    #[test]
    fn common_prefix_uses_directory_boundary() {
        let candidates = [
            candidate("/Users/me/work/api"),
            candidate("/Users/me/work/web"),
        ];

        let prefix = common_directory_prefix(candidates.iter()).unwrap();

        assert_eq!(prefix, "/Users/me/work/");
        assert_eq!(masked_cwd("/Users/me/work/api", Some(&prefix)), "*/api");
    }

    #[test]
    fn common_prefix_is_none_for_single_or_root_only_matches() {
        let single = [candidate("/Users/me/work/api")];
        let root_only = [candidate("/tmp/api"), candidate("/var/api")];

        assert_eq!(common_directory_prefix(single.iter()), None);
        assert_eq!(common_directory_prefix(root_only.iter()), None);
    }

    #[test]
    fn masking_falls_back_when_prefix_does_not_match() {
        assert_eq!(masked_cwd("/tmp/api", Some("/Users/me/")), "/tmp/api");
        assert_eq!(masked_cwd("/tmp/api", None), "/tmp/api");
    }
}
