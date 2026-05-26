#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SITUS_BIN="${SITUS_BIN:-"$ROOT/target/debug/situs"}"

if [[ "${SITUS_VERIFY_NO_RETRY:-0}" != "1" ]]; then
  retries="${SITUS_VERIFY_RETRIES:-3}"
  if ! [[ "$retries" =~ ^[1-9][0-9]*$ ]]; then
    retries=3
  fi
  status=0
  for ((attempt = 1; attempt <= retries; attempt++)); do
    if SITUS_VERIFY_NO_RETRY=1 "$0" "$@"; then
      if ((attempt > 1)); then
        echo "zsh widget verification passed on attempt $attempt/$retries"
      fi
      exit 0
    fi
    status=$?
    if ((attempt < retries)); then
      echo "zsh widget verification failed on attempt $attempt/$retries; retrying" >&2
      sleep "$attempt"
    fi
  done
  exit "$status"
fi

if ! command -v expect >/dev/null 2>&1; then
  echo "error: expect is required for zsh widget verification" >&2
  exit 1
fi

if [[ ! -x "$SITUS_BIN" ]]; then
  cargo build --manifest-path "$ROOT/Cargo.toml"
fi

TMP_BASE="${TMPDIR:-/tmp}"
TMP_BASE="${TMP_BASE%/}"
TMP_ROOT="$(mktemp -d "$TMP_BASE/situs-zsh-widget.XXXXXX")"
trap 'rm -rf "$TMP_ROOT"' EXIT

TAB_MID_DIR="$TMP_ROOT/tab-mid"
TAB_LATEST_DIR="$TMP_ROOT/tab-latest"
PUT_DIR="$TMP_ROOT/put-only"
PUT_START_DIR="$TMP_ROOT/put-start"
ENTER_MID_DIR="$TMP_ROOT/enter-mid"
ENTER_LATEST_DIR="$TMP_ROOT/enter-latest"
mkdir -p "$TAB_MID_DIR" "$TAB_LATEST_DIR" "$PUT_DIR" "$PUT_START_DIR" "$ENTER_MID_DIR" "$ENTER_LATEST_DIR"

TAB_MID_RAN="$TMP_ROOT/tab-mid-ran"
TAB_LATEST_RAN="$TMP_ROOT/tab-latest-ran"
PUT_RAN="$TMP_ROOT/put-ran"
ENTER_MID_RAN="$TMP_ROOT/enter-mid-ran"
ENTER_LATEST_RAN="$TMP_ROOT/enter-latest-ran"
PROMPT_ALIGN_DIR="$TMP_ROOT/prompt-align"
PROMPT_ALIGN_RAN="$TMP_ROOT/prompt-align-ran"
LEGACY_DIR="$TMP_ROOT/legacy-print-selection"
LEGACY_RAN="$TMP_ROOT/legacy-ran"

mkdir -p "$PROMPT_ALIGN_DIR" "$LEGACY_DIR"

HISTORY="$TMP_ROOT/history.tsv"
NOW="$(date +%s)"
OLDER="$((NOW - 1))"

{
  printf 'v1\t%s\t0\t%s\t%s\n' "$OLDER" "$TAB_MID_DIR" "touch $TAB_MID_RAN"
  printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$TAB_LATEST_DIR" "touch $TAB_LATEST_RAN"
} >"$HISTORY"

SITUS_HISTORY="$HISTORY" \
SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
SITUS_PICKER="${SITUS_PICKER:-inline}" \
SITUS_TAB_MID_DIR="$TAB_MID_DIR" \
expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_MODE=stay TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 16 cols 120\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {2/2 results}
send "\033\[A"
send "\t"
expect -re {MAGIC> .*tab-mid-ran}
send "\025"
send "pwd\r"
expect {
  -re "$env(SITUS_TAB_MID_DIR)" {}
  timeout {
    puts stderr "error: Tab did not cd to the selected directory"
    exit 1
  }
}
send "exit\r"
expect eof
EXPECT

if [[ -e "$TAB_MID_RAN" || -e "$TAB_LATEST_RAN" ]]; then
  echo "error: Tab executed a history command; it must only cd and fill BUFFER" >&2
  exit 1
fi

printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$PUT_DIR" "touch $PUT_RAN" >"$HISTORY"

SITUS_HISTORY="$HISTORY" \
SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
SITUS_PICKER="${SITUS_PICKER:-inline}" \
SITUS_PUT_START_DIR="$PUT_START_DIR" \
expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_MODE=stay TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "cd $env(SITUS_PUT_START_DIR)\r"
expect "MAGIC> "
send "stty rows 16 cols 120\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\033y"
expect -re {MAGIC> .*put-ran}
send "\025"
send "pwd\r"
expect {
  -re "$env(SITUS_PUT_START_DIR)" {}
  timeout {
    puts stderr "error: Alt-y changed directories; it must only fill BUFFER"
    exit 1
  }
}
send "exit\r"
expect eof
EXPECT

