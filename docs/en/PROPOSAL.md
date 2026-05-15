# Proposal: Tupã as Rust Crates (Crate-First Architecture)

## Executive Summary

**Strategic pivot:** Transition Tupã from a standalone compiled language to a **set of Rust crates** providing a typed policy/strategy DSL, with the standalone CLI as a deprecated secondary artifact.

**Rationale:** Faster adoption, lower integration friction, leverage Rust ecosystem, reach v1.0.0 in 4–6 months instead of years.

> **Update (2026-05-14):** The standalone `.tp` compiler and crates (`tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-cli`, `tupa-runtime` [old], `tupa-fmt`, `tupa-lint`, `tupa-audit`, `tupa-conformance`, `tupa-lsp`) were **removed** in v0.9.0. The current active crates are `tupa-core`, `tupa-core-macros`, `tupa-engine`, `tupa-plugin`, `tupa-pyffi`, and `cargo-tupa`. See [ARCHITECTURE.md](./ARCHITECTURE.md) for the current crate map.

---

## Current State (Standalone Language)

```text
┌─────────────────────────────────────────┐
│  Tupã Standalone Compiler                │
│  (.tp → LLVM IR → binary)                │
├─────────────────────────────────────────┤
│  tupa-parser  → AST                     │
│  tupa-typecheck → Typed AST             │
│  tupa-codegen → LLVM IR                 │
│  tupa-cli     → user-facing binary      │
│  tupa-runtime → execution engine        │
│  tupa-conformance → test suite          │
└─────────────────────────────────────────┘
```text

**Pain points:**

- Users must install separate binary/toolchain
- IDE support requires custom LSP (not built yet)
- Cargo integration is indirect (embedding crates only)
- Debugging spans two worlds (Rust host + Tupã code)
- Slow iteration on language features without recompiling whole compiler

---

## Proposed State (Rust Crate Ecosystem)

```text
┌──────────────────────────────────────────────────────────┐
│  Rust developer workflow (cargo build / rust-analyzer)  │
├──────────────────────────────────────────────────────────┤
│  tupa-core         ← DSL macros + core types (Safe, Tensor) │
│  tupa-core-macros  ← procedural macro implementation        │
│  tupa-engine       ← executor (channels, constraints, metrics) │
│  tupa-plugin       ← dynamic step function loading          │
│  tupa-pyffi        ← Python bindings (PyO3)                 │
│  cargo-tupa        ← CLI: check/run/fmt/lint                │
└──────────────────────────────────────────────────────────┘
```text

**Key insight:** The compiler *is* the Rust compiler. The Tupã DSL expands to Rust types and functions that the Rust compiler checks.

---

## Core Concepts (Crate-First)

### 1. `tupa-core` — The DSL Crate

**Purpose:** Provide proc-macros and types that turn Rust code into typed policy pipelines.

```rust
// Cargo.toml
[dependencies]
tupa-core = "1.0"
tupa-engine = "1.0"

// src/main.rs or src/strategy.rs
use tupa_core::{pipeline, step, constraint, Safe, Tensor};

tupa_core::Pipeline::new()
    .input::<MarketEvent>()
    .step("filter", |event| filter_noise(event))
    .step("score", |event| risk_score(event))
    .constraint("max_drawdown").le(0.15)
    .constraint("sharpe").ge(1.5)
    .build()
    .unwrap();

// Or using the macro (preferred)
pipeline! {
    input: MarketEvent,
    steps: [
        step("filter") { filter_noise(input) },
        step("score") { risk_score(input) }
    ],
    constraints: [
        metric("max_drawdown").le(0.15),
        metric("sharpe").ge(1.5)
    ]
}
```text

**What it provides:**

- `pipeline!` macro: parses DSL at compile time, expands to Rust structs impls
- Types: `Safe<T, !nan>`, `Tensor<T, shape=[...], density=d>` (zero-cost wrappers)
- Traits: `Pipeline`, `Step`, `Constraint`
- Compile-time verification: constraints proven or error; purity analysis for `∇`
- Gradient support: `grad!(f)` generates backward pass

### 2. `tupa-engine` — Execution Runtime

**Purpose:** Execute compiled pipelines with channels, deterministic scheduling, and constraint guards.

