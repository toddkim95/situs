# Troubleshooting

Start with:

```sh
situs doctor
```

## `Ctrl-G` Does Nothing

Check that the zsh integration is loaded:

```sh
eval "$(situs init zsh)"
```

Put that line near the end of `~/.zshrc`, then open a new terminal.

Check your binding:

```sh
echo "$SITUS_BINDKEY"
situs keymap
```

If another plugin owns the same key, choose another binding before loading the
integration:

```sh
export SITUS_BINDKEY='^X^G'
eval "$(situs init zsh)"
```

## No History Appears

Run:

```sh
situs stats
```

If the record count is zero, make sure you opened a new interactive zsh after
adding the init line.

You can manually add a test record:

```sh
situs record --cwd "$PWD" --status 0 -- "cargo build"
```

Then try:

```sh
situs choose --command "cargo build"
```

## Atuin Results Do Not Appear

Check status:

```sh
situs atuin status
```

Run a manual import:

```sh
situs import atuin
```

If Situs cannot find the database:

```sh
situs import atuin --db ~/.local/share/atuin/history.db
```

## The Picker Looks Broken

Try the plain picker:

```sh
SITUS_PLAIN=1 situs choose --command "cargo build"
```

If plain mode works, the issue is likely terminal rendering or raw-mode input.
Include your terminal, OS, and shell in the bug report.

## `Tab` Runs The Command

In the zsh widget, `Tab` should only cd and stage the selected command. `Enter`
should run it.

If `Tab` runs the command, include:

- terminal app
- zsh version
- `situs doctor` output
- whether `SITUS_PLAIN` is set
- whether you are calling `situs choose` directly or using the zsh key binding

If you just ran `cargo install --path . --force`, reload the zsh integration in
the already-open terminal:

```sh
source ~/.zshrc
```

or open a new terminal tab. `cargo install` updates the `situs` binary, but it
cannot replace a zsh widget function that is already loaded in the current
shell. Recent Situs binaries also guard the old `--print-selection` widget
protocol so stale widgets cd with a no-op instead of running the selected
command.

The direct CLI cannot change the parent shell's cwd. The cd-only behavior is a
zsh widget feature.

## Direct CLI Does Not Change My Shell Directory

A child process cannot change the parent shell's cwd. Use the zsh integration
for real shell cd behavior:

```sh
eval "$(situs init zsh)"
```

For scripts, use:

```sh
cd -- "$(situs choose --print-dir --command "cargo build")"
```
