use crate::error::{cli_error, CliResult};

pub(super) struct ArgCursor<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> ArgCursor<'a> {
    pub(super) fn new(args: &'a [String]) -> Self {
        Self { args, index: 0 }
    }

    pub(super) fn next(&mut self) -> Option<&'a str> {
        let value = self.args.get(self.index)?;
        self.index += 1;
        Some(value)
    }

    pub(super) fn next_value(&mut self, flag: &str) -> CliResult<&'a str> {
        self.next()
            .ok_or_else(|| cli_error(format!("missing value for {flag}")))
    }

    pub(super) fn remaining_joined(&self) -> String {
        self.args[self.index..].join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn returns_missing_value_errors_with_flag_name() {
        let values = args(&["--command"]);
        let mut cursor = ArgCursor::new(&values);
        assert_eq!(cursor.next(), Some("--command"));

        let error = cursor.next_value("--command").unwrap_err();

        assert!(error.to_string().contains("missing value for --command"));
    }

    #[test]
    fn exposes_rest_after_double_dash() {
        let values = args(&["cargo", "--", "test", "--", "--nocapture"]);
        let mut cursor = ArgCursor::new(&values);

        assert_eq!(cursor.next(), Some("cargo"));
        assert_eq!(cursor.next(), Some("--"));
        assert_eq!(cursor.remaining_joined(), "test -- --nocapture");
    }
}
