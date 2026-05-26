# Configuration

Situs uses a small line-based config file plus environment variable overrides.

## Paths

Config path:

```text
~/.config/situs-cli/config
```

History path:

```text
~/.local/share/situs-cli/history.tsv
```

Atuin default database lookup order:

1. `ATUIN_DB`
2. `$XDG_DATA_HOME/atuin/history.db`
3. `~/.local/share/atuin/history.db`

## Config File

Example:

```text
picker_mode=inline
atuin_sync=auto
bindkey=^G
language=en
```

Supported keys:

| Key | Values | Default |
| --- | --- | --- |
| `picker_mode` | `inline`, `fullscreen` | `inline` |
| `atuin_sync` | `off`, `auto`, `always` | `off` |
| `bindkey` | zsh/bash/fish key sequence or `None` | `^G` |
| `language` | `en`, `ko`, `zh-Hans`, `es`, `ja` | system locale, then `en` |

Environment variables win over config values.

## Environment Variables

| Variable | Values | Purpose |
| --- | --- | --- |
| `SITUS_BINDKEY` | zsh key sequence | zsh widget key binding |
| `SITUS_MODE` | `stay`, `restore` | zsh command execution behavior |
| `SITUS_PICKER` | `inline`, `fullscreen` | picker mode override |
| `SITUS_INLINE_ROWS` | positive integer | number of visible inline picker rows |
| `SITUS_HISTORY` | path | history file override |
| `SITUS_CONFIG` | path | config file override |
| `SITUS_ATUIN_SYNC` | `off`, `auto`, `always` | Atuin sync override |
| `SITUS_LANG` | `en`, `ko`, `zh-Hans`, `es`, `ja` | UI language override |
| `SITUS_PLAIN` | any value | force line-based picker |

Language resolution checks `SITUS_LANG`, then configured `language`, then
`LC_ALL`, `LC_MESSAGES`, and `LANG`. English is the fallback. User-facing text
can be localized, but commands, flags, config keys, environment variables,
paths, history source IDs, and widget protocol actions stay machine-readable.

## Picker Mode

Inline mode is the default because it fits the zsh widget workflow: the current
prompt stays visible, candidates render above the fixed bottom query, and the
final line holds compact shortcut help.

Fullscreen mode uses the terminal alternate screen and is useful when you want a
larger history surface:

```sh
situs choose --picker fullscreen --command "cargo test"
```

## Atuin Auto-Sync

Manual import:

```sh
situs import atuin
```

Enable automatic read-only import:

```sh
situs atuin enable
```

Modes:

- `off`: never auto-sync
- `auto`: import only when Atuin's database modified time changed
- `always`: check every time, mostly useful for debugging

Atuin is opened read-only and Situs appends imported records to its own
history, skipping exact duplicates.

## History Format

Situs currently stores local history in TSV at:

```text
~/.local/share/situs-cli/history.tsv
```

The file is intentionally simple while the CLI is young. Future versions may
move to SQLite when history volume or query complexity makes that worthwhile.
