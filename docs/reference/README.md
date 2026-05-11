# Tupã Reference Documentation (Crate-First)

> **Note:** Tupã is now distributed primarily as Rust crates (`tupa-core`, `tupa-engine`). This reference covers both the normative SPEC and the crate APIs.

## Normative Specification (Language Semantics)

These documents define the **language semantics** regardless of implementation technology. The crate implementation must behave according to these specs.

- **[SPEC](spec.md)** — Full normative specification, including lexical structure, type system, semantics, EBNF grammar, and diagnostics. **This is the source of truth.**
- **[Grammar (EBNF)](grammar.ebnf)** — Machine-readable grammar extracted from the SPEC (legacy `.tp` format). Still normative for `.tp` compatibility layer.
- **[Type Semantics](type_semantics.md)** — Concise formal summary of type rules, inference, subtyping, effects, and constraint resolution.

**These files are frozen at v0.1** (Phase 0 deliverable). They serve as the oracle for the `tupa-conformance` test suite.

---

## Crate API Reference

The following crates constitute the **implementation** of the SPEC as a Rust library.

| Crate | Docs | Description |
|---|---|---|
| [`tupa-core`](https://crates.io/crates/tupa-core) | [API docs](https://docs.rs/tupa-core) | DSL macros (`pipeline!`, `grad!`), types (`Safe<T>`, `Tensor`), traits |
| [`tupa-engine`](https://crates.io/crates/tupa-engine) | [API docs](https://docs.rs/tupa-engine) | Pipeline executor, constraint solver, channel scheduler |
| [`tupa-runtime`](https://crates.io/crates/tupa-runtime) | [API docs](https://docs.rs/tupa-runtime) | Low-level runtime primitives (internal) |
| [`tupa-audit`](https://crates.io/crates/tupa-audit) | [API docs](https://docs.rs/tupa-audit) | AST hashing, execution reproducibility |
| [`tupa-plugin`](https://crates.io/crates/tupa-plugin) | [API docs](https://docs.rs/tupa-plugin) | Dynamic plugin loading for custom step functions |
| [`tupa-fmt`](https://crates.io/crates/tupa-fmt) | [API docs](https://docs.rs/tupa-fmt) | Formatter for legacy `.tp` sources |
| [`tupa-lint`](https://crates.io/crates/tupa-lint) | [API docs](https://docs.rs/tupa-lint) | Linter for policy code (unused vars, naming, constraints) |

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
```

Expands to a struct implementing the `Pipeline` trait. Type-checked by rustc.

### safe<T, !constraint>

```rust
use tupa_core::Safe;

let x: Safe<f64, !nan> = Safe::new(3.14);  // Compile-time NaN check
```

Corresponds to `Safe<T, !c>` in the SPEC (section 3.2.6).

### Tensor<T, shape, density>

```rust
use tupa_core::Tensor;

type Image = Tensor<f32, { shape: [28, 28], density: 1.0 }>;
```

Corresponds to `Tensor<T, shape=[...], density=d>` in SPEC (section 3.2.5).

### Constraint Metrics

```rust
metric("throughput").ge(1000.0)    // ≥
metric("latency").le(50.0)         // ≤
metric("error_rate").lt(0.01)      // <
metric("profit").gt(0.0)           // >
metric("ratio").eq(1.0)            // ==
```

Constraints are checked **at compile time** when possible, **at runtime** when values are dynamic.

### Gradient (`∇`)

```rust
use tupa_core::grad;

let f = |x: f64| x * x;
let df = grad!(f);
assert_eq!(df(3.0), (6.0,));  // derivative
```

Corresponds to `∇f(x)` in SPEC (section 4.2.1). Only works on pure functions.

---

## Conformance

The `tupa-conformance` test suite validates that the crates behave according to the SPEC.

```bash
cargo run -p tupa-conformance
```

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

If you have existing `.tp` files, see [TRANSITION.md](../TRANSITION.md). The `tupa-cli` can still parse and check `.tp` files, but **no new features will target that format**.

Recommended path: convert to Rust DSL using `tupa-migrate` (coming soon) or manual port (straightforward mapping).

---

## Embedding in Non-Rust Hosts

For non-Rust applications, use FFI once `tupa-sys` is ready (Phase 3). Until then, the primary embedding target is Rust.

---

## See Also

- [Getting Started](../guides/getting_started.md) — walkthrough
- [Pipeline Guide](pipeline_guide.md) — full DSL reference
- [Type Semantics](type_semantics.md) — formal guarantees
- [Conformance README](../../crates/tupa-conformance/README.md) — test suite details
