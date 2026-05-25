#!/usr/bin/env bash
# scripts/verify-all.sh
# Runs the full CI/CD verification matrix locally.

set -euo pipefail

# ANSI color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color
BLUE='\033[0;34m'

echo -e "${BLUE}=== Starting Full Local Verification ===${NC}\n"

step() {
  echo -e "${BLUE}[Step] $1...${NC}"
}

success() {
  echo -e "${GREEN}✓ $1 passed!${NC}\n"
}

failure() {
  echo -e "${RED}✗ $1 failed!${NC}"
  exit 1
}

# 1. Cargo Format
step "Checking code formatting (cargo fmt)"
if cargo fmt -- --check; then
  success "Formatting"
else
  failure "Formatting"
fi

# 2. Cargo Clippy
step "Running static analysis (cargo clippy)"
if cargo clippy --all-targets -- -D warnings; then
  success "Clippy"
else
  failure "Clippy"
fi

# 3. Cargo Test
step "Running cargo unit and acceptance tests"
if cargo test; then
  success "Tests"
else
  failure "Tests"
fi

# 4. Documentation i18n Verification
step "Verifying documentation and translations"
if ./scripts/verify-doc-i18n.sh; then
  success "Documentation i18n"
else
  failure "Documentation i18n"
fi

# 5. Interactive PTY Picker Smoke Tests
step "Running PTY picker/widget smoke tests (inline and fullscreen)"
if ./scripts/verify-picker-modes.sh; then
  success "PTY smoke tests"
else
  failure "PTY smoke tests"
fi

echo -e "${GREEN}==========================================${NC}"
echo -e "${GREEN}  All verifications passed successfully! ${NC}"
echo -e "${GREEN}==========================================${NC}"
