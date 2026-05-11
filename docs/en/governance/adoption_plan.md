# Minimum Technical Adoption Plan (Crate-First Architecture)

> **Last updated:** 2026-05-10
> **Status:** Phase 0 complete. Phase 1–4 adapted for Rust crate ecosystem.
> **Architecture:** See [PROPOSAL.md](../en/PROPOSAL.md)

## Purpose

This document defines the incremental path to make Tupã production-ready **as a set of Rust crates** (`tupa-core`, `tupa-engine`, etc.). The standalone language implementation is deprecated in favor of a DSL embedded in Rust.

## Index

- [Phase 0: Minimal core ✅](#phase-0-minimal-core-complete)
- [Phase 1: Basic toolchain](#phase-1-basic-toolchain)
- [Phase 2: Developer experience](#phase-2-developer-experience)
- [Phase 3: Interoperability](#phase-3-interoperability)
- [Phase 4: Quality and trust](#phase-4-quality-and-trust)
- [Minimum deliverables](#minimum-deliverables)

---

## Phase 0: Minimal core ✅ COMPLETE

**Deliverables (all checked):**
- ✅ Core subset defined (syntax and basic types) — captured in `docs/reference/spec.md`
- ✅ Minimal formal specification — `spec.md` (human-readable) + `grammar.ebnf` (machine-readable) + `type_semantics.md` (type rules)
- ✅ Conformance test suite — `crates/tupa-conformance` (29 tests, all passing)
- ✅ JSON diagnostics — `tupa-cli check --format json` (proven)

**Artifacts location:**
- `docs/reference/spec.md` — normative specification
- `docs/reference/grammar.ebnf` — EBNF grammar
- `docs/reference/type_semantics.md` — formal type semantics
- `crates/tupa-conformance/` — test runner + manifest

**Status:** Phase 0 is the frozen baseline. All subsequent work must remain compatible with the SPEC.

---

## Phase 1: Basic toolchain (4–6 semanas)

**Objective:** Provide essential tooling as Rust crates, publish to crates.io.

| Deliverable | Current State | Work Needed |
|---|---|---|
| Official formatter | `tupa-fmt` exists (lib) | Create `cargo-tupa` wrapper + `tupa-fmt` CLI binary |
| Linter with minimal rules | `tupa-lint` exists (lib) | Expand rules (10+), integrate with `cargo tupa-lint` |
| Language server | ❌ | Optional: `tupa-lsp` only for legacy `.tp` files (DSL users use rust-analyzer) |

**Success criteria:**
- `cargo install tupa-cli` works (wrapper command)
- `cargo tupa fmt src/strategy.tp` formats file
- `cargo tupa lint src/` returns warnings/errors with codes
- Publish `tupa-core` 0.9.0, `tupa-engine` 0.9.0, `tupa-fmt` 0.9.0, `tupa-lint` 0.9.0 to crates.io

---

## Phase 2: Developer experience (4–8 semanas)

**Objective:** Make Tupã feel native to Rust developers.

| Deliverable | Description |
|---|---|
| Project templates | `cargo generate tupa-template` → scaffold `[lib]` + `[[bin]]` with example pipeline |
| Stable CLI | `cargo tupa` subcommands: `check`, `fmt`, `lint`, `run` (all stable API) |
| Didactic error messages | Diagnostic codes (`E2001`, `E3002`), span info, hints, `--explain` support |

**Quality bar:**
- All error messages include: code, message, file:line:col, suggestion
- JSON output for IDE integration (rust-analyzer compatible)
- Examples: `tupa-core` README has 5+ complete, runnable snippets

---

## Phase 3: Interoperability (6–10 semanas)

**Objective:** Enable FFI and embedding in non-Rust environments.

| Deliverable | Description |
|---|---|
| FFI with C/Rust | Implement `extern "C"` ABI in `tupa-runtime` for `Safe<T>` guards, tensor ops |
| Documented ABI | `docs/reference/abi.md` — C header generated via `cbindgen` |
| Minimal bindings | `tupa-sys` (C bindings crate), `tupa-pyffi` (Python extension module) |

**Milestone:** External languages can call Tupã-compiled pipelines via stable ABI.

---

## Phase 4: Quality and trust (ongoing)

**Objective:** Production readiness guarantees.

| Deliverable | Description |
|---|---|
| Public benchmarks | `cargo bench` suite through `criterion` (engine throughput, constraint solver latency) |
| Regression tests | Benchmarks tracked over time; alerts on >5% regressions |
| Versioning policy | SemVer 2.0: breaking changes in major, features in minor, fixes in patch |

**1.0.0 criteria (cumulative):**
1. Phase 0 ✅ complete
2. Phase 1 ✅ toolchain stable (no breaking changes in 0.x)
3. Phase 2 ✅ templates published, error messages didactic
4. Phase 3 ✅ FFI + ABI locked
5. 6 months of production use in ViperTrade (or equivalent)
6. Zero critical bugs outstanding (> P2)

---

## Minimum deliverables (for 1.0.0 release)

- SPEC v1.0 (normative, stable) — `docs/reference/spec.md`
- Conformance suite (`tupa-conformance`) — all tests pass
- `tupa-core`, `tupa-engine` crates on crates.io (1.0.0)
- CLI: `cargo tupa {check,fmt,lint,run}` stable
- FFI ABI documented + `tupa-sys` C bindings
- Benchmarks + regression tracking
- Migration guide from 0.x to 1.0 (if breaking changes)

---

## Phase Timeline (Estimative)

```
Month 1–2:  Phase 1 (toolchain) — crates published
Month 3–4:  Phase 2 (DX) — templates, polished CLI
Month 5–6:  Phase 3 (FFI) — C/Python bindings
Month 7–8:  Phase 4 (quality) — benchmarks, stabilization
Month 9–10: 1.0.0 release candidate
Month 11–12: Production hardening → 1.0.0
```

**Accelerated path:** 6 months if ViperTrade migrates early and provides feedback.

---

## Relationship to SPEC

The SPEC remains the **single source of truth** for language/DSL semantics, regardless of implementation technology. The crate implementation **must** pass `tupa-conformance` tests to claim SPEC compliance.

**Test oracle:** `cargo run -p tupa-conformance` is the certification gate for every release candidate.

---

## Notes

- This plan supersedes any previous roadmap that assumed a standalone compiled language.
- `.tp` file format is **legacy support only**; new code should use Rust DSL (`tupa-core` macros).
- The `tupa-conformance` test suite validates both the SPEC and the reference implementation (now the crates).
- See [PROPOSAL.md](../en/PROPOSAL.md) for rationale and architecture details.
- See [TRANSITION.md](../en/TRANSITION.md) for migration guide from `.tp` to Rust DSL.
