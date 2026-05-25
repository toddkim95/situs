# Contributing

Thanks for helping make Situs better. This project is intentionally narrow:
Situs is a command cwd resolver, not a general shell history replacement.

## Branch Strategy

Situs uses `develop` as the default development branch and `main` as the stable
release branch.

Typical flow:

```text
fork or feature branch -> pull request to develop -> release pull request to main
```

External contributors should open pull requests from a fork. Collaborators with
write access should still create a branch and open a pull request rather than
pushing directly to `develop` or `main`. Maintainers promote tested changes from
`develop` to `main` when preparing a release.

The protected branch rules for both `develop` and `main` should require pull
requests, passing CI, resolved conversations, and block direct force pushes and
deletions. `main` should be treated as release-only.

## Development Setup

Install Rust stable, then run:

```sh
cargo build --locked
cargo test --locked
```

For zsh picker work, install `zsh` and `expect` so the PTY smoke tests can run.

## Verification

Before opening a pull request, run:

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
scripts/verify-doc-i18n.sh
```

Run `cargo package --locked --no-verify` when changing packaging metadata,
README install instructions, or files expected to ship to crates.io.

RustSec advisory audits run in GitHub Actions when Cargo dependencies change
and on a weekly schedule. To check locally:

```sh
cargo install cargo-audit --locked
cargo audit
```

For picker or shell integration changes, also run:

```sh
scripts/verify-zsh-widget.sh
scripts/verify-picker-modes.sh
```

`scripts/verify-picker-modes.sh` checks both inline and fullscreen modes. Please
keep it passing when changing key handling, rendering, or the zsh widget
protocol. PTY scripts retry up to three times by default; use
`SITUS_VERIFY_RETRIES=1` when you want the first failure preserved for
debugging.

Regenerate screenshots after picker visual changes:

```sh
scripts/capture-screenshots.js
```

Screenshots are captured from the real picker with mock data from
`fixtures/screenshot-history.tsv`. Do not replace them with manually mocked
terminal art.

## Picker Change Contract

Picker behavior has a real shell contract, not just Rust unit tests:

- `Tab` must cd to the selected full cwd, keep the selected command/query in the
  zsh buffer, and must not execute the line.
- `Enter` must cd to the selected full cwd and execute the selected history
  command.
- `Esc` must leave the original user input alone.
- The bottom query line and final shortcut line must remain stable in both
  inline and fullscreen modes.
- Directory prefix masking is display-only and must never change the selected
  cwd sent to the shell.

When adding or changing a picker feature, update:

- unit tests for shared state/input behavior
- acceptance tests when CLI output or shell protocol changes
- `scripts/verify-zsh-widget.sh` for core zsh behavior
- `scripts/verify-picker-features.sh` for deeper key behavior
- `scripts/verify-picker-modes.sh` when both render modes are affected
- README and relevant docs
- i18n message coverage for English, Korean, and Simplified Chinese, or an
  explicit fallback note when a message intentionally stays English
- `situs keymap` and picker help text if a key binding changes

## Documentation

User-visible changes should update the docs in the same pull request. At a
minimum:

- README for quickstart or common workflow changes
- localized READMEs in `docs/ko/README.md` and `docs/zh-Hans/README.md` when
  the root README changes user-visible behavior
- `docs/configuration.md` for config/env changes
- `docs/usage.md` for workflow changes
- `docs/troubleshooting.md` for known sharp edges
- `docs/comparison.md` when positioning changes against adjacent tools

The root `README.md` is the English source of truth. Keep the language switcher
and localized READMEs in sync, and run:

```sh
scripts/verify-doc-i18n.sh
```

More detail is in [docs/i18n.md](docs/i18n.md).

## Pull Requests

Small, focused pull requests are easiest to review. Include:

- what changed
- why it changed
- how you verified it
- screenshots or terminal recordings for picker UI changes when possible

For polished demo recordings, [VHS](https://github.com/charmbracelet/vhs) is a
good optional tool, but it is not required for contributing. The project keeps
expect-based PTY scripts as the required behavior tests because they assert the
real zsh widget contract. Install VHS only when you want to create extra demo
recordings outside the README screenshot flow.

## Issues

Please include:

- shell and terminal
- OS
- `situs doctor` output with sensitive paths redacted if needed
- exact command typed before pressing the Situs key binding
- expected behavior and actual behavior
