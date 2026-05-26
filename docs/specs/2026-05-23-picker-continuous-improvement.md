# Situs Picker Continuous Improvement Spec

## Scope

This spec governs iterative picker improvements across three tracks:

1. Feature behavior
2. GUI correctness
3. Performance

The picker supports both inline and full-screen terminal views. Inline remains
the default shell-widget experience; full-screen uses the alternate screen for
users who prefer a larger TUI. Both modes share the same modern view inspired by
Atuin and openapi-tui. The query input is fixed on the second-to-last line.
Shortcut help is fixed on the final line. History candidates render above the
query, with newest matching history closest to the query line.

## Feature Behavior

- `Enter` runs the selected history command in the selected directory.
- `Tab` changes the current zsh shell directory to the selected directory and
  leaves the highlighted command in the shell buffer without accepting it.
- `Alt-Enter` and `Alt-Y` paste the selected command into the shell buffer
  without changing directory, accepting the line, or running it.
- `Esc` cancels without mutating the shell buffer or directory.
- `Ctrl-F` toggles failed history. `Ctrl-O` toggles inspect.
- `Ctrl-/` and `F1` toggle the help overlay. `situs keymap` prints the same
  shortcut contract outside the picker.
- `F2` cycles the source filter: all, local, Atuin.
- `F3` cycles the context filter: all, current directory, current git
  workspace. `situs choose --context all|directory|workspace` must behave the
  same way for plain/non-widget flows.
- `Ctrl-Y` copies the selected history command. `Ctrl-D` deletes the selected
  matching row from Situs's history file without mutating any upstream Atuin
  database.
- Picker mode can be selected with `--picker inline|fullscreen`, config key
  `picker_mode=inline|fullscreen`, or `SITUS_PICKER`.
- Query editing is cursor-based: text inserts at the cursor, backspace deletes
  before the cursor, delete removes at the cursor, and left/right/home/end or
  ctrl-a/ctrl-e move the query cursor.
- `Up` and `Down` move through candidates and copy the highlighted command into
  the bottom input. This must not collapse the candidate list; the search scope
  remains based on the user's typed search text until they edit the input.
- Widget mode must require an interactive TUI picker. It must not fall back to
  the plain picker from inside shell command substitution.

## GUI Correctness

- The query line is always the second-to-last rendered line.
- Shortcut help is always the last rendered line and uses compact key badges
  when color is enabled.
- Candidate rows are bottom-aligned directly above the query footer. Empty
  vertical space belongs between the header and the candidates, not between the
  candidates and the query.
- Candidate rows include clear command/directory/status/age columns when space
  allows.
- Source and run-count detail belongs in header chips or inspect/help views, not
  as noisy raw state in every row.
- Context filters should be visible as small header chips only when not `all`.
- Visible candidate rows mask a shared directory prefix as `*` so users can scan
  only the differing path segments. This is display-only; selected cwd values
  always stay full absolute paths.
- Normal mode must not render raw implementation state such as `scope`, `words`,
  or `shown`.
- Candidate rows are rendered bottom-up: index `0` is the newest result and
  appears nearest the query footer.
- Direction keys follow visual movement:
  - `Up` selects the row visually above the current row.
  - `Down` selects the row visually below the current row.
- Render width calculations must handle ANSI styles and common wide Unicode
  characters such as Korean text without overflowing the terminal line.

## Performance

- Opening the picker loads history once.
- Key presses rematch the already-loaded records; they must not reread the
  history file.
- Source/context filter changes rematch the already-loaded records and should
  compute current directory/workspace context once when the picker opens.
- Candidate matching should stay linear in record count per query update.
- Per-query matching should compute the query prefix once and avoid avoidable
  per-record prefix normalization or allocation.
- Rendering should truncate and pad in display columns, not bytes.
- A 10k-record smoke test must remain in the suite.
- Shared directory-prefix masking is computed only for the currently rendered
  visible rows, not the full history.

## Review Loop

For each iteration:

1. Pick the highest-risk gap in the three tracks.
2. Add a focused test that fails for that gap.
3. Implement the smallest change that passes.
4. Run the focused test, then the full verification suite.
5. Re-read this spec and update it only when the intended behavior changes.

## Verification Suite

- `cargo fmt -- --check`
- `cargo test`
- `cargo build`
- `scripts/verify-zsh-widget.sh`
- `scripts/verify-picker-features.sh`
- `scripts/verify-picker-modes.sh` when inline/fullscreen behavior changes

The zsh widget script is required for picker interaction changes. It exercises a
real PTY flow and verifies that `Up` changes the selected command, `Tab` only
changes directory and fills the buffer, and `Enter` changes directory and
executes the selected command. The picker feature script verifies deeper key
behavior including failed-history toggle, help overlay, source filter, context
filter, delete, and copy when `pbcopy` is available.
