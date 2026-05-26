#!/usr/bin/env bash
set -euo pipefail

export SITUS_LANG=en

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SITUS_BIN="${SITUS_BIN:-"$ROOT/target/debug/situs"}"
SITUS_PICKER="${SITUS_PICKER:-inline}"

if [[ "${SITUS_VERIFY_NO_RETRY:-0}" != "1" ]]; then
  retries="${SITUS_VERIFY_RETRIES:-3}"
  if ! [[ "$retries" =~ ^[1-9][0-9]*$ ]]; then
    retries=3
  fi
  status=0
  for ((attempt = 1; attempt <= retries; attempt++)); do
    if SITUS_VERIFY_NO_RETRY=1 "$0" "$@"; then
      if ((attempt > 1)); then
        echo "picker feature verification passed on attempt $attempt/$retries"
      fi
      exit 0
    fi
    status=$?
    if ((attempt < retries)); then
      echo "picker feature verification failed on attempt $attempt/$retries; retrying" >&2
      sleep "$attempt"
    fi
  done
  exit "$status"
fi

if ! command -v expect >/dev/null 2>&1; then
  echo "error: expect is required for picker feature verification" >&2
  exit 1
fi

if [[ ! -x "$SITUS_BIN" ]]; then
  cargo build --manifest-path "$ROOT/Cargo.toml"
fi

TMP_BASE="${TMPDIR:-/tmp}"
TMP_BASE="${TMP_BASE%/}"
TMP_ROOT="$(mktemp -d "$TMP_BASE/situs-picker-features.XXXXXX")"
trap 'rm -rf "$TMP_ROOT"' EXIT

start_zsh_expect() {
  local history="$1"
  shift

  SITUS_HISTORY="$history" \
  SITUS_BIN_DIR="$(dirname "$SITUS_BIN")" \
  SITUS_PICKER="$SITUS_PICKER" \
  "$@"
}

FEATURE_SUCCESS_DIR="$TMP_ROOT/fail-toggle-success"
FEATURE_FAILED_DIR="$TMP_ROOT/fail-toggle-failed"
mkdir -p "$FEATURE_SUCCESS_DIR" "$FEATURE_FAILED_DIR"
HISTORY="$TMP_ROOT/fail-toggle.tsv"
NOW="$(date +%s)"
OLDER="$((NOW - 1))"
{
  printf 'v1\t%s\t0\t%s\t%s\n' "$OLDER" "$FEATURE_SUCCESS_DIR" "touch $TMP_ROOT/success-ran"
  printf 'v1\t%s\t7\t%s\t%s\n' "$NOW" "$FEATURE_FAILED_DIR" "touch $TMP_ROOT/failed-ran"
} >"$HISTORY"

start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\006"
expect -re {2/2 results}
expect -re {EXIT 7}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT

HELP_DIR="$TMP_ROOT/help"
mkdir -p "$HELP_DIR"
HISTORY="$TMP_ROOT/help.tsv"
printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$HELP_DIR" "touch $TMP_ROOT/help-ran" >"$HISTORY"

start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\037"
expect -re {Keyboard}
expect -re {ctrl-y}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT

LOCAL_DIR="$TMP_ROOT/source-local"
ATUIN_DIR="$TMP_ROOT/source-atuin"
mkdir -p "$LOCAL_DIR" "$ATUIN_DIR"
HISTORY="$TMP_ROOT/source.tsv"
{
  printf 'v2\t%s\t0\t%s\t%s\tlocal\n' "$OLDER" "$LOCAL_DIR" "touch $TMP_ROOT/local-ran"
  printf 'v2\t%s\t0\t%s\t%s\tatuin\n' "$NOW" "$ATUIN_DIR" "touch $TMP_ROOT/atuin-ran"
} >"$HISTORY"

start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {2/2 results}
send "\033OQ"
expect -re {source: local}
expect -re {1/1 result}
send "\033OQ"
expect -re {source: atuin}
expect -re {1/1 result}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT

CONTEXT_DIR="$TMP_ROOT/context-current"
CONTEXT_OTHER_DIR="$TMP_ROOT/context-other"
mkdir -p "$CONTEXT_DIR" "$CONTEXT_OTHER_DIR"
HISTORY="$TMP_ROOT/context.tsv"
{
  printf 'v1\t%s\t0\t%s\t%s\n' "$OLDER" "$CONTEXT_OTHER_DIR" "touch $TMP_ROOT/context-other-ran"
  printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$CONTEXT_DIR" "touch $TMP_ROOT/context-current-ran"
} >"$HISTORY"

SITUS_CONTEXT_DIR="$CONTEXT_DIR" start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) SITUS_CONTEXT_DIR=$env(SITUS_CONTEXT_DIR) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "
send "cd -- $env(SITUS_CONTEXT_DIR)\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {2/2 results}
send "\033OR"
expect -re {context: directory}
expect -re {1/1 result}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT

DELETE_DIR="$TMP_ROOT/delete"
mkdir -p "$DELETE_DIR"
HISTORY="$TMP_ROOT/delete.tsv"
printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$DELETE_DIR" "touch $TMP_ROOT/delete-ran" >"$HISTORY"

start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\004"
expect -re {deleted 1 history rows}
expect -re {0/0 results}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT

if grep -q 'delete-ran' "$HISTORY"; then
  echo "error: Ctrl-D did not delete the selected history row" >&2
  exit 1
fi

if command -v pbcopy >/dev/null 2>&1 && printf 'situs-copy-check' | pbcopy >/dev/null 2>&1; then
  COPY_DIR="$TMP_ROOT/copy"
  mkdir -p "$COPY_DIR"
  HISTORY="$TMP_ROOT/copy.tsv"
  printf 'v1\t%s\t0\t%s\t%s\n' "$NOW" "$COPY_DIR" "touch $TMP_ROOT/copy-ran" >"$HISTORY"

  start_zsh_expect "$HISTORY" expect <<'EXPECT'
set timeout 8

spawn env SITUS_HISTORY=$env(SITUS_HISTORY) SITUS_PICKER=$env(SITUS_PICKER) PATH=$env(SITUS_BIN_DIR):$env(PATH) TERM=xterm-256color zsh -f
expect -re {[%#] }
send "PROMPT='MAGIC> '\r"
expect "MAGIC> "
send "stty rows 18 cols 132\r"
expect "MAGIC> "
send "export SITUS_TTY=\$(tty)\r"
expect "MAGIC> "
send "eval \"\$(situs init zsh)\"\r"
expect "MAGIC> "

send "touch"
send "\007"
expect -re {1/1 result}
send "\031"
expect -re {copied command}
send "\033"
expect "MAGIC> "
send "\025exit\r"
expect eof
EXPECT
else
  echo "skipping Ctrl-Y copy verification: pbcopy unavailable"
fi

echo "picker feature verification passed for $SITUS_PICKER"
