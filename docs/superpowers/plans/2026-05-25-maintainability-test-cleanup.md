# Maintainability And Test Cleanup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Reduce Situs's largest maintenance hotspots while preserving picker behavior and strengthening tests around the code being moved.

**Architecture:** Keep the existing crossterm inline/fullscreen picker behavior intact. Split oversized modules by responsibility, extract pure key/viewport helpers for focused tests, and isolate fixture-writing helpers so acceptance tests stay readable.

**Tech Stack:** Rust 2021, crossterm, ratatui-crossterm, zsh/PTY shell smoke scripts, Node.js screenshot capture script.

---

## File Structure

- `src/picker/render/`: owns visible picker lines only.
  - `mod.rs`: public render entry points and render characterization tests.
  - `header.rs`: header state and compact top bar rendering.
  - `footer.rs`: fixed query/footer line rendering and query cursor math.
  - `candidate.rs`: candidate list rows, badges, path truncation, and column layout.
  - `inspect.rs`: inspect panel rendering.
  - `helpers.rs`: width-aware padding, truncation, elapsed formatting, and side joining.
- `src/picker/keymap.rs`: maps `crossterm::event::KeyEvent` to semantic picker intents.
- `src/picker/keys.rs`: decodes terminal key byte sequences into crossterm key events.
- `src/picker/viewport.rs`: computes inline picker rows and emits terminal line cleanup/open/close sequences.
- `src/cli/args.rs`: small argument cursor helpers shared by CLI parsers.
- `tests/support/history.rs`: acceptance-test history fixture encoding helpers.
- `scripts/lib/history-fixture.js`: screenshot fixture TSV writer used by screenshot capture.
- `AGENTS.md`, `docs/development.md`: update the code map and testing contracts.

---

### Task 1: Establish Baseline

**Files:**
- Read-only: `src/picker/render.rs`
- Read-only: `src/picker/input.rs`
- Read-only: `src/picker/session.rs`
- Read-only: `src/cli/choose.rs`
- Read-only: `tests/acceptance.rs`

- [ ] **Step 1: Run the current focused tests**

Run:

```bash
cargo test --locked picker:: acceptance -- --nocapture
```

Expected: PASS. If it fails, use `superpowers:systematic-debugging` before changing production code.

- [ ] **Step 2: Run formatting baseline**

Run:

```bash
cargo fmt -- --check
```

Expected: PASS.

---

### Task 2: Split Picker Renderer

**Files:**
- Move: `src/picker/render.rs` to `src/picker/render/mod.rs`
- Create: `src/picker/render/header.rs`
- Create: `src/picker/render/footer.rs`
- Create: `src/picker/render/candidate.rs`
- Create: `src/picker/render/inspect.rs`
- Create: `src/picker/render/helpers.rs`

- [ ] **Step 1: Move the renderer into a module directory**

Run:

```bash
mkdir -p src/picker/render
mv src/picker/render.rs src/picker/render/mod.rs
```

Expected: `cargo test --locked picker::render -- --nocapture` still PASS before extraction.

- [ ] **Step 2: Extract helper functions**

Move these functions to `src/picker/render/helpers.rs` and import them from `mod.rs`:

```rust
pub(super) fn join_sides(...)
pub(super) fn fit_line(...)
pub(super) fn truncate(...)
pub(super) fn truncate_start(...)
pub(super) fn pad_right(...)
pub(super) fn pad_left(...)
pub(super) fn repeat_char(...)
pub(super) fn format_elapsed(...)
```

Run:

```bash
cargo test --locked picker::render -- --nocapture
```

Expected: PASS.

- [ ] **Step 3: Extract header/footer/candidate/inspect rendering**

Move related functions into the matching submodule:

```rust
// header.rs
pub(super) struct HeaderState { ... }
pub(super) fn header_line(...)

// footer.rs
pub(super) fn query_line(...)
pub(super) fn help_line(...)
pub(super) fn help_overlay_lines(...)
pub(super) fn query_cursor_column(...)

// candidate.rs
pub(super) fn candidate_header_line(...)
pub(super) fn candidate_line(...)

// inspect.rs
pub(super) fn inspect_lines(...)
```

Run:

```bash
cargo test --locked picker::render -- --nocapture
```

Expected: PASS.

---

### Task 3: Extract Picker Keymap And Terminal Key Decoding

**Files:**
- Create: `src/picker/keymap.rs`
- Create: `src/picker/keys.rs`
- Modify: `src/picker/mod.rs`
- Modify: `src/picker/input.rs`
- Modify: `src/picker/session.rs`

- [ ] **Step 1: Add key intent characterization tests**

Create `src/picker/keymap.rs` with tests for these mappings:

```rust
#[test]
fn maps_tab_and_enter_to_picker_intents() { ... }

#[test]
fn maps_navigation_and_editing_keys_to_picker_intents() { ... }
```

Run:

```bash
cargo test --locked picker::keymap -- --nocapture
```

