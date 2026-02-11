#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_DIR="$REPO_ROOT/examples/expected"
mkdir -p "$EXPECTED_DIR"

# Normalize output similar to tests (remove workspace root)
normalize() {
  local root="$REPO_ROOT"
  sed "s|$root||g" | sed 's|/examples/|examples/|g' | sed 's|^/||'
}

run_and_save_stdout() {
  local out_file="$1"; shift
  echo "Running: cargo run -p tupa-cli -- $*" >&2
  CARGO_TERM_QUIET=true cargo run -q -p tupa-cli -- "$@" | normalize > "$EXPECTED_DIR/$out_file"
  echo "Wrote $EXPECTED_DIR/$out_file" >&2
}

run_and_save_stderr() {
  local out_file="$1"; shift
  echo "Running (expect failure): cargo run -p tupa-cli -- $*" >&2
  if CARGO_TERM_QUIET=true cargo run -q -p tupa-cli -- "$@" 1>/dev/null 2>/dev/null; then
    echo "Command unexpectedly succeeded: $*" >&2
    exit 1
  fi
  # capture stderr
  (CARGO_TERM_QUIET=true cargo run -q -p tupa-cli -- "$@" 2>&1 1>/dev/null || true) | normalize > "$EXPECTED_DIR/$out_file"
  echo "Wrote $EXPECTED_DIR/$out_file" >&2
}

# Positive outputs
run_and_save_stdout lex_hello.txt lex examples/hello.tp
run_and_save_stdout lex_hello.json lex --format json examples/hello.tp
run_and_save_stdout parse_hello.txt parse examples/hello.tp
run_and_save_stdout check_hello.txt check examples/hello.tp
run_and_save_stdout check_hello.json check --format json examples/hello.tp

# Codegen outputs (a representative subset)
run_and_save_stdout codegen_hello.txt codegen examples/hello.tp
run_and_save_stdout codegen_arith.txt codegen examples/arith.tp
run_and_save_stdout codegen_if_match.txt codegen examples/if_match.tp
run_and_save_stdout codegen_for_range.txt codegen examples/for_range.tp
run_and_save_stdout codegen_while.txt codegen examples/while.tp
run_and_save_stdout codegen_match_string.txt codegen examples/match_string.tp
run_and_save_stdout codegen_array_ops.txt codegen examples/array_ops.tp
run_and_save_stdout codegen_bool_print.txt codegen examples/bool_print.tp
run_and_save_stdout codegen_unary_ops.txt codegen examples/unary_ops.tp
run_and_save_stdout codegen_pow_ops.txt codegen examples/pow_ops.tp
run_and_save_stdout codegen_match_expr.txt codegen examples/match_expr.tp
run_and_save_stdout codegen_if_expr.txt codegen examples/if_expr.tp
run_and_save_stdout codegen_return_if_expr.txt codegen examples/return_if_expr.tp
run_and_save_stdout codegen_string_concat.txt codegen examples/string_concat.tp
run_and_save_stdout codegen_string_array_ops.txt codegen examples/string_array_ops.tp
run_and_save_stdout codegen_lambda_basic.txt codegen examples/lambda_basic.tp
run_and_save_stdout codegen_if_unit_expr.txt codegen examples/if_unit_expr.tp

# Negative cases (stderr)
run_and_save_stderr check_invalid_type.txt check examples/invalid_type.tp
run_and_save_stderr check_invalid_type.json check --format json examples/invalid_type.tp
run_and_save_stderr check_invalid_return.txt check examples/invalid_return.tp
run_and_save_stderr check_invalid_call.txt check examples/invalid_call.tp

echo "All goldens updated in $EXPECTED_DIR" >&2
