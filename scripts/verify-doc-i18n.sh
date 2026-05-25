#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_file() {
  local path="$1"
  if [[ ! -f "$root_dir/$path" ]]; then
    echo "missing required i18n doc: $path" >&2
    exit 1
  fi
}

require_text() {
  local path="$1"
  local text="$2"
  if ! grep -Fq "$text" "$root_dir/$path"; then
    echo "missing '$text' in $path" >&2
    exit 1
  fi
}

require_file "README.md"
require_file "docs/ko/README.md"
require_file "docs/zh-Hans/README.md"
require_file "docs/es/README.md"
require_file "docs/ja/README.md"
require_file "docs/i18n.md"

require_text "README.md" "[한국어](docs/ko/README.md)"
require_text "README.md" "[简体中文](docs/zh-Hans/README.md)"
require_text "README.md" "[Español](docs/es/README.md)"
require_text "README.md" "[日本語](docs/ja/README.md)"

require_text "docs/ko/README.md" "[English](../../README.md)"
require_text "docs/ko/README.md" "[简体中文](../zh-Hans/README.md)"
require_text "docs/ko/README.md" "[Español](../es/README.md)"
require_text "docs/ko/README.md" "[日本語](../ja/README.md)"
require_text "docs/ko/README.md" "source of truth"
require_text "docs/ko/README.md" "command cwd resolver"

require_text "docs/zh-Hans/README.md" "[English](../../README.md)"
require_text "docs/zh-Hans/README.md" "[한국어](../ko/README.md)"
require_text "docs/zh-Hans/README.md" "[Español](../es/README.md)"
require_text "docs/zh-Hans/README.md" "[日本語](../ja/README.md)"
require_text "docs/zh-Hans/README.md" "source of truth"
require_text "docs/zh-Hans/README.md" "command cwd resolver"

require_text "docs/es/README.md" "[English](../../README.md)"
require_text "docs/es/README.md" "[한국어](../ko/README.md)"
require_text "docs/es/README.md" "[简体中文](../zh-Hans/README.md)"
require_text "docs/es/README.md" "[日本語](../ja/README.md)"
require_text "docs/es/README.md" "source of truth"
require_text "docs/es/README.md" "command cwd resolver"

require_text "docs/ja/README.md" "[English](../../README.md)"
require_text "docs/ja/README.md" "[한국어](../ko/README.md)"
require_text "docs/ja/README.md" "[简体中文](../zh-Hans/README.md)"
require_text "docs/ja/README.md" "[Español](../es/README.md)"
require_text "docs/ja/README.md" "source of truth"
require_text "docs/ja/README.md" "command cwd resolver"

echo "README i18n docs verified"
