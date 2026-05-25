#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SITUS_BIN="${SITUS_BIN:-"$ROOT/target/debug/situs"}"

for mode in inline fullscreen; do
  echo "verifying picker mode: $mode"
  SITUS_BIN="$SITUS_BIN" SITUS_PICKER="$mode" "$ROOT/scripts/verify-zsh-widget.sh"
  SITUS_BIN="$SITUS_BIN" SITUS_PICKER="$mode" "$ROOT/scripts/verify-picker-features.sh"
done

echo "picker mode verification passed"
