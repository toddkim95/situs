use std::env;

pub(super) fn header_bar(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("{}{value}\x1b[0m", header_style())
    }
}

pub(super) fn query_bar(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("{}{value}\x1b[0m", query_style())
    }
}

pub(super) fn help_bar(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("{}{value}\x1b[0m", help_style())
    }
}

pub(super) fn selected_bar(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[48;2;44;52;68m{value}\x1b[0m")
    }
}

pub(super) fn header_brand_badge(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!(
            "\x1b[48;2;64;214;122m\x1b[38;2;10;15;24m {value} {}",
            header_style()
        )
    }
}

pub(super) fn header_mode_badge(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!(
            "\x1b[48;2;68;92;145m\x1b[38;2;236;244;255m {value} {}",
            header_style()
        )
    }
}

pub(super) fn header_count_badge(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!(
            "\x1b[48;2;48;58;78m\x1b[38;2;236;244;255m {value} {}",
            header_style()
        )
    }
}

pub(super) fn query_label_badge(value: &str) -> String {
    if no_color() {
        format!(" {value} ")
    } else {
        format!(
            "\x1b[48;2;99;102;241m\x1b[38;2;248;250;252m {value} {}",
            query_style()
        )
    }
}

pub(super) fn query_prompt(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[38;2;163;230;53m{value}{}", query_style())
    }
}

pub(super) fn help_key_badge(value: &str) -> String {
    if no_color() {
        format!("[{value}]")
    } else {
        format!(
            "\x1b[48;2;55;65;81m\x1b[38;2;236;244;255m {value} {}",
            help_style()
        )
    }
}

pub(super) fn bold(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[1m{value}\x1b[22m")
    }
}

pub(super) fn dim(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[2m{value}\x1b[22m")
    }
}

pub(super) fn green(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[32m{value}\x1b[39m")
    }
}

pub(super) fn yellow(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[33m{value}\x1b[39m")
    }
}

pub(super) fn muted_cyan(value: &str) -> String {
    if no_color() {
        value.to_string()
    } else {
        format!("\x1b[38;2;122;162;181m{value}\x1b[39m")
    }
}

fn no_color() -> bool {
    env::var_os("NO_COLOR").is_some()
}

fn header_style() -> &'static str {
    "\x1b[48;2;22;27;39m\x1b[38;2;226;232;240m"
}

fn query_style() -> &'static str {
    "\x1b[48;2;19;23;32m\x1b[38;2;226;232;240m"
}

fn help_style() -> &'static str {
    "\x1b[48;2;12;15;24m\x1b[38;2;148;163;184m"
}
