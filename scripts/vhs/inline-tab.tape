Output docs/assets/screenshots/inline-tab.gif

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

# 4. Select a candidate in a different directory (e.g. /Users/me/work/rust-tools)
Down
Sleep 800ms
Down
Sleep 800ms
Down
Sleep 800ms

# 5. Press Tab to only cd and put the command without running
Tab
Sleep 3s