```rust
use tupa_engine::{Engine, Executor};

let engine = Engine::new();
let plan = compile_pipeline!(...);  // from tupa-core macro
let result = engine.run(plan, input).await?;
```text

Features:

- Channel-based concurrency (ownership-based, no data races)
- Deterministic step ordering (unless `@async` used)
- Constraint guards: runtime fallback when compile-time proof impossible
- Plugin loading (`tupa-plugin` integration)

### 3. `tupa-runtime` — Base Types & Primitives

**Purpose:** Shared types used across all crates.

```rust
pub mod types {
    pub struct Safe<T, const C: [Constraint]>(T);
    pub struct Tensor<T, const SHAPE: [usize], const DENSITY: f32>(...);
    pub struct Channel<T>(...);
}
```text

### 4. `tupa-audit`, `tupa-fmt`, `tupa-lint` — Already exist, integrate

**Already implemented:** These crates work and just need API alignment to new DSL.

### 5. `tupa-lsp` — New (Optional)

**Purpose:** Editor support for `.tp` files (if we keep standalone files) and Rust DSL.

Alternative: Since Rust macros are expanded, rust-analyzer already understands the end Rust code. LSP may be unnecessary for DSL users. Keep only for legacy `.tp` file support (deprecated path).

### 6. `tupa-conformance` — SPEC Validator (Standalone)

**Keep as binary:** Validates that `spec.md` + `grammar.ebnf` + `type_semantics.md` match implementation. Publishes to CI only.

---

## Deliverables by Phase (Adapted)

### Phase 0 — Minimal Core ✅ COMPLETE

**Status:** Already delivered.

- ✅ Core subset defined (spec + grammar + type semantics)
- ✅ Minimal formal specification (`docs/reference/spec.md`, `grammar.ebnf`, `type_semantics.md`)
- ✅ Conformance test suite (`tupa-conformance` — 29 tests pass)
- ✅ JSON diagnostics (`tupa-cli check --format json`)

**This remains the normative reference** for behavior, independent of implementation strategy.

---

### Phase 1 — Basic Toolchain (3–4 semanas)

| Deliverable | Current | Gap |
|---|---|---|
| Official formatter | `tupa-fmt` exists | Integrate into `cargo tupa-fmt` command |
| Linter with minimal rules | `tupa-lint` exists | Integrate into `cargo tupa-lint`, expand rule set |
| Language server | ❌ | Optional — build `tupa-lsp` for `.tp` files only |

**Work:**

- Add `cargo-tupa` wrapper command (or subcommands via `tupa-cli` still)
- Publish crates to crates.io
- Basic LSP: completion + diagnostics for `.tp` (not for DSL)

---

### Phase 2 — Developer Experience (4–6 semanas)

| Deliverable | Plan |
|---|---|
| Project templates | `cargo generate tupa-template` (binary + library) |
| Stable CLI | `cargo tupa` subcommands: `build`, `run`, `fmt`, `check`, `lint` |
| Didactic error messages | Use Rust's `std::error::Error` + custom `Diagnostic` type with codes, hints, spans |

**Note:** CLI stability means that `tupa check file.tp --format json` is stable. DSL-first users may not need CLI at all.

---

### Phase 3 — Interoperability (6–8 semanas)

| Deliverable | Current | Plan |
|---|---|---|
| FFI C/Rust | Spec exists, not implemented | Implement `extern "C"` ABI in `tupa-runtime` + build scripts |
| Documented ABI | Spec sec. 7.2 | Publish ABI doc + `cbindgen` generated headers |
| Minimal bindings | None | Generate `tupa-sys` crate (C bindings) + Python bindings via `tupa-pyffi` |

---

### Phase 4 — Quality and Trust (ongoing)

| Deliverable | Plan |
|---|---|
| Public benchmarks | `criterion` suite for engine throughput, memory, constraint check time |
| Regression tests | Continuous performance tracking via `cargo bench` + CI |
| Versioning policy | Semantic Versioning 2.0 — breaking changes in major, features in minor |
| 1.0.0 criteria | API freeze after Fase 3 + 6 months of production use in ViperTrade |

---

## Migration Path (Standalone → Crates)

**Timeline:** 4–6 months parallel development, then deprecation.

1. **Month 1–2:** Build `tupa-core` (DSL macro + type system)
   - Keep `.tp` parser for backwards compatibility
   - Implement import: `tupa-core::parse_file("strategy.tp") → Pipeline AST`
   - Build `tupa-engine` executor

