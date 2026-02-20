# Examples

## Purpose

Curated examples reflecting the current state of the parser, typechecker, and codegen.

## Curation and playground

- Use this folder for curated and stable examples.
- Use [examples/playground](playground/README.md) for quick tests and experiments.

## Files

### General

- `hello.tp`: basic syntax (`fn`, `let`, calls).
- `functions.tp`: function types and calls via variables.
- `if_else.tp`: control flow with `if/else if/else`.
- `if_expr.tp`: `if` as an expression.
- `if_unit_expr.tp`: `if` without `else` (unit).
- `return_if_expr.tp`: `return` with `if` as an expression.
- `guards.tp`: `match` with guards.
- `closure_basic.tp`: basic anonymous functions (lambdas).
- `match.tp`: `match` with guards and wildcard.
- `match_guard_if_expr.tp`: `match` with guard using `if` as expression.
- `arrays.tp`: array literals, indexing, and `[T; N]` types.
- `float_array_ops.tp`: arrays of `f64` and indexing.
- `string_array_ops.tp`: arrays of `string` and indexing.
- `loops.tp`: `while` and `for in` with ranges (`..`).
- `types.tp`: type annotations and explicit returns.
- `arith.tp`: arithmetic and basic concatenation.
- `if_match.tp`: `if` and `match` with strings.
- `while.tp`: `while` loop.
- `for_range.tp`: `for` with ranges.
- `break_continue.tp`: loop control.
- `array_ops.tp`: arrays, indexing, and assignment.
- `bool_print.tp`: printing booleans.
- `bool_ops.tp`: `&&`, `||`, `==`, `!=`.
- `unary_ops.tp`: `-` and `!`.
- `pow_ops.tp`: `**` power on `i64`.
- `match_guard.tp`: `match` with guards.
- `match_bind.tp`: identifier binding in `match`.
- `match_expr.tp`: `match` as expression.
- `string_eq.tp`: string equality/inequality.
- `string_concat.tp`: runtime string concatenation.
- `string_plus_eq.tp`: concatenation with `+=`.
- `function_call.tp`: user-defined function call.
- `enum_basic.tp`: basic enum declaration.
- `trait_basic.tp`: basic trait declaration.
- `safe_hate_speech_propagation.tp`: ethical constraint propagation via `Safe` parameter.
- `safe_misinformation_return.tp`: ethical constraint propagation via `Safe` return.
- `safe_misinformation_hate_speech.tp`: propagation with multiple ethical constraints.

### Audit

- `audit/fraud_pipeline.tp`: key validation example with `Safe` and `@safety`.
- `audit/credit_pipeline.tp`: credit pipeline with 3 states and formal proof in <50 lines.

### Negative cases (should fail)

- `invalid_lex_char.tp`: invalid character in lexer.
- `invalid_parse_missing_semicolon.tp`: missing `;` in `let`.
- `invalid_type.tp`: type error in `let`.
- `invalid_return.tp`: missing return in non-`unit` function.
- `invalid_call.tp`: incorrect call arity.
- `invalid_unknown_var.tp`: unknown variable.
- `invalid_unknown_function.tp`: unknown function.
- `invalid_unknown_type.tp`: unknown type.
- `invalid_call_target.tp`: call on non-functional value.
- `invalid_binary_op.tp`: invalid types in binary op.
- `invalid_unary_op.tp`: invalid type in unary op.
- `invalid_break.tp`: `break` outside loop.
- `invalid_continue.tp`: `continue` outside loop.
- `invalid_print_arity.tp`: invalid `print` arity.
- `invalid_match_guard.tp`: invalid type in `match` guard.
- `invalid_match_pattern.tp`: invalid type in `match` pattern.
- `invalid_match_arm_type.tp`: divergent types in `match` arms.
- `invalid_index_type.tp`: invalid index type.
- `invalid_index_base.tp`: indexing on non-array base.
- `invalid_array_mixed.tp`: array with mixed types.
- `invalid_assign_type.tp`: invalid type in assignment.
- `invalid_assign_index_value.tp`: invalid type in indexed assignment.
- `invalid_return_type.tp`: invalid return type.
- `invalid_if_condition.tp`: invalid `if` condition type.
- `invalid_while_condition.tp`: invalid `while` condition type.
- `invalid_for_range_type.tp`: `for in` over invalid type.
- `invalid_range_bounds.tp`: invalid range bounds.
- `invalid_safe_hate_speech.tp`: ethical constraint not proven in `Safe<string, ...>`.
- `invalid_safe_hate_speech_base.tp`: ethical constraint on invalid base.
- `invalid_safe_misinformation.tp`: ethical constraint not proven in `Safe<string, ...>`.
- `invalid_safe_misinformation_base.tp`: ethical constraint on invalid base.
- `invalid_safe_param_base.tp`: invalid constraint in `Safe` parameter.
- `invalid_safe_return_base.tp`: invalid constraint in `Safe` return.

## How to test

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- check examples/match.tp

# JSON output
cargo run -p tupa-cli -- lex --format json examples/hello.tp
cargo run -p tupa-cli -- check --format json examples/hello.tp

# golden outputs (CLI)
cargo test -p tupa-cli -- tests::cli_golden

# examples with errors (should fail)
cargo run -p tupa-cli -- lex examples/invalid_lex_char.tp
cargo run -p tupa-cli -- parse examples/invalid_parse_missing_semicolon.tp
cargo run -p tupa-cli -- check examples/invalid_type.tp
cargo run -p tupa-cli -- check examples/invalid_return.tp
cargo run -p tupa-cli -- check examples/invalid_call.tp
```
