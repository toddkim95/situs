# Agent Notes

- Product positioning: Situs is a focused `command cwd resolver`, not a broad
  shell history replacement. It should help the user resolve "where did this
  command work before?" and then run or stage the command from that cwd.
- When a requested UX or behavior is ambiguous, pause before implementing.
- Restate the intended behavior in a compact example, especially for terminal UI layout and key interaction details.
- Ask for confirmation when different interpretations would lead to different user-visible behavior.
- Once the user confirms the interpretation, implement and verify it directly rather than continuing to debate the same point.
- For picker changes, verify with an actual zsh/PTY flow when possible, not only unit tests.
- Picker UX contract: the query line is fixed on the second-to-last line, shortcut help is fixed on the final line, and command history candidates render above the query.
- Picker visual direction: Situs should feel modern and slightly fancy, but the default screen must stay quiet, readable, and decision-focused. Prefer tasteful dark surfaces, subtle badges, restrained color, clear selected-row contrast, and generous alignment over dense status text.
- Keep the header simple by default: brand/version, mode, result count, and the most important state only. Show extra chips such as `all history`, `atuin+local`, `Inspect`, or broadened query state only when they explain a current mode or state.
- Keep advanced detail out of the default list. Put full cwd, exit details, source, run count, sync state, and action explanations in inspect/help/preview views instead of crowding the main picker.
- Footer help should be compact and stable: short key badges on the final line, with a richer help overlay or keymap command for detailed shortcuts.
- Keep `situs keymap`, the picker help overlay, README controls, and zsh/widget behavior in sync whenever a key binding changes.
- Source filtering is a picker-visible state: `F2` cycles all/local/Atuin, header chips may show the active filter, and matching must use the already-loaded record set.
- Context filtering is a picker-visible state: `F3` cycles all/directory/workspace, `--context` must match it in CLI/plain mode, and workspace filtering should use the current git root when available.
- `Ctrl-Y` copies the selected command; `Ctrl-D` deletes matching rows from Situs's history file only. Do not mutate Atuin's SQLite database.
- Picker modes: inline is the default shell-widget mode; fullscreen uses the alternate screen. Any picker mode change must consider CLI flags, config, env override, README, spec, and zsh widget behavior.
- Tab contract: in the zsh widget, Tab must `cd` to the selected full cwd, fill the shell buffer with the selected command/query, reset the prompt, and must not accept or execute the line.
- Enter/run contract: in the zsh widget, after setting `BUFFER` and `CURSOR`,
  call `zle reset-prompt`, then `zle accept-line`, then return immediately.
  Keep a zsh/PTY regression case with a multiline prompt and `RPROMPT` so prompt
  geometry regressions are caught, not only command execution semantics.
- Reinstall caveat: `cargo install --path . --force` updates the binary but does
  not refresh an already-loaded zsh widget in the user's current terminal.
  Troubleshooting and final notes should tell the user to `source ~/.zshrc` or
  open a new terminal after widget behavior changes. Keep the legacy
  `--print-selection` guard so stale widgets cannot execute the selected command
  on Tab.
- Directory prefix masking is display-only. Never mutate `Candidate.cwd` or shell selection output when showing `*` for shared path prefixes.
- Refactor and performance changes should follow the Superpowers loop: add or identify behavior/performance contract tests first, make the smallest change, then rerun focused tests before the full verification gate.
- When adding user-visible features, update README and `docs/specs/2026-05-23-picker-continuous-improvement.md`; update `situs help` text for CLI flags or commands.
- i18n contract: every user-facing feature or message change must update the
  i18n message coverage for English, Korean, and Simplified Chinese, or make an
  explicit fallback decision in tests/docs. Do not localize protocol values,
  command names, flags, config keys, or environment variables.
- Runtime i18n uses `SITUS_LANG`, then `LC_ALL`, `LC_MESSAGES`, and `LANG`;
  English is the fallback. Keep new runtime strings behind `MessageKey` coverage
  tests unless they are intentionally protocol/machine-readable.
