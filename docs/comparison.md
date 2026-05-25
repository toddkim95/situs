# Comparison

Situs is a **command cwd resolver**. It is adjacent to shell history search
and directory jumping tools, but it is not trying to replace them.

## Atuin

Atuin is a full shell history system. It records rich command context, stores
history in SQLite, supports encrypted sync, and provides a powerful history UI.

Situs is smaller. It can import Atuin history, then focuses on this question:

```text
I already typed a command. Where did this command succeed before?
```

Use Atuin when you want the best general history platform. Use Situs when you
want a cwd-aware resolver for the command already in your prompt.

## McFly

McFly improves shell history search and ranking. It is useful when you want to
find a command from history.

Situs assumes you already know the command shape and need the directory that
made it work.

## zoxide

zoxide is a smarter `cd`. It learns directory frequency and lets you jump to
directories quickly.

Situs jumps through command history instead of directory frequency. The target
directory is selected because a command worked there.

## fzf

fzf is a general-purpose fuzzy finder with strong shell key bindings.

Situs uses a purpose-built picker because it needs domain-specific actions:

- stage selected cwd and command without running
- run selected command from selected cwd
- hide failed runs by default
- inspect command status and source
- keep zsh widget output stable

## HSTR

HSTR is a shell history suggest box for bash and zsh. Situs overlaps in the
interactive history surface, but keeps cwd resolution and action semantics at
the center.

## Positioning

Situs should stay:

- focused
- cwd-aware
- shell-widget friendly
- compatible with Atuin rather than competitive with it
- optimized for "run this command where it worked before"
