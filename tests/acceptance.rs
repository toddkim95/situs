use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

mod support;

use support::history::record_line;

fn situs() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_situs"))
}

struct TestHome {
    root: PathBuf,
    history: PathBuf,
    config: PathBuf,
    xdg_data: PathBuf,
}

impl TestHome {
    fn new(label: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "situs-acceptance-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let xdg_data = root.join("xdg-data");
        fs::create_dir_all(&xdg_data).unwrap();
        Self {
            history: root.join("history.tsv"),
            config: root.join("config"),
            xdg_data,
            root,
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(situs());
        command
            .env("HOME", &self.root)
            .env("XDG_DATA_HOME", &self.xdg_data)
            .env("SITUS_HISTORY", &self.history)
            .env("SITUS_CONFIG", &self.config)
            .env("SITUS_ATUIN_SYNC", "off")
            .env("LANG", "C")
            .env_remove("LC_ALL")
            .env_remove("LC_MESSAGES")
            .env_remove("SITUS_LANG")
            .env_remove("ATUIN_DB");
        command
    }

    fn write_history(&self, records: &[(&str, i32, &Path, &str)]) {
        let mut contents = String::new();
        for (timestamp, status, cwd, command) in records {
            contents.push_str(&record_line(timestamp, *status, cwd, command));
        }
        fs::write(&self.history, contents).unwrap();
    }
}

fn run(command: &mut Command) -> Output {
    command.output().unwrap()
}

fn run_with_stdin(command: &mut Command, stdin: &str) -> Output {
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn doctor_reports_configured_picker_mode() {
    let home = TestHome::new("doctor-picker-mode");
    fs::write(&home.config, "picker_mode=fullscreen\natuin_sync=off\n").unwrap();

    let output = run(home.command().arg("doctor"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(stdout(&output).contains("picker mode     fullscreen"));
}

#[test]
fn keymap_prints_picker_shortcuts() {
    let home = TestHome::new("keymap");

    let output = run(home.command().arg("keymap"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs Keymap"));
    assert!(stdout.contains("Ctrl-/"));
    assert!(stdout.contains("Ctrl-Y"));
    assert!(stdout.contains("Ctrl-D"));
    assert!(stdout.contains("F2"));
    assert!(stdout.contains("F3"));
}

#[test]
fn keymap_uses_korean_when_situs_lang_is_ko() {
    let home = TestHome::new("keymap-ko");

    let output = run(home.command().env("SITUS_LANG", "ko").arg("keymap"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs 키맵"));
    assert!(stdout.contains("선택기"));
    assert!(stdout.contains("선택한 디렉터리로 cd"));
}

#[test]
fn keymap_uses_simplified_chinese_when_situs_lang_is_zh_hans() {
    let home = TestHome::new("keymap-zh-hans");

    let output = run(home.command().env("SITUS_LANG", "zh-Hans").arg("keymap"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs 键位"));
    assert!(stdout.contains("选择器"));
    assert!(stdout.contains("cd 到选中的目录"));
}

#[test]
fn doctor_uses_korean_when_situs_lang_is_ko() {
    let home = TestHome::new("doctor-ko");
    fs::write(&home.config, "picker_mode=inline\natuin_sync=off\n").unwrap();

    let output = run(home.command().env("SITUS_LANG", "ko").arg("doctor"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs 진단"));
    assert!(stdout.contains("히스토리 레코드"));
    assert!(stdout.contains("선택기 모드"));
}

#[test]
fn init_zsh_prints_widget_and_default_binding() {
    let home = TestHome::new("init-zsh");

    let output = run(home.command().args(["init", "zsh"]));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("situs choose --print-widget-selection"));
    assert!(stdout.contains("SITUS_BINDKEY:=^G"));
    assert!(stdout.contains("bindkey \"$SITUS_BINDKEY\" situs-accept-from-history"));
}

#[test]
fn init_bash_prints_widget_and_binding() {
    let home = TestHome::new("init-bash");

    let output = run(home.command().args(["init", "bash"]));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("situs choose --print-widget-selection"));
    assert!(stdout.contains("bind -x"));
}

#[test]
fn init_fish_prints_widget_and_binding() {
    let home = TestHome::new("init-fish");

    let output = run(home.command().args(["init", "fish"]));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("situs choose --print-widget-selection"));
    assert!(stdout.contains("bind $SITUS_BINDKEY __situs_choose_widget"));
}

#[test]
fn record_then_plain_choose_prints_selected_directory_and_command() {
    let home = TestHome::new("record-choose");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();

    let record = run(home.command().args([
        "record",
        "--cwd",
        project.to_str().unwrap(),
        "--status",
        "0",
        "--",
        "cargo",
        "build",
    ]));
    assert!(record.status.success(), "stderr: {}", stderr(&record));

    let output = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--print-selection",
            "--command",
            "cargo build",
        ]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!("{}\ncargo build\n", project.display())
    );
}

#[test]
fn plain_choose_print_dir_outputs_only_directory() {
    let home = TestHome::new("choose-print-dir");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    home.write_history(&[("10", 0, &project, "cargo build")]);

    let output = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--print-dir",
            "--command",
            "cargo build",
        ]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(stdout(&output), format!("{}\n", project.display()));
}

#[test]
fn plain_choose_broadens_query_and_prints_selected_history_command() {
    let home = TestHome::new("choose-broaden");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    home.write_history(&[("10", 0, &project, "cargo install --path . --force")]);

    let output = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--print-selection",
            "--command",
            "cargo install",
        ]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!("{}\ncargo install --path . --force\n", project.display())
    );
}

#[test]
fn plain_choose_can_filter_to_current_git_workspace() {
    let home = TestHome::new("choose-context-workspace");
    let repo = home.root.join("repo");
    let project = repo.join("project");
    let other = home.root.join("other");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&other).unwrap();
    let git = Command::new("git")
        .args(["init"])
        .arg(&repo)
        .output()
        .unwrap();
    assert!(
        git.status.success(),
        "git init stderr: {}",
        String::from_utf8_lossy(&git.stderr)
    );
    home.write_history(&[
        ("10", 0, &other, "cargo test --other"),
        ("20", 0, &project, "cargo test --workspace"),
    ]);

    let output = run_with_stdin(
        home.command()
            .current_dir(&project)
            .env("SITUS_PLAIN", "1")
            .args([
                "choose",
                "--context",
                "workspace",
                "--print-selection",
                "--command",
                "cargo test",
            ]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!("{}\ncargo test --workspace\n", project.display())
    );
}

#[test]
fn plain_choose_can_filter_to_current_directory_context() {
    let home = TestHome::new("choose-context-directory");
    let project = home.root.join("project");
    let other = home.root.join("other");
    fs::create_dir_all(&project).unwrap();
    fs::create_dir_all(&other).unwrap();
    home.write_history(&[
        ("10", 0, &other, "cargo test --other"),
        ("20", 0, &project, "cargo test --project"),
    ]);

    let output = run_with_stdin(
        home.command()
            .current_dir(&project)
            .env("SITUS_PLAIN", "1")
            .args([
                "choose",
                "--context",
                "directory",
                "--print-selection",
                "--command",
                "cargo test",
            ]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert_eq!(
        stdout(&output),
        format!("{}\ncargo test --project\n", project.display())
    );
}

#[test]
fn import_atuin_cli_imports_database_history() {
    let home = TestHome::new("import-atuin");
    let db = home.root.join("atuin.db");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    let connection = Connection::open(&db).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE history (
                timestamp INTEGER,
                exit INTEGER,
                cwd TEXT,
                command TEXT,
                deleted_at INTEGER
            );",
        )
        .unwrap();
    let project_text = project.to_string_lossy().into_owned();
    connection
        .execute(
            "INSERT INTO history (timestamp, exit, cwd, command, deleted_at)
             VALUES (?1, ?2, ?3, ?4, NULL)",
            (1_700_000_000_i64, 0_i32, project_text, "cargo build"),
        )
        .unwrap();

    let import = run(home
        .command()
        .args(["import", "atuin", "--db", db.to_str().unwrap()]));
    assert!(import.status.success(), "stderr: {}", stderr(&import));
    assert!(stdout(&import).contains("Imported 1 Atuin history records"));
    let history = fs::read_to_string(&home.history).unwrap();
    assert!(history.contains("\tatuin\n"));

    let choose = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--print-selection",
            "--command",
            "cargo build",
        ]),
        "\n",
    );
    assert!(choose.status.success(), "stderr: {}", stderr(&choose));
    assert_eq!(
        stdout(&choose),
        format!("{}\ncargo build\n", project.display())
    );
}

#[test]
fn failed_history_is_hidden_until_include_failed_is_requested() {
    let home = TestHome::new("failed-history");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    home.write_history(&[("10", 2, &project, "cargo build")]);

    let hidden = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--print-selection",
            "--command",
            "cargo build",
        ]),
        "\n",
    );
    assert!(!hidden.status.success());
    assert!(stderr(&hidden).contains("only failed history found"));

    let shown = run_with_stdin(
        home.command().env("SITUS_PLAIN", "1").args([
            "choose",
            "--include-failed",
            "--print-selection",
            "--command",
            "cargo build",
        ]),
        "\n",
    );
    assert!(shown.status.success(), "stderr: {}", stderr(&shown));
    assert_eq!(
        stdout(&shown),
        format!("{}\ncargo build\n", project.display())
    );
}

