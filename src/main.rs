mod atuin;
mod cli;
mod command;
mod config;
mod context;
mod doctor;
mod error;
mod history;
mod i18n;
mod matcher;
mod model;
mod picker;
mod setup;
mod shell;
mod stats;
mod terminal;

fn main() {
    let exit_code = match cli::run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("situs: {error}");
            1
        }
    };

    std::process::exit(exit_code);
}
