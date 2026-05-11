# Roadmap (Crate-First Architecture)

> **Updated:** 2026-05-10 — reflecting the transition to Rust crates.
> See [PROPOSAL.md](./PROPOSAL.md) for strategic rationale.

## Purpose

This document outlines the delivery sequence for Tupã as a set of Rust crates (`tupa-core`, `tupa-engine`, …) targeting v1.0.0.

---

## Current Status (May 2026)

- **Phase 0 complete:** SPEC finalized, conformance suite (29 tests) green
- **Existing crates:** `tupa-fmt`, `tupa-lint`, `tupa-audit`, `tupa-plugin`, `tupa-parser`, `tupa-typecheck`
- **Legacy:** Standalone `.tp` compilation path deprecated (still functional but not primary)

---

## Milestone Sequence

### M1 — Foundation Crates (Month 1–2)

**Goal:** Publish `tupa-core` and `tupa-engine` 0.9.0 to crates.io

| Task | Owner | Target |
|---|---|---|
| Design `pipeline!` macro AST | core team | Week 2 |
| Implement constraint solver at compile time | typecheck team | Week 4 |
| Build executor with channel-based steps | runtime team | Week 6 |

- Integration tests: 5 ViperTrade pipelines ported| QA | Week 8 |

**Deliverable:** `tupa-core = "0.1"`可用

---

### M2 — Toolchain Integration (Month 3–4)

**Goal:** `cargo tupa` feels native

- Create `cargo-tupa` subcommand wrapper (`cargo tupa fmt`, `cargo tupa lint`, `cargo tupa check`)
- Stabilize `tupa-fmt` formatting rules (no breaking changes)
- Expand `tupa-lint` rule set (≥10 warnings, all documented)
- Documentation: Getting started, template, examples all using Rust DSL
- Publish `tupa-core` 0.2.0 + `tupa-engine` 0.2.0

**Success:** `cargo tupa fmt && cargo tupa lint && cargo tupa check` passes on example project

---

### M3 — FFI & Bindings (Month 5–6)

**Goal:** Non-Rust languages can execute Tupã pipelines

| Task | Status |
|---|---|
| Implement C ABI in `tupa-runtime` | planned |
| Generate `tupa-sys` (C bindings crate) | planned |

- Python bindings (`tupa-pyffi`) | planned |
| Document ABI stability guarantees | planned |

**Gate:** External integration test (Python script calls Rust pipeline) passes.

---

### M4 — Stabilization (Month 7–8)

**Goal:** API freeze for 1.0.0 candidate

- No breaking changes in `tupa-core`/`tupa-engine` public API
- Complete spec-to-implementation traceability matrix
- Security audit of constraint solver
- Performance baseline established (benchmarks in `criterion/`)
- Migration guide from 0.x → 1.0 written

**Release candidate:** `tupa-core 1.0.0-rc.1`

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

- [PROPOSAL.md](./PROPOSAL.md) — strategic rationale and architecture
- [TRANSITION.md](./TRANSITION.md) — migration from `.tp` to Rust DSL
- [ADOPTION_PLAN_OLD](../en/governance/adoption_plan.md.old) — previous plan (archived)
