# Competitive CLI Research

## Positioning

Situs is closest to `ctxhist`: both solve "run this command in the directory
where it worked before." It intentionally should not become a full Atuin clone.
The strongest product shape is still a small command cwd resolver with a modern
inline picker.

## Compared Tools

- Atuin: SQLite shell history with cwd, exit code, duration, host/session,
  global/directory/workspace filters, compact/full UI, preview, configurable
  keymaps, and encrypted sync.
  Source: https://docs.atuin.sh/cli/configuration/config/
- McFly: smart history search that ranks by cwd, recent command context,
  frequency, recency, selected-before signals, and exit status.
  Source: https://github.com/cantino/mcfly
- zoxide: directory jumper ranked by frecency, with interactive selection when
  paired with fzf.
  Source: https://zoxide.org/blog/zoxide-fzf-interactive-guide-en/
- fzf: general fuzzy finder with shell key bindings, dynamic reload, source
  switching, and preview windows.
  Source: https://github.com/junegunn/fzf
- HSTR: shell history suggest box with delete and bookmark/favorite workflows.
  Source: https://github.com/dvorka-oss/hstr
- ctxhist: narrow command-history-with-directory tool that uses fzf to re-run
  commands in their original directories.
  Source: https://www.reddit.com/r/commandline/comments/1jmub9z/

## Implemented From This Review

- Context filtering inspired by Atuin: `F3` cycles all/directory/workspace, and
  `situs choose --context all|directory|workspace` works in plain mode too.
- History introspection inspired by Atuin/HSTR: `situs stats` summarizes
  record count, failures, source mix, top commands, and top directories.
- Ranking contract inspired by McFly/zoxide remains frequency/recency/success
  based and is now covered by matcher tests.

## Good Future Candidates

- Favorites/pins: HSTR-style bookmarked commands, likely `Ctrl-P` in picker and
  a small sidecar file. This should rank pinned command+cwd pairs first without
  mutating imported Atuin rows.
- History filters: Atuin-style command/cwd ignore rules in config, useful for
  secrets and noisy AI/tooling commands.
- Preview pane: fzf/Atuin-style long-command preview, probably only in
  fullscreen or inspect mode to keep inline mode quiet.
- Duration tracking: Atuin records duration. Situs could add an optional v3
  history field and zsh hook timing, then show slow commands in inspect.
- Selected-before boost: McFly-style positive feedback when a candidate is
  chosen. This likely belongs in a sidecar selection database.

## Rejected For Now

- Encrypted cross-machine sync: Atuin already owns this well; Situs can
  continue to import/sync read-only from Atuin instead of running its own server.
- Neural ranking: too heavy for Situs's narrow command cwd resolver role.
- Full replacement of shell history: this would weaken Situs's small,
  interoperable posture.