- Current code map: `src/cli/` owns command dispatch plus `choose`, `record`,
  and `import` subcommands; `src/cli/args.rs` owns small shared flag parsing
  helpers. `src/history/` owns TSV storage, escaping, and time formatting;
  `src/atuin/` owns read-only DB access, import merging, and sync state;
  `src/matcher/` owns candidate generation plus source/context filters;
  `src/picker/` owns input, rendering, session control, path masking, clipboard,
  and render modes. Within picker, `keymap.rs` maps raw keys to semantic
  intents, `keys.rs` decodes terminal bytes, `viewport.rs` owns inline/fullscreen
  row math and ANSI line-space sequences, and `render/` owns visual line
  construction split by header/footer/candidate/inspect/helper concerns.
  `src/i18n/` owns locale resolution plus per-locale message tables. Keep new
  code in the nearest existing module before adding a new one.
- README i18n contract: root `README.md` is the English source of truth.
  Localized public READMEs live at `docs/ko/README.md` and
  `docs/zh-Hans/README.md`. When root README behavior text changes, update
  both translations or document an explicit fallback, keep the language switcher
  synced, and run `scripts/verify-doc-i18n.sh`.
- Open-source docs contract: keep README as the fast public entry point, and put
  detailed install/config/usage/development/troubleshooting/release material in
  `docs/`. When behavior changes, update the relevant detailed doc in the same
  change.
- CI contract: keep GitHub Actions aligned with the local verification matrix.
  Use locked Cargo commands for reproducibility, keep Linux and macOS coverage
  for supported Unix shells, run documentation i18n checks, and keep the zsh/PTY
  picker smoke tests in CI. Packaging metadata changes should be covered by
  `cargo package --locked --no-verify`.
- Security audit contract: keep the RustSec advisory workflow active for
  dependency changes and scheduled scans. When `Cargo.toml` or `Cargo.lock`
  changes, run or account for `cargo audit`, and document any ignored advisory
  with a reason rather than silently suppressing it.
- Keep GitHub community files current for public release readiness:
  `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CHANGELOG.md`,
  issue templates, PR template, and CI workflow.
- README screenshots live in `docs/assets/screenshots/` and are generated by
  `scripts/capture-screenshots.js`. The script must capture the real picker via
  PTY using mock data from `fixtures/screenshot-history.tsv`; do not use
  hand-painted fixture screenshots. If picker visuals change, regenerate inline
  and fullscreen screenshots and keep README image references current. Keep the
  built-in retry path intact unless you are debugging a first failure. Screenshot
  TSV fixture writing lives in `scripts/lib/history-fixture.js`; acceptance test
  TSV fixture writing lives in `tests/support/history.rs`.
- Before a release or README install change, confirm the final GitHub repository
  owner/name and update install URLs in README and `docs/installation.md`.
- Keep competitive research notes in `docs/research/` when feature decisions are driven by Atuin/McFly/zoxide/fzf/HSTR comparisons.
- Keep `scripts/verify-zsh-widget.sh` aligned with the core widget protocol. Keep `scripts/verify-picker-features.sh` aligned with deeper key behavior such as Ctrl-F, Ctrl-/, F2, F3, Ctrl-D, and Ctrl-Y. Use `scripts/verify-picker-modes.sh` when mode behavior is relevant so inline and fullscreen are both checked. A picker interaction change is not done until `cargo fmt -- --check`, `cargo test`, `cargo build`, and the zsh/PTY smoke test pass when permissions allow.
- Whenever a picker feature is added or changed, add or update tests for both render modes. Unit tests should cover the shared state/input logic, `scripts/verify-zsh-widget.sh` should cover core zsh widget behavior, and `scripts/verify-picker-features.sh` plus `scripts/verify-picker-modes.sh` should cover the user-visible behavior in both inline and fullscreen modes.
