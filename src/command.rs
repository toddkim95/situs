pub(crate) fn normalize_command(command: &str) -> String {
    command.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn command_word_count(command: &str) -> usize {
    command.split_whitespace().count()
}

pub(crate) fn command_prefix(command: &str, words: usize) -> String {
    command_prefix_words(command, words).join(" ")
}

pub(crate) fn command_prefix_words(command: &str, words: usize) -> Vec<&str> {
    command.split_whitespace().take(words).collect::<Vec<_>>()
}

#[cfg(test)]
fn command_has_prefix(command: &str, prefix: &str) -> bool {
    let normalized = normalize_command(command);
    let prefix = normalize_command(prefix);
    if prefix.is_empty() {
        return false;
    }

    let prefix_words = prefix.split_whitespace().collect::<Vec<_>>();
    command_has_prefix_words(&normalized, &prefix_words)
}

pub(crate) fn command_has_prefix_words(command: &str, prefix_words: &[&str]) -> bool {
    if prefix_words.is_empty() {
        return false;
    }

    let mut command_words = command.split_whitespace();

    prefix_words.iter().enumerate().all(|(index, prefix_word)| {
        let Some(command_word) = command_words.next() else {
            return false;
        };

        if index + 1 == prefix_words.len() {
            command_word.starts_with(prefix_word)
        } else {
            command_word == *prefix_word
        }
    })
}

pub(crate) fn should_ignore_command(command: &str) -> bool {
    command == "situs"
        || command.starts_with("situs ")
        || command == "st"
        || command.starts_with("st ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_prefix_match_accepts_partial_last_word() {
        assert!(command_has_prefix("cargo install --path .", "cargo i"));
        assert!(command_has_prefix(
            "cargo install --path .",
            "cargo install --p"
        ));
        assert!(!command_has_prefix("cargo build", "cargo i"));
        assert!(!command_has_prefix("cargo install", "car install"));
    }

    #[test]
    fn command_prefix_words_match_command_prefix_behavior() {
        let prefix_words = vec!["cargo", "i"];

        assert!(command_has_prefix_words(
            "cargo install --path .",
            &prefix_words
        ));
        assert!(!command_has_prefix_words("cargo build", &prefix_words));
        assert!(!command_has_prefix_words("cargo", &prefix_words));
    }

    #[test]
    fn command_prefix_words_borrows_the_requested_prefix() {
        assert_eq!(
            command_prefix_words(" cargo   install --path . ", 2),
            vec!["cargo", "install"]
        );
        assert_eq!(
            command_prefix_words("cargo install", 5),
            vec!["cargo", "install"]
        );
        assert!(command_prefix_words("   ", 1).is_empty());
    }
}
