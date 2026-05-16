# Roadmap (Crate-First Architecture)

> **Updated:** 2026-05-10 — reflecting the transition to Rust crates.
> See [PROPOSAL.md](../PROPOSAL.md) for strategic rationale.

## Purpose

This document outlines the delivery sequence for Tupã as a set of Rust crates (`tupa-core`, `tupa-engine`, …) targeting v1.0.0.

---

## Current Status (May 2026)

- **Phase 0 complete:** SPEC finalized, conformance suite (29 tests) green
- **Phase 1 complete:** `tupa-core`, `tupa-engine`, `tupa-plugin`, `tupa-pyffi`, `cargo-tupa` published (0.9.0 → 0.9.4)
- **Active crates:** `tupa-core-macros`, `tupa-core`, `tupa-engine`, `tupa-plugin`, `tupa-pyffi`, `cargo-tupa`
- **Removed crates:** `tupa-parser`, `tupa-typecheck`, `tupa-lexer`, `tupa-codegen`, `tupa-runtime` (old), `tupa-cli`, `tupa-fmt`, `tupa-lint`, `tupa-audit`, `tupa-conformance`, `tupa-effects`, `tupa-sys` — these were part of the legacy `.tp` toolchain and are no longer in the workspace.
- **Legacy:** Standalone `.tp` compilation is no longer supported in the repository. Migration to Rust DSL required.

---

## Milestone Sequence (Completed)

### M1 — Foundation Crates (Weeks 1–4) ✅

**Goal:** Publish `tupa-core` 0.9.0 and `tupa-engine` 0.9.0.

- `pipeline!` macro AST design
- Constraint solver (compile-time constant folding)
- Channel-based executor (sequential + parallel)
- **Delivered:** 0.9.0 released

---

### M2 — CLI Maturation (Weeks 5–12) ✅

**Goal:** `cargo tupa` feels native.

- `cargo-tupa` subcommands: `check`, `run`, `fmt`, `lint`, `discover`
- Parallel execution with `produces`/`requires`
- Plugin system integration
- **Delivered:** 0.9.1 → 0.9.4 released

---

### M3 — Engine Enhancements (Weeks 13–16) ✅ (Current)

**Goal:** Metrics, cancellation, environment config.

- Per-step timeout (`TUPA_STEP_TIMEOUT`)
- StepMetrics collection (timestamps, state)
- Pipeline cancellation (`Executor::cancel()`)
- Channel capacity config (`TUPA_CHANNEL_CAPACITY`)
- `--metrics-output` for `cargo tupa run`
- Integration tests + CI local
- **Target:** 0.9.4 (in progress)

---

### M4 — Stabilization (Months 7–8) ⏳ In Progress

**Goal:** API freeze for 1.0.0 candidate

- No breaking changes in public API
- Complete SPEC-to-implementation traceability
- Security audit of constraint solver
- Performance benchmarks (`criterion`)
- Migration guide 0.x → 1.0
- FFI (C ABI + Python) — Phase 3 deferred to post-1.0

**Release candidate:** `tupa-core 1.0.0-rc.1` (Q4 2026)

---

### M5 — Production Hardening (Month 9–12)

**Goal:** 6 months of production exposure

- ViperTrade runs ≥80% of strategies via `tupa-core` DSL
- Bug bash + regression test additions
- Documentation completeness: all public APIs documented, all examples runnable
- Final ABI lock (no changes to C interface)
- Release `1.0.0`

---

## Versioning Policy

- Crates follow **SemVer 2.0.0**
- `tupa-core` 1.0.0 freezes DSL macro syntax and constraint API
- `tupa-engine` 1.0.0 freezes executor behavior and scheduling guarantees
- Breaking changes require major version bump + migration guide + 6 months deprecation cycle

---

## What Changed vs Old Roadmap

| Old (standalone) | New (crate-first) |
|---|---|
| Build full compiler frontend (parser + typechecker) | Use Rust compiler + proc-macros |
| Codegen to LLVM → native binary | Interpreted/JIT engine (Cranelift future) |
| Custom LSP required | rust-analyzer works out of the box |
| Install `.tp` toolchain | `cargo add tupa-core` |
| `.tp` as primary source | Rust DSL as primary; `.tp` legacy only |
| Years to 1.0 | 4–6 months |

---

## Ongoing (Parallel to Milestones)

- **Conformance suite (`tupa-conformance`):** every PR must pass
- **SPEC updates:** minor clarifications allowed; major changes require RFC
- **Community:** GitHub Discussions, StackOvercast tag `tupalang`, Discord (if needed)
- **Applied reference:** ViperTrade migrations tracked publicly

---

## Post-1.0 Vision

After 1.0:

1. **Ecosystem expansion:**
   - `tupa-burn` integration (GPU tensors via Burn crate)
   - `tupa-onnx` (export pipeline as ONNX graph)
   - `tupa-viz` (pipeline DAG visualization)

2. **Performance:**
   - Optional Cranelift JIT for hot paths
   - SIMD tensor kernels
   - Multi-threaded step execution (where safe)

3. **Language extensions (via crates, not core):**
   - `tupa-stats` — statistical functions
   - `tupa-ml` — pre-built models
   - `tupa-risk` — regulatory rule sets

---

## Decision Gates

Each milestone has a go/no-go decision based on:

- Test coverage ≥90%
- Benchmarks stable (<3% variance)
- Zero P0 bugs open
- at least 1 external contributor has ported a pipeline

---

## Risks

| Risk | Mitigation |
|---|---|
| Macros become too magical | Keep expansion readable; provide `tupa-expand --pretty` |
| Performance lags vs LLVM codegen | Profile early; add optional JIT later |

- Rust experts may find DSL limiting | Favor expressiveness; allow raw Rust closure fallback |
| Community adoption slow | Publish early, engage ViperTrade, write tutorials |

---

## See Also

- [PROPOSAL.md](../PROPOSAL.md) — strategic rationale and architecture
- [TRANSITION.md](../TRANSITION.md) — migration from `.tp` to Rust DSL
- [Adoption Plan](../governance/adoption_plan.md) — delivery milestones to v1.0
