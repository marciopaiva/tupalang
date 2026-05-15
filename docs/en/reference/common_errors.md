# Common Errors — Rust-DSL

This document describes frequent errors when writing Tupã pipelines using the `pipeline!` macro and their solutions.

---

## 1) Cannot find macro `pipeline`

**Cause:** `tupa-core` is not in `Cargo.toml` or `use tupa_core::pipeline;` is missing.

**Solution:**

```toml
[dependencies]
tupa-core = "0.9"
tupa-engine = "0.9"
```

```rust
use tupa_core::pipeline;
```

---

## 2) E3002 — Constraint cannot be proven at compile time

**Cause:** A constraint expression is not a compile-time constant.

**Example:**

```rust
pipeline! {
    name: MyPolicy,
    input: Trade,
    steps: [
        step("risk") { calculate_risk(input) }  // non-const function
    ],
    constraints: [
        metric("risk").lt(10.0)  // ❌ cannot prove at compile time
    ]
}
```

**Solution:** Either make the step value a `const fn` or accept runtime-only checking:

```rust
const fn constant_risk() -> f64 { 5.0 }

pipeline! {
    constraints: [
        metric("risk").lt(10.0)  // ✅ provable
    ]
}
```

---

## 3) E3001 — Step function not found

**Cause:** Step body refers to an undefined function.

**Example:**

```rust
pipeline! {
    steps: [
        step("x") { nonexistent_function(input) }  // ❌ not in scope
    ]
}
```

**Solution:** Ensure the function is in scope and has the correct signature:

```rust
fn compute(input: &Input) -> i32 { ... }

pipeline! {
    steps: [
        step("x") { compute(input) }  // ✅
    ]
}
```

---

## 4) Duplicate step name

**Cause:** Two steps share the same identifier.

**Solution:** Rename one of the steps.

---

## 5) Metric reference not produced by any step

**Cause:** `requires` or `constraints` reference a metric name that no step `produces`.

**Solution:** Ensure some step declares `produces ["metric_name"]`.

---

## 6) Type mismatch in step body

**Cause:** Expression in step body doesn't match expected return type (or input type mismatch).

**Solution:** Check function signature matches pipeline expectations.

---

## 7) Missing `pipeline!` name or input

**Cause:** `pipeline!` block missing required `name:` or `input:` fields.

**Solution:** Add both fields:

```rust
pipeline! {
    name: MyPipeline,
    input: MyInputType,
    // ...
}
```

---

## 8) Async step without `#[tokio::main]` or async runtime

**Cause:** Pipeline uses async step functions but main is not async.

**Solution:** Add `#[tokio::main]` to `main` or use synchronous steps.

---

## Finding More Help

- Compiler errors include span info and suggestions — read them carefully.
- Run `cargo tupa lint` to catch common mistakes early.
- See [Diagnostics Checklist](../reference/diagnostics_checklist.md) for a systematic guide.
