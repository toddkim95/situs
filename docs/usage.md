# Usage

Situs is designed around one interactive workflow:

1. Type the command you want to run.
2. Press the Situs key binding.
3. Pick the directory where that command previously worked.
4. Press `Tab` to stage it or `Enter` to run it.

## Example

First run:

```sh
cd ~/work/app
cargo install --path . --force
```

Later, from another directory:

```sh
cargo install
# press Ctrl-G
```

Situs can show previous commands such as:

```text
cargo install --path . --force    .../work/app    ok    15m ago
```

Press:

- `Tab` to cd to `~/work/app` and leave the command in the shell buffer
- `Enter` to cd to `~/work/app` and run the selected history command

## Matching

Situs first tries to match the command you typed. If there is no exact match,
it broadens matching so partial commands can still find useful history.

For example, searching for:

```sh
cargo install
```

can find:

```sh
cargo install --path . --force
```

## Direct CLI Usage

Open the picker directly:

```sh
situs choose --command "cargo build"
```

Run from the selected directory:

```sh
situs run -- cargo build
```

Print only the selected directory:

```sh
situs choose --print-dir --command "cargo build"
```

Use a context filter:

```sh
situs choose --context workspace --command "cargo test"
situs choose --context directory --command "npm test"
```

Include failed commands:

```sh
situs choose --include-failed --command "cargo build"
```

## Execution Modes

The zsh integration supports two modes:

- `stay`: `cd` to the selected directory and stay there after the command runs.
- `restore`: run from the selected directory, then return to the original
  directory.

Set restore mode:

```sh
export SITUS_MODE=restore
eval "$(situs init zsh)"
```

Direct CLI calls can pass:

```sh
situs choose --mode restore --command "cargo build"
```

## Picker Controls

| Key | Action |
| --- | --- |
| `Up` / `Down` | Select history rows and sync the query with the selected command |
| `PageUp` / `PageDown` | Jump through history rows |
| `Left` / `Right` | Move the bottom query cursor |
| `Home` / `End` | Jump to query start or end |
| `Ctrl-A` / `Ctrl-E` | Jump to query start or end |
| `Tab` | cd to selected directory and keep command in zsh |
| `Enter` | cd to selected directory and run selected command |
| `Ctrl-F` | Show or hide failed history |
| `Ctrl-O` | Inspect selected history |
| `Ctrl-/` or `F1` | Toggle help overlay |
| `F2` | Cycle source filter |
| `F3` | Cycle context filter |
| `Ctrl-Y` | Copy selected command |
| `Ctrl-D` | Delete selected Situs history row |
| `Ctrl-U` | Clear query |
| `Esc` / `Ctrl-C` | Quit |

`situs keymap` prints the current shortcut summary.
