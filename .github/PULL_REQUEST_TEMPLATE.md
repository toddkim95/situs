## Summary

What changed?

## Why

What user problem or maintenance problem does this solve?

## Verification

- [ ] `cargo fmt -- --check`
- [ ] `cargo test --locked`
- [ ] `cargo clippy --locked --all-targets -- -D warnings`
- [ ] `cargo build --locked`
- [ ] `cargo package --locked --no-verify` when packaging metadata or public docs changed
- [ ] `cargo audit` when Cargo dependencies changed
- [ ] `scripts/verify-doc-i18n.sh`
- [ ] `scripts/verify-picker-modes.sh` when picker or shell behavior changed
- [ ] Updated runtime i18n messages and localized READMEs when user-facing text changed

## Notes

Screenshots, terminal recordings, or compatibility notes:
