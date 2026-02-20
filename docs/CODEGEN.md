# Codegen

## Purpose

Describe the current state of `tupa-codegen` and the `parse -> typecheck -> codegen` flow.

`tupa-codegen` generates a functional LLVM-like textual IR (not full LLVM) covering all MVP features, including anonymous functions (lambdas), function values, print as a built-in, string concatenation, arrays, control flow, and more.

## CLI usage

```bash
cargo run -p tupa-cli -- codegen examples/hello.tp
cargo run -p tupa-cli -- codegen examples/arith.tp
cargo run -p tupa-cli -- codegen examples/array_ops.tp

# JSON output
cargo run -p tupa-cli -- codegen --format json examples/hello.tp

# Pipelines: generate plans with hybrid backend
cargo run -p tupa-cli -- codegen --format llvm examples/pipeline/fraud_complete.tp
# Emits: fraud_complete.ll and fraud_complete.plan.json

# Plan-only
cargo run -p tupa-cli -- codegen --plan-only examples/pipeline/fraud_complete.tp
```

## Current output

Textual IR output (simplified example):

```text
declare i32 @printf(i8*, ...)
@.fmt_int = private unnamed_addr constant [5 x i8] c"%ld\0A\00"

define void @main() {
entry:
  %t0 = alloca i64
  store i64 42, i64* %t0
  ret void
}
```

In JSON, the output comes wrapped in an object:

```json
{
  "codegen": "declare i32 @printf(i8*, ...)\n..."
}
```

## Supported features

- `i64`, `f64`, `bool`, and `string` literals (strings are global constants)
- `let`, `return`, `print` (as a built-in)
- Arithmetic operators and comparisons in `i64` and `f64`
- `if`/`match` (including guards, identifier binding, and `match` on `string` via `strcmp`)
- `while`, `for` over ranges, `break`/`continue`
- Arrays of `i64`, `f64`, and `string`, indexing and assignment
- User-defined functions and anonymous functions (lambdas)
- Function/lambda calls as function values
- Runtime string concatenation
- `+=` for strings (via concatenation)
- Automated golden tests to ensure IR stability
- Pipelines: ExecutionPlan JSON with `steps`, `constraints`, `metrics`, `metric_plans`
- Runtime `tupa run`: executes plan with JSON input and emits report

## Next steps

- Reduce remaining `TODO`s in codegen
- Support more types, closures, and optimizations
- Emit native binaries via `llvm`/`inkwell`