2. **Month 3–4:** Polish integration
   - `cargo tupa` command as wrapper over CLI
   - VS Code extension (uses `tupa-lsp` if we build it)
   - Migrate ViperTrade strategy layer to `tupa-core` DSL

3. **Month 5–6:** Deprecate standalone compilation
   - Mark `tupa-codegen` as deprecated
   - Binary `tupa` becomes thin wrapper over cargo subcommands
   - Release `tupa-core`/`tupa-engine` 1.0.0 on crates.io

4. **Post-1.0**
   - Drop LLVM codegen (use interpreted engine or JIT via `cranelift`?)
   - Keep compiler as library only

---

## Impact on Existing Users

| User Type | Current | Migration |
|---|---|---|
| ViperTrade | Uses `tupa-cli` + `.tp` files | Gradual migration to Rust DSL; support both for 1 year |
| Embedded users | `tupa-parser` + `tupa-typecheck` crates | Already using Rust; migrate to `tupa-core` API (minimal) |
| New users | Learn `.tp` syntax | Write Rust with `tupa-core` macros — no new language to learn |

---

## Decision Matrix

| Criteria | Standalone Language | Crate-First |
|---|---|---|
| Time to 1.0.0 | 2+ years | 4–6 months |
| IDE support | Custom LSP required | rust-analyzer out of the box |
| Learning curve | Learn `.tp` syntax | Write Rust (familiar to target audience) |
| Interoperability | FFI bridge required | Direct Rust calls |
| Performance | Native binary (LLVM) | Interpreted/JIT (can add Cranelift later) |
| Ecosystem | Isolated | Cargo/crates.io integration |
| Maintainability | Separate compiler codebase | Smaller scope (macros + engine) |

---

## Recommendation

**Adopt crate-first architecture.**

**Why:**

1. **Market fit:** Target audience (Rust engineers in trading/risk/AI) already knows Rust
2. **Velocity:** Reuse Rust compiler frontend; focus on policy DSL innovations
3. **Quality:** Leverage Rust's borrow checker, traits, generics instead of reinventing type system
4. **Adoption:** `cargo add tupa-core` vs "install Tupã toolchain"
5. **1.0 feasible:** Core innovation (Safe types, constraint checking, deterministic pipelines) delivered via crates within 6 months

**What we keep:**

- SPEC v1 (normative reference) — still useful even if implementation changes
- `tupa-conformance` — validates SPEC compliance
- Type semantics (`Safe`, `Tensor`) — now as Rust types with compile-time checks

**What we deprecate:**

- `.tp` as primary source format (becomes legacy import format)
- `tupa-codegen` / LLVM backend (engine interprets IR or uses Cranelift)
- Standalone binary as primary delivery (still available as `cargo tupa` wrapper)

---

## Next Steps (Immediate)

1. **This week:**
   - Create `TRANSITION.md` with migration examples
   - Update `adoption_plan.md` to reflect Fase 0 complete + new Phases
   - Rewrite `roadmap.md` for crate-first
   - Update `README.md` top-level description

2. **Next sprint (2 weeks):**
   - Scaffold `tupa-core` crate ( macros, AST, trait bounds)
   - Implement proof-of-concept: `pipeline!` macro that expands to executable Rust
   - Prototype constraint solver integration at compile time

3. **Month 1:**
   - MVP `tupa-core` + `tupa-engine` working end-to-end
   - Port 3–5 ViperTrade example pipelines to Rust DSL
   - Decision point: continue or revert

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Macros too complex/opaque | Medium | High | Good error messages; provide `tupa-expand` to show expanded code |
| Performance loss vs LLVM | Medium | Medium | Profile early; add Cranelift JIT later if needed |
| Community rejection of DSL in Rust | Low | High | Surveys, early adopter feedback, gradual migration path |
| Spec drift (impl ≠ spec) | Low | High | Keep `tupa-conformance` as canonical validator |

---

## Conclusion

**Tupã's unique value is the type system and constraint model, not the fact it's a separate language.** By embracing Rust as the host, we accelerate delivery, improve UX, and reach v1.0 faster while preserving the core innovations that make Tupã distinct.

**Decision:** Implement crate-first architecture. Maintain SPEC as normative reference. Deprecate standalone compiler over 12 months.
