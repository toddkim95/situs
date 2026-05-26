Output docs/assets/screenshots/inline-demo.gif

Set Shell zsh
Set FontSize 15
Set Width 1000
Set Height 400
Set Padding 15
Set Theme "Catppuccin Mocha"

# Start the inline picker with mock history
Type "SITUS_HISTORY=scripts/vhs/mock-history.tsv SITUS_LANG=en SITUS_INLINE_ROWS=8 ./target/debug/situs choose --picker inline --command cargo"
Enter
Sleep 1.5s

# 1. Navigate candidates (Up/Down)
Down
Sleep 600ms
Down
Sleep 600ms
Up
Sleep 600ms

# 2. Filter candidates by typing
Type " test"
Sleep 1.2s

# 3. Copy candidate (Ctrl-Y)
Ctrl+Y
Sleep 1.5s

# 4. Clear query using Backspace
Backspace 5
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

# 8. Select candidate via PutOnly (Alt-Enter)
Alt+Enter
Sleep 2s
