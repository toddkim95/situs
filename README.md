# situs-cli

[![CI](https://github.com/toddkim95/situs/actions/workflows/ci.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/ci.yml)
[![Security](https://github.com/toddkim95/situs/actions/workflows/security.yml/badge.svg)](https://github.com/toddkim95/situs/actions/workflows/security.yml)

[English](README.md) | [한국어](docs/ko/README.md) | [简体中文](docs/zh-Hans/README.md) | [Español](docs/es/README.md) | [日本語](docs/ja/README.md)

`situs` is a small **command cwd resolver** for zsh.

It remembers where a command worked before, then lets you run or stage that
command from the remembered directory without manually `cd`-ing around.

```text
~/notes
> cargo build
  press Ctrl-G

Situs opens a compact picker:
  cargo build --release        .../work/app        ok        2h ago
> cargo build
  esc quit  up/down select  tab cd  enter run
```

Why this exists:

1. You run `cargo build --release` successfully in `/Users/me/work/app`.
2. Later, from another directory, you type `cargo build --release`.
3. Press the Situs key binding.
4. Pick the previous working directory.
5. Situs turns the shell line into `cd -- /Users/me/work/app && cargo build --release`.

Situs is not a full shell history replacement. Atuin, McFly, fzf, and HSTR are
excellent history searchers; zoxide is an excellent directory jumper. Situs is
focused on one narrow job: resolving "where did this command work before?"

## Screenshots

### Inline Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![Inline search picker showing recent cargo commands above the fixed query line](docs/assets/screenshots/inline-search.svg) | ![Inline inspect view showing the selected command, cwd, status, source, runs, and actions](docs/assets/screenshots/inline-inspect.svg) | ![Inline help view showing Situs keyboard shortcuts](docs/assets/screenshots/inline-help.svg) |

### Fullscreen Picker

| Search | Inspect | Help |
| --- | --- | --- |
| ![Fullscreen search picker showing the same command cwd resolver workflow with more vertical room](docs/assets/screenshots/fullscreen-search.svg) | ![Fullscreen inspect view with command metadata](docs/assets/screenshots/fullscreen-inspect.svg) | ![Fullscreen help view with keyboard shortcuts](docs/assets/screenshots/fullscreen-help.svg) |

## Features

- Remembers command, cwd, exit status, timestamp, and source.
- Prioritizes successful command runs by default.
- Opens a compact inline picker that keeps your current command line visible.
- Supports a fullscreen TUI picker when you prefer a larger surface.
- Lets `Tab` stage the selected directory and command without running it.
- Lets `Enter` cd to the selected directory and run the selected history command.
- Broadens matching from exact commands to useful partial commands such as
  `cargo install`, `cargo install --path`, and `cargo install --path .`.
- Filters by local history, Atuin history, current directory, or current git
  workspace.
- Can import Atuin's SQLite history read-only.
- Keeps a plain line-based picker for non-TTY and scripting scenarios.
- Supports `zsh`, `bash`, and `fish` shells on macOS and Linux.

## Install

### From GitHub

After this repository is public:

```sh
cargo install --git https://github.com/toddkim95/situs
```

If the repository is published under a different owner or name, replace the URL
with the final GitHub URL.

### From A Local Checkout

```sh
git clone https://github.com/toddkim95/situs
cd situs
cargo install --path .
```

### From crates.io

After the crate is published:

```sh
cargo install situs-cli
```

## Quickstart

Add Situs to zsh:

```sh
eval "$(situs init zsh)"
```

Put the same line near the end of `~/.zshrc`, then open a new terminal.

The default binding is `Ctrl-G`. You can change it before loading the init
script:

```sh
export SITUS_BINDKEY='^G'
eval "$(situs init zsh)"
```

Run diagnostics:

```sh
situs doctor
```

Print the picker shortcuts:

```sh
situs keymap
```

For a guided setup flow:

```sh
situs setup
```

The setup wizard can choose picker mode, widget shortcut, shell profile
installation, display language, and a one-time Atuin import.

More install details are in [docs/installation.md](docs/installation.md).

## Everyday Usage

Run commands normally. The zsh integration records interactive commands after
they finish:

```sh
cd ~/work/app
cargo test
```

Later, from any directory:

```sh
cargo test
# press Ctrl-G
```

In the picker:

| Key | Action |
| --- | --- |
| `Up` / `Down` | Select history rows and sync the query with the selected command |
| `Left` / `Right` | Move the query cursor |
| `Tab` | `cd` to the selected directory and keep the command in the shell buffer |
| `Enter` | `cd` to the selected directory and run the selected history command |
| `Ctrl-F` | Toggle failed command history |
| `Ctrl-O` | Inspect the selected history item |
| `F2` | Cycle source filter: all, local, Atuin |
| `F3` | Cycle context filter: all, directory, workspace |
| `Ctrl-Y` | Copy the selected command |
| `Ctrl-D` | Delete the selected Situs history row |
| `Esc` | Quit and keep the original shell input |

Full usage notes are in [docs/usage.md](docs/usage.md).

## Picker Modes

Inline picker, the default:

```sh
situs choose --picker inline --command "cargo build"
```

Fullscreen picker:

```sh
situs choose --picker fullscreen --command "cargo build"
```

Make fullscreen the default with:

```sh
export SITUS_PICKER=fullscreen
```

or run:

```sh
situs setup
```

When several visible rows share the same directory prefix, Situs masks that
shared prefix with `*` so the meaningful path segment is easier to scan. The
real selected directory is still the full path.

## Atuin

Situs can import Atuin history without mutating Atuin's database:

```sh
situs import atuin
```

Enable automatic read-only import before searches:

```sh
situs atuin enable
```

Check status or disable it:

```sh
situs atuin status
situs atuin disable
```

Atuin integration details are in [docs/configuration.md](docs/configuration.md).

## Commands

```sh
situs init zsh
situs setup
situs doctor
situs keymap
situs atuin enable
situs atuin status
situs import atuin
situs record --cwd "$PWD" --status 0 -- "cargo build"
situs choose --picker fullscreen --mode restore --command "cargo build"
situs choose --context workspace --command "cargo test"
situs choose --print-dir --command "cargo build"
situs run -- cargo build
situs stats
```

Run `situs --help` for the complete command summary.

## Configuration

Common environment variables:

| Variable | Purpose |
| --- | --- |
| `SITUS_BINDKEY` | zsh key binding, default `^G` |
| `SITUS_MODE` | zsh execution mode: `stay` or `restore` |
| `SITUS_PICKER` | picker mode: `inline` or `fullscreen` |
| `SITUS_INLINE_ROWS` | number of inline picker rows |
| `SITUS_HISTORY` | override history file path |
| `SITUS_CONFIG` | override config file path |
| `SITUS_ATUIN_SYNC` | Atuin sync override: `off`, `auto`, or `always` |
| `SITUS_LANG` | UI language: `en`, `ko`, `zh-Hans`, `es`, or `ja` |
| `SITUS_PLAIN` | use the simple line-based picker |

See [docs/configuration.md](docs/configuration.md) for storage paths, config
file values, and execution mode details.

## How Situs Compares

| Tool | Main job | Situs's relationship |
| --- | --- | --- |
| Atuin | Rich shell history, context, sync | Situs can import Atuin and uses a smaller cwd resolver workflow |
| McFly | Smart shell history search | Situs resolves the cwd for the command you already started typing |
| fzf | General fuzzy finder and shell key bindings | Situs has a purpose-built picker and shell protocol |
| zoxide | Directory jumping | Situs jumps based on command history, not directory frequency |
| HSTR | Shell history suggest box | Situs keeps command, cwd, status, and action semantics together |

The longer comparison is in [docs/comparison.md](docs/comparison.md).

## Development

Run the full verification matrix (formatting, clippy, unit/acceptance tests, doc translations, and PTY smoke tests) locally:

```sh
scripts/verify-all.sh
```

You can also run individual steps:

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo package --locked --no-verify
scripts/verify-doc-i18n.sh
scripts/verify-picker-modes.sh
```

RustSec advisory audits run in GitHub Actions. To check locally:

```sh
cargo install cargo-audit --locked
cargo audit
```

More contributor guidance is in [CONTRIBUTING.md](CONTRIBUTING.md) and
[docs/development.md](docs/development.md).

When adding or changing user-facing features, update i18n message coverage in
English, Korean, and Simplified Chinese, or document an explicit fallback in the
same change. Runtime and README translation maintenance is documented in
[docs/i18n.md](docs/i18n.md).

Regenerate README screenshots with:

```sh
scripts/capture-screenshots.js
```

The screenshot script uses the real picker with mock history from
`fixtures/screenshot-history.tsv` and retries each capture up to three times.

## Troubleshooting

Start with:

```sh
situs doctor
```

Common fixes:

- Make sure `eval "$(situs init zsh)"` is loaded in `~/.zshrc`.
- Open a new shell after changing `SITUS_BINDKEY`, `SITUS_PICKER`, or
  `SITUS_MODE`.
- After reinstalling with `cargo install --path . --force`, run
  `source ~/.zshrc` or open a new terminal so the already-loaded zsh widget is
  refreshed.
- Use `situs stats` to confirm that history is being recorded.
- Use `situs atuin status` if Atuin results are not appearing.
- Set `SITUS_PLAIN=1` to isolate terminal rendering issues.

More cases are covered in [docs/troubleshooting.md](docs/troubleshooting.md).

## Contributing

Bug reports, UX notes, and small focused pull requests are welcome. Picker
changes need both unit coverage and zsh/PTY smoke coverage because tiny terminal
protocol changes can break the actual shell workflow.

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## Security

Please do not open public issues for security-sensitive reports. See
[SECURITY.md](SECURITY.md).

## License

MIT. See [LICENSE](LICENSE).
