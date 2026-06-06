# Tupã Reference Documentation (Crate-First)

> **Note:** Tupã is now distributed primarily as Rust crates (`tupa-core`, `tupa-engine`). This reference covers both the normative SPEC and the crate APIs.

## Normative Specification (Language Semantics)

These documents define the **language semantics** regardless of implementation technology. The crate implementation must behave according to these specs.

- **[SPEC](spec.md)** — Full normative specification, including lexical structure, type system, semantics, EBNF grammar, and diagnostics. **This is the source of truth.**
- **[Grammar (EBNF)](grammar.ebnf)** — Machine-readable grammar extracted from the SPEC. Retained as a historical reference for the original `.tp` grammar.
- **[Type Semantics](type_semantics.md)** — Concise formal summary of type rules, inference, subtyping, effects, and constraint resolution.

**These files are frozen at v0.1** (Phase 0 deliverable) and remain the normative reference for the `pipeline!` DSL semantics.

---

## Crate API Reference

The following crates constitute the **implementation** of the SPEC as a Rust library.

| Crate | Docs | Description |
|---|---|---|
| [`tupa-core`](https://crates.io/crates/tupa-core) | [API docs](https://docs.rs/tupa-core) | DSL macro (`pipeline!`), types (`Safe<T, C>`, `Tensor<T>`), traits |
| [`tupa-core-macros`](https://crates.io/crates/tupa-core-macros) | [API docs](https://docs.rs/tupa-core-macros) | Procedural macro implementation for `pipeline!` (internal) |
| [`tupa-engine`](https://crates.io/crates/tupa-engine) | [API docs](https://docs.rs/tupa-engine) | Pipeline executor, constraint solver, channel scheduler |
| [`tupa-plugin`](https://crates.io/crates/tupa-plugin) | [API docs](https://docs.rs/tupa-plugin) | Dynamic plugin loading for custom step functions |
| [`tupa-pyffi`](https://crates.io/crates/tupa-pyffi) | [API docs](https://docs.rs/tupa-pyffi) | Python bindings (PyO3) — alpha |
| [`tupa-lints`](https://crates.io/crates/tupa-lints) | [API docs](https://docs.rs/tupa-lints) | Lint identifier constants for pipeline quality |
| [`cargo-tupa`](https://crates.io/crates/cargo-tupa) | [API docs](https://docs.rs/cargo-tupa) | Cargo subcommand: `check`, `run`, `fmt`, `lint`, `expand` |

---

## Key Concepts

### Pipeline DSL

```rust
use tupa_core::pipeline;

pipeline! {
    name: MyPolicy,
    input: Trade,
    steps: [
        step("score") { risk_score(input) },
        step("check") { input.size <= MAX }
    ],
    constraints: [
        metric("max_drawdown").le(0.15),
        metric("sharpe").ge(1.5)
    ]
}
```text

Expands to a struct implementing the `Pipeline` trait. Type-checked by rustc.

### safe<T, !constraint>

```rust
use tupa_core::Safe;

let x: Safe<f64, !nan> = Safe::new(3.14);  // Compile-time NaN check
```text

Corresponds to `Safe<T, !c>` in the SPEC (section 3.2.6).

### Tensor<T, shape, density>

```rust
use tupa_core::Tensor;

type Image = Tensor<f32, { shape: [28, 28], density: 1.0 }>;
```text

Corresponds to `Tensor<T, shape=[...], density=d>` in SPEC (section 3.2.5).

### Constraint Metrics

```rust
metric("throughput").ge(1000.0)    // ≥
metric("latency").le(50.0)         // ≤
metric("error_rate").lt(0.01)      // <
metric("profit").gt(0.0)           // >
metric("ratio").eq(1.0)            // ==
```text

Constraints are checked **at compile time** when possible, **at runtime** when values are dynamic.

### Gradient (`∇`)

```rust
use tupa_core::grad;

let f = |x: f64| x * x;
let df = grad!(f);
assert_eq!(df(3.0), (6.0,));  // derivative
```text

Corresponds to `∇f(x)` in SPEC (section 4.2.1). Only works on pure functions.

---

## Conformance

The `tupa-conformance` test suite validates that the crates behave according to the SPEC.

```bash
cargo run -p tupa-conformance
```text

All public crates must pass conformance before release.

---

## Diagnostics

When a pipeline is invalid, the compiler emits Rust errors with Tupã-specific codes:

| Code | Meaning |
|---|---|
| `E2001` | Type mismatch |
| `E3001` | Invalid constraint |
| `E3002` | Cannot prove constraint at compile time |
| `E2005` | Invalid call target (step function not found) |
| `E5001` | Non-exhaustive match |

See [Diagnostics Checklist](../reference/diagnostics_checklist.md) for full list.

---

## Migration from `.tp`

The standalone `.tp` toolchain (including `tupa-cli`) was **removed in 0.9.0**. Port existing `.tp` files manually to the Rust DSL (`pipeline!` macro); there is no automatic conversion tool. See the per-language transition guides under `docs/<lang>/TRANSITION.md`.

---

## Embedding in Non-Rust Hosts

For non-Rust applications, use FFI once `tupa-sys` is ready (Phase 3). Until then, the primary embedding target is Rust.

---

## See Also

- [Getting Started](../guides/getting_started.md) — walkthrough
- [Pipeline Guide](pipeline_guide.md) — full DSL reference
- [Type Semantics](type_semantics.md) — formal guarantees
- [Conformance README](../../crates/tupa-conformance/README.md) — test suite details
