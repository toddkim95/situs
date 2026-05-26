Output docs/assets/screenshots/inline-demo.gif

Set Shell zsh
Set FontSize 15
Set Width 1000
Set Height 450
Set Padding 15
Set Theme "Catppuccin Mocha"

# Setup shell environment silently
Hide
Type "export PATH=/Users/geonwookim/Desktop/projects/situs/worktrees/feature-vhs-screenshots/target/debug:$PATH"
Enter
Type "export SITUS_HISTORY=scripts/vhs/mock-history.tsv"
Enter
Type "export SITUS_LANG=en"
Enter
Type "eval '$(situs init zsh)'"
Enter
Type "clear"
Enter
Sleep 1s
Show

# 1. Print current working directory
Type "pwd"
Enter
Sleep 1.5s

# 2. Visually explain the widget launch trigger
Type "# Pressing Ctrl-G to launch Situs..."
Sleep 1.8s
Backspace 36
Sleep 300ms

# 3. Trigger Situs widget with Ctrl-G
Ctrl+G
Sleep 1.5s

# 4. Filter list by typing
Type " install"
Sleep 1.2s

# 5. Clear filter query
Backspace 8
Sleep 800ms

# 6. Toggle history source (F2: ESC + OQ)
Ctrl+[
Type "OQ"
Sleep 1s
Ctrl+[
Type "OQ"
Sleep 1s
Ctrl+[
Type "OQ"
Sleep 1s

# 7. Open and close help overlay (Ctrl-/: raw byte 0x1F)
Type "\x1f"
Sleep 1.8s
Type "\x1f"
Sleep 800ms

# 8. Navigate to a command in a different directory (e.g. /Users/me/work/rust-tools)
Down
Sleep 800ms
Down
Sleep 800ms

# 9. Press Enter to cd and run the command automatically!
Enter
Sleep 3s
