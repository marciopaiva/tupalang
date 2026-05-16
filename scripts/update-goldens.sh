#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_DIR="${EXPECTED_DIR:-$REPO_ROOT/examples/expected}"
mkdir -p "$EXPECTED_DIR"

export RUSTFLAGS="-Awarnings"

normalize() {
  local root="$REPO_ROOT"
  sed "s|$root||g" | sed 's|/examples/|examples/|g' | sed 's|^/||'
}

run_and_save_stdout() {
  local out_file="$1"; shift
  echo "Running: cargo run -p cargo-tupa -- $*" >&2
  CARGO_TERM_QUIET=true cargo run -q -p cargo-tupa -- "$@" | normalize > "$EXPECTED_DIR/$out_file"
  echo "Wrote $EXPECTED_DIR/$out_file" >&2
}

# Test expand command on Rust pipeline files
run_and_save_stdout expand_simple_pipeline.txt expand --file examples/simple_pipeline.rs

echo "All goldens updated in $EXPECTED_DIR" >&2