#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

STRICT_LINKS="${CI_LOCAL_STRICT_LINKS:-0}"

run_step() {
  local title="$1"
  shift
  echo -e "${YELLOW}==>${NC} ${title}"
  "$@"
  echo -e "${GREEN}ok${NC} ${title}"
  echo
}

require_cmd() {
  local cmd="$1"
  local hint="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo -e "${RED}missing:${NC} '$cmd' (${hint})" >&2
    exit 1
  fi
}

run_lychee_docs() {
  local -a base=(
    --verbose
    --no-progress
    --scheme http
    --scheme https
    --exclude "^https://img.shields.io/"
    --exclude "^https://github.com/.+\\.wiki\\.git$"
    "**/*.md"
  )

  if lychee --help | grep -q -- "--exclude-mail"; then
    lychee "${base[@]}" --exclude-mail
  else
    lychee "${base[@]}" --exclude "^mailto:"
  fi
}

echo -e "${GREEN}Local CI check (CI + Docs Lint)${NC}"
echo

require_cmd cargo "install Rust/Cargo"
require_cmd markdownlint "npm install -g markdownlint-cli@0.41.0"
require_cmd lychee "cargo install lychee"

run_step "cargo fmt --all --check" cargo fmt --all --check
run_step "cargo clippy --workspace --all-targets -- -D warnings" cargo clippy --workspace --all-targets -- -D warnings
run_step "TUPA_RUN_E2E=0 cargo test --workspace --locked" env TUPA_RUN_E2E=0 cargo test --workspace --locked
run_step "markdownlint **/*.md" markdownlint "**/*.md"
run_step "scripts/docs-parity-check.sh" ./scripts/docs-parity-check.sh

if run_lychee_docs; then
  echo -e "${GREEN}ok${NC} lychee docs links"
else
  if [[ "$STRICT_LINKS" == "1" ]]; then
    echo -e "${RED}fail:${NC} lychee docs links (strict mode)"
    exit 1
  fi
  echo -e "${YELLOW}warn:${NC} lychee docs links found issues (non-strict mode)."
  echo -e "      Use CI_LOCAL_STRICT_LINKS=1 to fail on link issues."
fi

echo

echo -e "${GREEN}All local CI checks passed.${NC}"
