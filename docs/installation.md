# Installation

Situs is currently installed with cargo.

## Requirements

- Rust stable and cargo
- zsh for shell integration
- A terminal that supports interactive TUI input

For development and smoke tests:

- `expect`
- `pbcopy` on macOS for copy verification, when available

## Install From GitHub

After this repository is public:

```sh
cargo install --git https://github.com/geonwookim/situs-cli
```

If the repository is published under a different owner or name, replace the URL
with the final GitHub URL.

## Install From Source

```sh
git clone https://github.com/geonwookim/situs-cli
cd situs-cli
cargo install --path .
```

The binary is named `situs`.

## zsh Setup

Add this near the end of `~/.zshrc`:

```sh
eval "$(situs init zsh)"
```

Open a new terminal, or run that line once in the current shell.

The generated integration:

- records interactive command history
- binds Situs to `Ctrl-G` by default
- opens the picker using the current shell buffer as the query
- stages or executes the selected command through zle

## Guided Setup

```sh
situs setup
```

The setup flow can write Situs's config file and help choose picker mode and
Atuin auto-sync behavior.

## Key Binding

Set `SITUS_BINDKEY` before loading the zsh integration:

```sh
export SITUS_BINDKEY='^G'
eval "$(situs init zsh)"
```

`Ctrl-G` is the default because it is easy to press with the left hand and is
less likely to conflict than `Ctrl-R`.

## Upgrade

From a Git checkout:

```sh
git pull
cargo install --path . --force
```

From GitHub:

```sh
cargo install --git https://github.com/geonwookim/situs-cli --force
```

## Uninstall

```sh
cargo uninstall situs-cli
```

Remove this line from `~/.zshrc`:

```sh
eval "$(situs init zsh)"
```

Optional data cleanup:

```sh
rm -r ~/.local/share/situs-cli
rm -r ~/.config/situs-cli
```

Only remove those directories if you no longer need Situs history or config.