if [[ -e "$PUT_RAN" ]]; then
  echo "error: Alt-y executed the selected history command" >&2
  exit 1
fi

{
  printf 'v1\t%s\t0\t%s\t%s\n' "$OLDER" "$ENTER_MID_DIR" "touch $ENTER_MID_RAN"
  printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$ENTER_LATEST_DIR" "touch $ENTER_LATEST_RAN"
} >"$HISTORY"

SITUS_HISTORY="$HISTORY" \
SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
SITUS_PICKER="${SITUS_PICKER:-inline}" \
SITUS_ENTER_MID_DIR="$ENTER_MID_DIR" \
expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_MODE=stay TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 16 cols 120\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {2/2 results}
send "\033\[A"
send "\r"
expect "MAGIC> "
send "pwd\r"
expect {
  -re "$env(SITUS_ENTER_MID_DIR)" {}
  timeout {
    puts stderr "error: Enter did not stay in the selected directory"
    exit 1
  }
}
send "exit\r"
expect eof
EXPECT

if [[ ! -e "$ENTER_MID_RAN" ]]; then
  echo "error: Enter did not execute the selected history command" >&2
  exit 1
fi

if [[ -e "$ENTER_LATEST_RAN" ]]; then
  echo "error: Enter executed the wrong selected history command" >&2
  exit 1
fi

printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$PROMPT_ALIGN_DIR" "touch $PROMPT_ALIGN_RAN" >"$HISTORY"

SITUS_HISTORY="$HISTORY" \
SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
SITUS_PICKER="${SITUS_PICKER:-inline}" \
SITUS_PROMPT_ALIGN_DIR="$PROMPT_ALIGN_DIR" \
expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_MODE=stay TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT=$'MAGIC %~\\n> '\r"
expect "> "
send {RPROMPT='RIGHT [%?]'}
send "\r"
expect "> "
send "stty rows 18 cols 120\r"
expect "> "
send "export SITUS_TTY=\$(tty)\r"
expect "> "
send "eval \"\$(situs init zsh)\"\r"
expect "> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\r"
expect {
  -re {MAGIC [^\r\n]*prompt-align[^\r\n]*[\r\n]+> [^\r\n]*RIGHT \[0\]} {}
  timeout {
    puts stderr "error: Enter did not redraw the multiline/right prompt after picker close"
    exit 1
  }
}
send "pwd\r"
expect {
  -re "$env(SITUS_PROMPT_ALIGN_DIR)" {}
  timeout {
    puts stderr "error: prompt alignment smoke test did not stay in the selected directory"
    exit 1
  }
}
send "exit\r"
expect eof
EXPECT

if [[ ! -e "$PROMPT_ALIGN_RAN" ]]; then
  echo "error: prompt alignment smoke test did not execute the selected command" >&2
  exit 1
fi

printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$LEGACY_DIR" "touch $LEGACY_RAN" >"$HISTORY"

SITUS_HISTORY="$HISTORY" \
SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
SITUS_PICKER="${SITUS_PICKER:-inline}" \
SITUS_LEGACY_DIR="$LEGACY_DIR" \
expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_MODE=stay TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 16 cols 120\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "_situs_legacy_widget() {\r"
send "  local cmd=\"\$BUFFER\"\r"
send "  local selected selected_dir selected_cmd\r"
send "  zle -I\r"
send "  selected=\"\$(command situs choose --print-selection --command \"\$cmd\")\"\r"
send "  local situs_choose_status=\$?\r"
send "  if test \$situs_choose_status -eq 0 && test -n \"\$selected\"; then\r"
send "    selected_dir=\"\${selected%%\$'\\n'*}\"\r"
send "    selected_cmd=\"\${selected#*\$'\\n'}\"\r"
send "    BUFFER=\"cd -- \${(q)selected_dir} && \$selected_cmd\"\r"
send "    CURSOR=\${#BUFFER}\r"
send "    zle accept-line\r"
send "  fi\r"
send "  zle redisplay\r"
send "}\r"
expect "MAGIC> "
send "zle -N situs-legacy-widget _situs_legacy_widget\r"
expect "MAGIC> "
send "bindkey '^G' situs-legacy-widget\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\t"
expect "MAGIC> "
send "pwd\r"
expect {
  -re "$env(SITUS_LEGACY_DIR)" {}
  timeout {
    puts stderr "error: legacy --print-selection Tab did not cd"
    exit 1
  }
}
send "exit\r"
expect eof
EXPECT

if [[ -e "$LEGACY_RAN" ]]; then
  echo "error: legacy --print-selection Tab executed the selected command" >&2
  exit 1
fi

echo "zsh widget verification passed"
