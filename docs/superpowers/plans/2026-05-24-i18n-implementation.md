# i18n Implementation Plan

## Goal

Add English-default runtime i18n with Korean and Simplified Chinese support while
preserving Situs's shell protocols, config values, and compact picker layout.

## Spec

- Locale resolution order is `SITUS_LANG`, `LC_ALL`, `LC_MESSAGES`, `LANG`,
  then English fallback.
- Supported locales are English, Korean, and Simplified Chinese.
- Unsupported locales fall back to English.
- Commands, flags, config keys, env vars, history source IDs, and widget
  protocol actions remain English/machine-readable.
- `situs keymap`, `situs doctor`, `situs setup`, and the picker runtime UI
  use localized user-facing labels.
- Picker translations must keep the fixed bottom query line and candidate rows
  readable at narrow and wide terminal widths.

## Steps

1. Add failing tests for locale resolution, message completeness, keymap/doctor
   acceptance output, and picker Korean wide-character rendering.
2. Add `src/i18n/` with typed message keys and per-locale static tables.
3. Wire i18n into `cli`, `doctor`, `setup`, and `picker/render`.
4. Keep existing English output stable enough for current tests.
5. Run `cargo fmt -- --check`, `cargo test`, and `cargo clippy --all-targets -- -D warnings`.

## Done Criteria

- All tests pass.
- New i18n tests prove English, Korean, and Simplified Chinese coverage.
- Picker render tests prove localized labels do not reintroduce raw scope lines
  or break the bottom query contract.