Expected: FAIL because `KeyIntent` and `key_intent` are not wired yet.

- [ ] **Step 2: Implement key intent mapping**

Add:

```rust
pub(super) enum KeyIntent { ... }
pub(super) fn key_intent(key: KeyEvent) -> KeyIntent { ... }
```

Update `src/picker/input.rs` so `handle_picker_key` applies `KeyIntent` instead of matching raw key events directly.

Run:

```bash
cargo test --locked picker::input picker::keymap -- --nocapture
```

Expected: PASS.

- [ ] **Step 3: Extract pure terminal key decoder tests**

Create `src/picker/keys.rs` with tests for:

```rust
#[test]
fn decodes_arrow_escape_sequences() { ... }

#[test]
fn decodes_shift_tab_escape_sequence() { ... }
```

Run:

```bash
cargo test --locked picker::keys -- --nocapture
```

Expected: FAIL until decoder helpers are implemented and session uses them.

- [ ] **Step 4: Implement key decoder helpers and update session**

Move terminal byte decoding from `src/picker/session.rs` into `src/picker/keys.rs` while keeping behavior identical.

Run:

```bash
cargo test --locked picker::session picker::keys -- --nocapture
```

Expected: PASS.

---

### Task 4: Extract Inline Viewport Helpers

**Files:**
- Create: `src/picker/viewport.rs`
- Modify: `src/picker/mod.rs`
- Modify: `src/picker/session.rs`

- [ ] **Step 1: Add viewport tests**

Test that picker rows are capped by terminal height and that cleanup lines contain erase-line commands.

Run:

```bash
cargo test --locked picker::viewport -- --nocapture
```

Expected: FAIL until the helper module exists.

- [ ] **Step 2: Move row math and terminal line sequences**

Move these helpers from `session.rs` to `viewport.rs`:

```rust
picker_row_count
picker_start_row
move_to_row
erase_line
render_lines
clear_lines
open_picker_space
close_picker_space
```

Run:

```bash
cargo test --locked picker::session picker::viewport -- --nocapture
```

Expected: PASS.

---

### Task 5: Clean CLI Argument And Fixture Helpers

**Files:**
- Create: `src/cli/args.rs`
- Modify: `src/cli/mod.rs`
- Modify: `src/cli/choose.rs`
- Create: `tests/support/mod.rs`
- Create: `tests/support/history.rs`
- Modify: `tests/acceptance.rs`
- Create: `scripts/lib/history-fixture.js`
- Modify: `scripts/capture-screenshots.js`

- [ ] **Step 1: Add argument cursor tests**

Create tests for missing flag values and `--` rest handling in `src/cli/args.rs`.

Run:

```bash
cargo test --locked cli::args -- --nocapture
```

Expected: FAIL until helper is implemented.

- [ ] **Step 2: Implement argument cursor helper**

Add:

```rust
pub(super) struct ArgCursor<'a> { ... }
impl<'a> ArgCursor<'a> {
    pub(super) fn new(args: &'a [String]) -> Self;
    pub(super) fn next(&mut self) -> Option<&'a str>;
    pub(super) fn next_value(&mut self, flag: &str) -> CliResult<&'a str>;
    pub(super) fn rest_joined(&self) -> String;
}
```

Use it in `choose` parsing without changing flags or error text.

Run:

```bash
cargo test --locked cli::choose cli::args -- --nocapture
```

Expected: PASS.

- [ ] **Step 3: Isolate test fixture helpers**

Move TSV fixture encoding from `tests/acceptance.rs` into `tests/support/history.rs`, then update acceptance tests to call `history::record_line`.

Run:

```bash
cargo test --locked --test acceptance -- --nocapture
```

Expected: PASS.

- [ ] **Step 4: Isolate screenshot fixture helpers**

Move `encodeField` and mock-history writing from `scripts/capture-screenshots.js` into `scripts/lib/history-fixture.js`.

Run:

```bash
node scripts/capture-screenshots.js --help
```

Expected: exits successfully or prints the same usage behavior as before.

---

### Task 6: Documentation And Full Verification

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/development.md`
- Possibly modify: `docs/specs/2026-05-23-picker-continuous-improvement.md`

- [ ] **Step 1: Update code map and testing contracts**

Document the new picker submodules, CLI args helper, fixture helper locations, and the rule that behavior changes must cover both inline and fullscreen picker modes.

- [ ] **Step 2: Run complete local verification**

Run:

```bash
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked
scripts/verify-doc-i18n.sh
scripts/verify-picker-modes.sh
```

Expected: all PASS. If any fail, debug root cause before final response.

---

## Self-Review

- Spec coverage: covers the outstanding refactor candidates: renderer split, keymap/session extraction, CLI parser cleanup, acceptance/screenshot fixture cleanup, docs, and verification.
- Placeholder scan: no TBD/TODO/later placeholders; each task names concrete files and commands.
- Type consistency: helper names and module names are consistent across tasks.
