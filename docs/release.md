# Release Checklist

This checklist is for maintainers preparing a GitHub or crates.io release.

## Before Tagging

```sh
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
cargo package --locked --no-verify
scripts/verify-picker-modes.sh
cargo audit
```

Then:

- open a release pull request from `develop` to `main`
- update `CHANGELOG.md`
- confirm GitHub Actions are green on Linux and macOS
- confirm the Security workflow is green
- confirm README install URLs and repository owner
- confirm `Cargo.toml` metadata
- confirm `LICENSE`
- regenerate README screenshots if picker visuals changed
- run `situs doctor` from an installed binary
- smoke test `Ctrl-G`, `Tab`, `Enter`, and `Esc` in zsh

## crates.io

Dry run:

```sh
cargo package --locked --no-verify
```

Publish:

```sh
cargo publish
```

## GitHub Release

- create a signed tag if possible
- tag from `main` after the release pull request has merged
- paste the changelog section into the release notes
- include known limitations
- include upgrade instructions

## After Release

- install from the published source
- run `situs --help`
- run `situs doctor`
- verify the README quickstart in a fresh shell
