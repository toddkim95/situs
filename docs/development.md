# Development

## Architecture

Situs is organized around a few small layers:

- `cli`: command parsing and command dispatch, split by subcommand
- `shell`: zsh integration script generation
- `history`: local history storage, TSV escaping, and age formatting
- `atuin`: read-only Atuin DB access, import merging, and sync behavior
- `matcher`: command/cwd matching, candidate ranking, and filters
- `picker`: interactive terminal UI and key handling
- `i18n`: locale resolution and per-locale runtime strings
- `config`: config file and environment override resolution
- `doctor` and `stats`: diagnostics

The larger layers are directory modules:

- `src/cli/args.rs`, `choose.rs`, `record.rs`, and `import.rs` keep small flag
  parsing helpers and subcommand behavior separate from top-level dispatch.
- `src/history/encoding.rs` and `time.rs` keep storage escaping and age
  formatting reusable.
- `src/atuin/db.rs`, `import.rs`, and `sync.rs` keep SQLite access, import
  dedupe, and auto-sync state independent.
- `src/matcher/candidates.rs` and `filters.rs` separate ranking from
  source/context filtering.
- `src/picker/keymap.rs`, `keys.rs`, `input.rs`, `path.rs`, `style.rs`,
  `width.rs`, `viewport.rs`, `session.rs`, `plain.rs`, `tui.rs`, and
  `clipboard.rs` keep terminal input, key decoding, viewport geometry, and
  command matching separated.
- `src/picker/render/` keeps visual line construction split by
  `header.rs`, `footer.rs`, `candidate.rs`, `inspect.rs`, and shared helpers.
- `src/i18n/*.rs` stores one locale table per file, with `mod.rs` owning
  `Locale`, `I18n`, and `MessageKey`.

## Branch Strategy

The repository uses `develop` for day-to-day development and `main` for stable
release history.

- External contributors open pull requests from forks into `develop`.
- Collaborators with write access create feature branches and open pull
  requests into `develop`.
- Maintainers promote `develop` to `main` with a release pull request.
- Direct pushes, force pushes, and branch deletions should be blocked on both
  `develop` and `main` through GitHub rulesets or branch protection.

GitHub Actions must run for pull requests targeting protected branches and for
pushes to both `develop` and `main`, so required status checks match the active
branch strategy.

## Test Matrix

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
```

Run the RustSec advisory audit locally when changing dependencies:

```sh
cargo install cargo-audit --locked
cargo audit
```

GitHub Actions also runs the security audit when `Cargo.toml`, `Cargo.lock`, or
the security workflow changes, plus a weekly scheduled scan for newly published
advisories.

Run the zsh/PTY checks for picker behavior:

```sh
scripts/verify-zsh-widget.sh
scripts/verify-picker-features.sh
scripts/verify-picker-modes.sh
```

`verify-picker-features.sh` covers deeper picker keys such as failed-history,
help, source/context filters, delete, and copy. `verify-picker-modes.sh` runs
the widget and feature smoke tests in both inline and fullscreen modes. PTY
checks retry up to three times by default because terminal allocation can be
briefly flaky in CI or sandboxed environments. Override with
`SITUS_VERIFY_RETRIES=1` when debugging the first failure.

## Screenshots

README screenshots live in `docs/assets/screenshots/`.

Regenerate them with:

```sh
scripts/capture-screenshots.js
```

The script runs the real picker in an expect PTY, not a hand-painted fixture. It
uses `fixtures/screenshot-history.tsv` as mock command history, rewrites the
relative timestamps at runtime, and retries each screenshot up to three times.
Override with `SITUS_SCREENSHOT_RETRIES=1` when debugging.

It passes `SITUS_TTY=$(tty)` to the spawned process so Situs opens the PTY
slave directly instead of relying on `/dev/tty`, which can be blocked in
sandboxed environments.

The screenshot fixture writer lives in `scripts/lib/history-fixture.js`.
Acceptance-test TSV fixture helpers live in `tests/support/history.rs`.

For polished demo videos, [VHS](https://github.com/charmbracelet/vhs) is the
best optional tool to add later: it records terminal sessions from `.tape`
files and supports PNG screenshots. Keep it optional for contributors because
the expect-based PTY checks are still the source of truth for shell behavior.
[asciinema](https://docs.asciinema.org/) plus
[agg](https://docs.asciinema.org/manual/agg/) is also good for shareable
recordings, but it is less direct for asserting zsh widget behavior.

Optional VHS workflow:

```sh
brew install vhs
vhs new demo.tape
vhs demo.tape
```

VHS also needs `ttyd` and `ffmpeg` on `PATH`; the Homebrew package is the
simplest route on macOS.

Only use VHS outputs as extra demos. README screenshots should continue to come
from `scripts/capture-screenshots.js` so they use the same mock history and
retry behavior as local verification.

## Adding A Feature

1. Define the user-visible behavior.
2. Add or update focused unit tests.
3. Add acceptance or PTY coverage if shell behavior changes.
4. Implement the smallest change that satisfies the behavior.
5. Update README, localized READMEs, and docs.
6. Run the verification matrix.

## Performance Notes

Interactive picker input should not reread history on every key press. Load
records once, then rematch in memory.

Avoid formatting expensive path display values repeatedly when the candidate set
has not changed. The current code already caches shared path display work in the
render path where practical.

Large-history changes should include a smoke test or benchmark-style check that
uses thousands of records.

## Release Checklist

Before a public release:

- update `CHANGELOG.md`
- run the full verification matrix
- confirm GitHub Actions are green on Linux and macOS
- regenerate screenshots when picker visuals changed
- check `cargo package --locked --no-verify` locally if publishing to crates.io
- confirm README install URLs match the public repository
- create a GitHub release with a short changelog
- attach binaries only after there is a reproducible release workflow
