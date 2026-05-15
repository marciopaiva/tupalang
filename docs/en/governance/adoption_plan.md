# Minimum Technical Adoption Plan (Crate-First Architecture)

> **Last updated:** 2026-05-14
> **Status:** Phase 2 (0.9.4) active. Phase 0 & 1 complete.
> **Architecture:** See [PROPOSAL.md](../PROPOSAL.md)

## Purpose

Defines the incremental path to make Tupã production-ready **as a set of Rust crates** (`tupa-core`, `tupa-engine`, `cargo-tupa`, etc.). The standalone `.tp` language implementation was **removed** from the workspace in 0.9.0 in favor of a DSL embedded in Rust via the `pipeline!` macro.

---

## Index

- [Phase 0: Minimal core ✅](#phase-0-minimal-core-complete)
- [Phase 1: Basic toolchain ✅](#phase-1-basic-toolchain-complete)
- [Phase 2: Developer experience 🚧](#phase-2-developer-experience-in-progress)
- [Phase 3: Interoperability ⏳](#phase-3-interoperability-planned)
- [Phase 4: Quality & trust ⏳](#phase-4-quality--trust-planned)
- [Minimum deliverables](#minimum-deliverables)

---

## Phase 0: Minimal core ✅ COMPLETE

**Deliverables (all checked):**

- ✅ Core subset defined (syntax and basic types) — `docs/reference/spec.md` (Rust-DSL)
- ✅ Minimal formal SPEC — normative spec + type semantics
- ✅ Type-checked macro expansion (rustc guarantees)
- ✅ Basic executor (`Executor::run`, `run_parallel`) in `tupa-engine`

**Artifacts:** `docs/reference/spec.md`, `crates/tupa-core/`, `crates/tupa-engine/`

---

## Phase 1: Basic toolchain ✅ COMPLETE

**Delivered (0.9.0 → 0.9.4):**

- ✅ `cargo-tupa` CLI with `check`, `run`, `fmt`, `lint`, `discover`
- ✅ Parallel execution (`produces`/`requires`)
- ✅ Plugin system (`tupa-plugin`)
- ✅ Python bindings (`tupa-pyffi`, alpha)
- ✅ All 6 active crates published to crates.io at 0.9.4

**Success criteria met:**

```bash
cargo add tupa-core tupa-engine
cargo tupa fmt
cargo tupa lint
cargo tupa run --parallel
```

---

## Phase 2: Developer experience 🚧 IN PROGRESS (0.9.4)

**Focus:** CLI maturation, engine enhancements, error messages.

| Feature | Status |
|---|---|
| CLI subcommands (discover, fmt, lint) | ✅ |
| Engine per-step timeout (`TUPA_STEP_TIMEOUT`) | ✅ |
| Engine metrics export (`--metrics-output`) | ✅ |
| Pipeline cancellation (`Executor::cancel()`) | ✅ |
| Diagnostic codes (E####) | 🚧 |
| Expanded lint rules (10+) | ⏳ |
| `tupa-expand` tool | ⏳ |

**Target:** 0.9.5 (Sprint 6, Q3 2026)

---

## Phase 3: Interoperability ⏳ Planned

**Goal:** Stable FFI for C and Python.

| Deliverable | Target |
|---|---|
| C ABI + `tupa-sys` crate | Q4 2026 |
| Python bindings stable (`tupa-pyffi`) | Q4 2026 |
| ABI specification document | Q4 2026 |

---

## Phase 4: Quality & trust ⏳ Planned

**Goal:** API freeze for 1.0.0 candidate (6 months production hardening).

| Deliverable | Target |
|---|---|
| Benchmark suite (`criterion`) | Q1 2027 |
| Regression tracking (CI benchmarks) | Q1 2027 |
| SemVer lock → 1.0.0-rc.1 | Q2 2027 |
| Security audit | Q2 2027 |
| Documentation polish | Q2 2027 |

**Success metrics for 1.0.0:**

- All active crates API stable
- ≥100k downloads on crates.io
- ViperTrade ≥80% migrated to Rust DSL
- Zero P0 bugs for 30 days
- Benchmarks stable (variance <3%)
- FFI ABI documented and tested (C + Python)

---

## Minimum Deliverables (Production-Ready)

- [x] Rust crates published (tupa-core, tupa-engine, tupa-plugin, tupa-pyffi, cargo-tupa)
- [x] SPEC finalized and testable (`docs/reference/spec.md`)
- [x] `cargo tupa` CLI with essential subcommands
- [x] Migration guide from legacy `.tp` to Rust DSL
- [x] ViperTrade integration with real pipelines
- [ ] 1.0.0 API stability guarantee
- [ ] Full audit trail (persistence optional)
- [ ] FFI stable (C + Python)
