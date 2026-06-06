# Frequently Asked Questions (Crate-First)

Updated for the **Rust crates** approach (May 2026).

---

## General

### 1) Is Tupã production-ready?

**Not yet.** We are targeting v1.0.0 release in late 2026. Current crates (`tupa-core`, `tupa-engine`) are beta quality but usable for prototyping and early adopters. ViperTrade runs production workloads using the newer Rust DSL.

If you need stability now, pin to a specific version and expect occasional breaking changes until 1.0.

---

### 2) What is Tupã's main focus?

Tupã provides **deterministic, type-safe policy pipelines** for domains where correctness matters:

- Trading & risk systems (position limits, drawdown controls)
- AI inference orchestration (model selection, safety guards)
- Compliance & fraud detection (explainable decisions)

It is **not** a general-purpose language. Think of it as a **DSL for policy/strategy** embedded in Rust.

---

### 3) How do I start using Tupã?

**For Rust projects:**

```bash
cargo add tupa-core tupa-engine
```text

Then write a `pipeline! { ... }` block. See [Getting Started](../guides/getting_started.md).

**For non-Rust projects:**

- Waiting for FFI (Phase 3, late 2026)
- Or embed a Rust microservice exposing Tupã policies via RPC

---

### 4) Why Rust crates instead of a standalone language?

See [PROPOSAL.md](../en/PROPOSAL.md). Short version:

- Faster adoption (`cargo add` vs new toolchain)
- Immediate IDE support (rust-analyzer)
- Leverage Rust's type system instead of reinventing one
- Reach v1.0 in 4–6 months instead of years

The SPEC remains normative; the implementation just happens to be a Rust library now.

---

### 5) Can I still use `.tp` files?

No. The standalone `.tp` language and its toolchain (`tupa-cli`, `tupa-parser`, `tupa-typecheck`, …) were **removed in 0.9.0**. All development uses the `pipeline!` macro in Rust.

See [TRANSITION.md](../TRANSITION.md) to migrate existing `.tp` pipelines.

---

### 6) Where are the examples?

- **Rust DSL examples:** `examples/` (e.g. `simple_pipeline.rs`, `migration/`) and the `tupa-engine` crate examples (`cargo run -p tupa-engine --example minimal`)
- **Applied:** [ViperTrade strategies](https://github.com/marciopaiva/vipertrade)
- **Spec examples:** [SPEC §10](../reference/spec.md#10-validated-examples)

---

## Technical

### 7) What are `Safe<T, !constraint>` types?

Types that prove at compile time a constraint holds:

```rust
let x: Safe<f64, !nan> = Safe::new(3.14);  // OK
// let y: Safe<f64, !nan> = Safe::new(f64::NAN); // ❌ Compile error
```text

The constraint `!nan` is verified via constant folding or static analysis. If the compiler cannot prove it, you get an error.

See SPEC §3.2.6 and [Type Semantics](../reference/type_semantics.md).

---

### 8) How does the gradient operator (`∇`) work?

In Rust DSL: use `grad!` macro:

```rust
use tupa_core::grad;

let f = |x: f64| x * x;
let df = grad!(f);
let derivative = df(3.0);  // (6.0,)
```text

Only works on **pure** functions (no I/O, no randomness). The macro generates a backward-pass function at compile time via symbolic differentiation.

---

### 9) Are there plans for a language server (LSP)?

**Not needed for Rust DSL** — rust-analyzer already provides:

- Completion inside `pipeline!{}`
- Go to definition for step functions
- Type hints and errors

Since the standalone `.tp` language was removed in 0.9.0, no separate LSP is planned.

---

### 10) How do I debug pipelines?

**Compile-time errors:** Rust compiler messages (with Tupã-specific codes `E2001`, `E3002`, etc.)

**Runtime errors:** `Executor::run()` returns `PipelineResult` with details:

```rust
match result {
    Ok(out) => ...,
    Err(Error::ConstraintFailed { metric, expected, actual }) => ...,
    Err(Error::StepPanic { step, reason }) => ...,
}
```text

**Tracing:** enable `RUST_LOG=tupa_engine=debug` for step-by-step logs.

**Audit trail:** call `tupa_audit::enable()` — hashes each step's AST and output.

---

### 11) How is purity enforced?

Default: steps must be pure (no side effects). The compiler checks:

- No `print!`, `std::fs::read`, network calls
- No `rand()`, `time::now()`
- No mutation of non-local `static mut`

Jit impure: use `#[tupa::side_effects(io)]` attribute — but then that step cannot participate in gradient calculations.

---

### 12) What about performance? Isn't interpreted slower than LLVM?

Currently, `tupa-engine` is interpreted (dispatch via match on step name). Overhead ~100–200ns per step — fine for strategy policy (not HFT).

Future optimizations (Phase 4):

- Cranelift JIT for hot pipelines
- SIMD tensor kernels
- Inline pure step bodies (specialization)

Benchmarks will be published in Q3 2026.

---

## Ecosystem

### 13) Will there be a package registry?

No separate registry — use **crates.io**. All Tupã crates publish there.

ViperTrade strategies remain as source code in your repo (no packaging needed).

---

### 14) How does FFI work?

Phase 3 (Q4 2026) will deliver:

- `tupa-sys` — C-compatible ABI (call pipelines from C, Python, etc.)
- `tupa-pyffi` — Python extension module (pip installable)

Until then, Rust is the only supported host.

---

### 15) What about Python integration?

Python users can call Rust pipelines via `tupa-pyffi` (coming) or via a small Rust wrapper binary that exposes a REST/gRPC endpoint. We are not re-implementing a Python compiler.

---

## Process

### 16) How are SPEC changes proposed?

Open a GitHub Issue with `[RFC]` prefix. Discuss in Discussions. Approved changes require:

- Update `docs/reference/spec.md` (normative)
- Update `docs/reference/grammar.ebnf` if syntax changes
- Add conformance tests covering new/changed behavior
- Update `type_semantics.md` if type rules change

No breaking changes to SPEC without a new major version (v1 → v2).

---

### 17) How are breaking changes communicated?

SemVer policy:

- **Major:** breaking grammar or type system change → new major version (`1.0` → `2.0`)
- **Minor:** backward-compatible feature (new attribute, new constraint operator) → new minor (`0.2` → `0.3`)
- **Patch:** bug fixes, doc updates → new patch (`0.2.1` → `0.2.2`)

Breaking changes in minor version require 6 months deprecation cycle and migration guide.

---

### 18) How do I report bugs or request features?

- Bugs: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues) — include minimal reproducer
- Features: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues) — start in "Ideas" category

---

### 19) Is ViperTrade the only user?

ViperTrade is the primary applied reference, but Tupã is designed for any system needing type-safe policy pipelines (risk engines, ML orchestrators, compliance). We welcome other adopters.

---

### 20) What's the license?

MIT. See [LICENSE](../../LICENSE). You can use Tupã crates in proprietary software.

---

*Last updated 2026-05-10. For architecture details, see [ARCHITECTURE.md](./ARCHITECTURE.md).*
