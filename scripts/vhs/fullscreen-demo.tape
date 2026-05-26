Output docs/assets/screenshots/fullscreen-demo.gif

Set Shell zsh
Set FontSize 15
Set Width 1200
Set Height 550
Set Padding 15
Set Theme "Catppuccin Mocha"

# Start the fullscreen picker with mock history
Type "SITUS_HISTORY=scripts/vhs/mock-history.tsv SITUS_LANG=en ./target/debug/situs choose --picker fullscreen --command cargo"
Enter
Sleep 1.5s

# 1. Navigate candidates (Up/Down) to show real-time Inspection Panel update
Down
Sleep 600ms
Down
Sleep 600ms
Up
Sleep 600ms

# 2. Filter candidates by typing
Type " install"
Sleep 1.2s

# 3. Copy candidate (Ctrl-Y)
Ctrl+Y
Sleep 1.5s

# 4. Clear query using Backspace
Backspace 8
Sleep 800ms

# 5. Toggle history source (F2: ESC + OQ)
Ctrl+[
Type "OQ"
Sleep 1s
Ctrl+[
Type "OQ"
Sleep 1s
Ctrl+[
Type "OQ"
Sleep 1s

# 6. Toggle context filter (F3: ESC + OR)
Ctrl+[
Type "OR"
Sleep 1s
Ctrl+[
Type "OR"
Sleep 1s
Ctrl+[
Type "OR"
Sleep 1s

# 7. Open and close Help overlay (Ctrl-/: raw byte 0x1F)
Type "\x1f"
Sleep 1.8s
Type "\x1f"
Sleep 800ms

# 8. Select candidate (Enter)
Enter
Sleep 2s