#[test]
fn setup_writes_inline_picker_mode_by_default() {
    let home = TestHome::new("setup-inline");

    let output = run_with_stdin(home.command().arg("setup"), "\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let config = fs::read_to_string(&home.config).unwrap();
    assert!(config.contains("picker_mode=inline\n"));
    assert!(!config.contains("atuin_sync=auto\n"));
}

#[test]
fn setup_writes_fullscreen_picker_mode_without_atuin_prompt_when_atuin_is_absent() {
    let home = TestHome::new("setup-fullscreen");

    let output = run_with_stdin(home.command().arg("setup"), "2\n");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let config = fs::read_to_string(&home.config).unwrap();
    assert!(config.contains("picker_mode=fullscreen\n"));
    assert!(!config.contains("atuin_sync=auto\n"));
}

#[test]
fn atuin_enable_status_disable_cycle_updates_config() {
    let home = TestHome::new("atuin-cycle");

    let enable = run(home.command().args(["atuin", "enable"]));
    assert!(enable.status.success(), "stderr: {}", stderr(&enable));
    assert!(fs::read_to_string(&home.config)
        .unwrap()
        .contains("atuin_sync=auto\n"));

    let status = run(home.command().args(["atuin", "status"]));
    assert!(status.status.success(), "stderr: {}", stderr(&status));
    assert!(stdout(&status).contains("Atuin auto-sync off"));
    assert!(stdout(&status).contains("env override SITUS_ATUIN_SYNC=off"));

    let disable = run(home.command().args(["atuin", "disable"]));
    assert!(disable.status.success(), "stderr: {}", stderr(&disable));
    assert!(fs::read_to_string(&home.config)
        .unwrap()
        .contains("atuin_sync=off\n"));
}

#[test]
fn stats_summarizes_history() {
    let home = TestHome::new("stats");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    home.write_history(&[
        ("10", 0, &project, "cargo test"),
        ("20", 1, &project, "cargo test"),
        ("30", 0, &project, "cargo build"),
    ]);

    let output = run(home.command().arg("stats"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs Stats"));
    assert!(stdout.contains("records     3"));
    assert!(stdout.contains("failed      1"));
    assert!(stdout.contains("2  cargo test"));
}

#[test]
fn run_executes_selected_history_command_in_selected_directory() {
    let home = TestHome::new("run-command");
    let project = home.root.join("project");
    fs::create_dir_all(&project).unwrap();
    home.write_history(&[("10", 0, &project, "pwd")]);

    let output = run_with_stdin(
        home.command()
            .env("SITUS_PLAIN", "1")
            .args(["run", "--", "pwd"]),
        "\n",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    assert!(stdout(&output).contains(project.to_str().unwrap()));
}

#[test]
fn widget_selection_mode_refuses_plain_fallback() {
    let home = TestHome::new("widget-needs-tui");

    let output = run(home.command().env("SITUS_PLAIN", "1").args([
        "choose",
        "--print-widget-selection",
        "--command",
        "cargo build",
    ]));

    assert!(!output.status.success());
    assert!(stderr(&output).contains("interactive picker is required"));
}

#[test]
fn keymap_uses_spanish_when_situs_lang_is_es() {
    let home = TestHome::new("keymap-es");

    let output = run(home.command().env("SITUS_LANG", "es").arg("keymap"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Keymap de Situs"));
    assert!(stdout.contains("Selector"));
    assert!(stdout.contains("cd al directorio seleccionado"));
}

#[test]
fn keymap_uses_japanese_when_situs_lang_is_ja() {
    let home = TestHome::new("keymap-ja");

    let output = run(home.command().env("SITUS_LANG", "ja").arg("keymap"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs キーマップ"));
    assert!(stdout.contains("ピッカー"));
    assert!(stdout.contains("選択したディレクトリにcd"));
}

#[test]
fn doctor_uses_spanish_when_situs_lang_is_es() {
    let home = TestHome::new("doctor-es");
    fs::write(&home.config, "picker_mode=inline\natuin_sync=off\n").unwrap();

    let output = run(home.command().env("SITUS_LANG", "es").arg("doctor"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Diagnóstico de Situs"));
    assert!(stdout.contains("registros del historial"));
    assert!(stdout.contains("modo de selector"));
}

#[test]
fn doctor_uses_japanese_when_situs_lang_is_ja() {
    let home = TestHome::new("doctor-ja");
    fs::write(&home.config, "picker_mode=inline\natuin_sync=off\n").unwrap();

    let output = run(home.command().env("SITUS_LANG", "ja").arg("doctor"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs ドクター"));
    assert!(stdout.contains("履歴レコード"));
    assert!(stdout.contains("ピッカーモード"));
}

#[test]
fn configured_language_overrides_system_locale() {
    let home = TestHome::new("configured-lang-override");
    fs::write(
        &home.config,
        "picker_mode=inline\natuin_sync=off\nlanguage=ko\n",
    )
    .unwrap();

    // With LANG=C (set in command()), language=ko in config should override it!
    let output = run(home.command().arg("doctor"));

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("Situs 진단"));
    assert!(stdout.contains("히스토리 레코드"));
    assert!(stdout.contains("선택기 모드"));
}
