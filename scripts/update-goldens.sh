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
run_and_save_stdout audit_hello.txt audit examples/audit_hello.tp --input examples/audit_inputs.json
run_and_save_stdout audit_hello.json audit --format json examples/audit_hello.tp --input examples/audit_inputs.json

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
run_and_save_stderr lex_invalid_char.txt lex examples/invalid_lex_char.tp
run_and_save_stderr lex_invalid_char.json lex --format json examples/invalid_lex_char.tp
run_and_save_stderr parse_invalid_missing_semicolon.txt parse examples/invalid_parse_missing_semicolon.tp
run_and_save_stderr parse_invalid_missing_semicolon.json parse --format json examples/invalid_parse_missing_semicolon.tp
run_and_save_stderr check_invalid_type.txt check examples/invalid_type.tp
run_and_save_stderr check_invalid_type.json check --format json examples/invalid_type.tp
run_and_save_stderr check_invalid_return.txt check examples/invalid_return.tp
run_and_save_stderr check_invalid_return.json check --format json examples/invalid_return.tp
run_and_save_stderr check_invalid_call.txt check examples/invalid_call.tp
run_and_save_stderr check_invalid_call.json check --format json examples/invalid_call.tp
run_and_save_stderr check_invalid_unknown_var.txt check examples/invalid_unknown_var.tp
run_and_save_stderr check_invalid_unknown_var.json check --format json examples/invalid_unknown_var.tp
run_and_save_stderr check_invalid_unknown_function.txt check examples/invalid_unknown_function.tp
run_and_save_stderr check_invalid_unknown_function.json check --format json examples/invalid_unknown_function.tp
run_and_save_stderr check_invalid_unknown_type.txt check examples/invalid_unknown_type.tp
run_and_save_stderr check_invalid_unknown_type.json check --format json examples/invalid_unknown_type.tp
run_and_save_stderr check_invalid_call_target.txt check examples/invalid_call_target.tp
run_and_save_stderr check_invalid_call_target.json check --format json examples/invalid_call_target.tp
run_and_save_stderr check_invalid_binary_op.txt check examples/invalid_binary_op.tp
run_and_save_stderr check_invalid_binary_op.json check --format json examples/invalid_binary_op.tp
run_and_save_stderr check_invalid_unary_op.txt check examples/invalid_unary_op.tp
run_and_save_stderr check_invalid_unary_op.json check --format json examples/invalid_unary_op.tp
run_and_save_stderr check_invalid_break.txt check examples/invalid_break.tp
run_and_save_stderr check_invalid_break.json check --format json examples/invalid_break.tp
run_and_save_stderr check_invalid_continue.txt check examples/invalid_continue.tp
run_and_save_stderr check_invalid_continue.json check --format json examples/invalid_continue.tp
run_and_save_stderr check_invalid_print_arity.txt check examples/invalid_print_arity.tp
run_and_save_stderr check_invalid_print_arity.json check --format json examples/invalid_print_arity.tp
run_and_save_stderr check_invalid_match_guard.txt check examples/invalid_match_guard.tp
run_and_save_stderr check_invalid_match_guard.json check --format json examples/invalid_match_guard.tp
run_and_save_stderr check_invalid_match_pattern.txt check examples/invalid_match_pattern.tp
run_and_save_stderr check_invalid_match_pattern.json check --format json examples/invalid_match_pattern.tp
run_and_save_stderr check_invalid_match_arm_type.txt check examples/invalid_match_arm_type.tp
run_and_save_stderr check_invalid_match_arm_type.json check --format json examples/invalid_match_arm_type.tp
run_and_save_stderr check_invalid_index_type.txt check examples/invalid_index_type.tp
run_and_save_stderr check_invalid_index_type.json check --format json examples/invalid_index_type.tp
run_and_save_stderr check_invalid_array_mixed.txt check examples/invalid_array_mixed.tp
run_and_save_stderr check_invalid_array_mixed.json check --format json examples/invalid_array_mixed.tp
run_and_save_stderr check_invalid_return_type.txt check examples/invalid_return_type.tp
run_and_save_stderr check_invalid_return_type.json check --format json examples/invalid_return_type.tp
run_and_save_stderr check_invalid_assign_type.txt check examples/invalid_assign_type.tp
run_and_save_stderr check_invalid_assign_type.json check --format json examples/invalid_assign_type.tp
run_and_save_stderr check_invalid_while_condition.txt check examples/invalid_while_condition.tp
run_and_save_stderr check_invalid_while_condition.json check --format json examples/invalid_while_condition.tp
run_and_save_stderr check_invalid_for_range_type.txt check examples/invalid_for_range_type.tp
run_and_save_stderr check_invalid_for_range_type.json check --format json examples/invalid_for_range_type.tp
run_and_save_stderr check_invalid_range_bounds.txt check examples/invalid_range_bounds.tp
run_and_save_stderr check_invalid_range_bounds.json check --format json examples/invalid_range_bounds.tp
run_and_save_stderr check_invalid_if_condition.txt check examples/invalid_if_condition.tp
run_and_save_stderr check_invalid_if_condition.json check --format json examples/invalid_if_condition.tp
run_and_save_stderr check_invalid_assign_index_value.txt check examples/invalid_assign_index_value.tp
run_and_save_stderr check_invalid_assign_index_value.json check --format json examples/invalid_assign_index_value.tp
run_and_save_stderr check_invalid_index_base.txt check examples/invalid_index_base.tp
run_and_save_stderr check_invalid_index_base.json check --format json examples/invalid_index_base.tp
run_and_save_stderr check_invalid_safe_param_base.txt check examples/invalid_safe_param_base.tp
run_and_save_stderr check_invalid_safe_param_base.json check --format json examples/invalid_safe_param_base.tp
run_and_save_stderr check_invalid_safe_return_base.txt check examples/invalid_safe_return_base.tp
run_and_save_stderr check_invalid_safe_return_base.json check --format json examples/invalid_safe_return_base.tp

echo "All goldens updated in $EXPECTED_DIR" >&2
