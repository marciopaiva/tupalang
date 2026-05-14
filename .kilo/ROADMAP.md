# Tupã Development Roadmap

**Project:** Tupã — Typed Policy & Strategy DSL for Rust  
**Architecture:** Crate-first (Rust crates, not standalone compiler)  
**Target:** v1.0.0 stable release  
**Timeline:** 6–9 months from Phase 1 start

---

## Phase 0 — Baseline ✅ COMPLETE

**Status:** Delivered and frozen.

| Item | Status | Location |
|---|---|---|
| Core subset (syntax + types) | ✅ | `docs/reference/spec.md` §1.5 |
| Minimal formal SPEC | ✅ | `docs/reference/spec.md` (1083 lines) |
| EBNF grammar | ✅ | `docs/reference/grammar.ebnf` |
| Type semantics | ✅ | `docs/reference/type_semantics.md` |
| Conformance suite | ✅ | `crates/tupa-conformance` (29 tests) |
| JSON diagnostics | ✅ | `tupa-cli check --format json` |

**Normative reference:** These artifacts are frozen. No breaking changes without RFC and major version bump.

---

## Phase 1 — Basic Toolchain (Weeks 1–8) ✅ COMPLETE

**Goal:** Publish `tupa-core` 0.9.0 and `tupa-engine` 0.9.0 to crates.io. Provide `cargo tupa` wrapper.

**Completed:**

- ✅ `tupa-core` crate scaffolded with `pipeline!` macro
- ✅ Constraint solver (compile-time constant folding)
- ✅ `tupa-engine` executor with channel-based execution
- ✅ Parallel execution (`@parallel` annotation)
- ✅ Plugin system (`tupa-plugin` crate)
- ✅ `cargo-tupa` CLI (check, run, fmt, lint)
- ✅ Project template (`tupa-template`)
- ✅ Published to crates.io (alpha)

**Sprint 4 (Parallel Execution) completed** — see `docs/en/IMPLEMENTATION_PLAN.md` for details.

---

## Phase 2 — Developer Experience (Weeks 9–20) 🚧 IN PROGRESS

**Goal:** Polish tooling, templates, error messages, and lint rules.

### Timeline

| Week | Task | Status |
|------|------|--------|
| 9–10 | Project template (`cargo generate tupa-template`) | ✅ |
| 11–12 | Stable CLI (`cargo tupa` subcommands) | ✅ |
| 13–16 | Didactic error messages (diagnostic codes, hints) | 🚧 |
| 17–20 | Expand `tupa-lint` rules (10+ warnings) | ⏳ |

**Acceptance criteria:**

- All `pipeline!` macro errors emit `error[E####]` with full spans
- `cargo tupa check --format json` stable
- Linter catches common mistakes (impurity, shadowing, unused metrics)

---

## Phase 3 — Interoperability (Weeks 21–40) ⏳ PLANNED

**Goal:** FFI for C and Python.

### Milestones

| Week | Deliverable | Status |
|------|-------------|--------|
| 21–26 | C ABI + `tupa-sys` crate | ⏳ |
| 27–32 | Python bindings (`tupa-pyffi`) | ⏳ |
| 33–40 | Documentation + examples (C, Python) | ⏳ |

**Deliverables:**

- Opaque C handles (`tupa_pipeline_t`, `tupa_run()`, etc.)
- Python module: `import tupa; pipeline = tupa.Pipeline.from_rust(...)`
- ABI specification document (normative)
- `cbindgen`-generated headers

---

## Phase 4 — Quality and Trust (Weeks 41–80) ⏳ PLANNED

**Goal:** Stabilize API, benchmarks, 6 months production hardening.

### Milestones

| Week | Task | Status |
|------|------|--------|
| 41–48 | Benchmark suite (`criterion`) | ⏳ |
| 49–56 | Regression tracking (CI benchmarks) | ⏳ |
| 57–60 | SemVer lock → 1.0.0-rc.1 | ⏳ |
| 61–80 | Production hardening, docs polish | ⏳ |

**Success metrics for 1.0.0:**

- ✅ All conformance tests passing (29/29)
- ✅ ≥100k downloads on crates.io
- ✅ ViperTrade ≥80% migrated to Rust DSL
- ✅ Zero P0 bugs for 30 days
- ✅ Benchmarks stable (variance <3%)
- ✅ FFI ABI documented and tested (C + Python)
- ✅ All public APIs have `///` docs

---

## Concurrent Ongoing Work (All Phases)

These tasks run in parallel throughout development:

| Task | Frequency | Owner |
|---|---|---|
| Conformance suite maintenance | Every PR | CI |
| Security patches | As needed | Security team |
| Community support | Daily | Maintainers |
| CI/CD upkeep | As needed | Infra |

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Macro API too complex | Medium | High | `tupa-expand --pretty` tool; good error messages |
| Performance < targets | Medium | Medium | Profile early; add Cranelift JIT later |
| FFI ABI instability | Low | High | Freeze ABI at 1.0; use `cbindgen` |
| Community adoption slow | Low | High | Early adopter program; ViperTrade migration |

---

## Post-1.0 Roadmap (Preview)

After stable 1.0 release:

- **Cranelift JIT** — ahead-of-time compilation for hot pipelines
- `tupa-ml` crate — PyTorch/Burn integration for ML inference
- `tupa-onnx` — export pipelines to ONNX format
- **WASM target** — run pipelines in browser/WASI
- **Multi-pipeline DAGs** — compose pipelines into workflows
- **Formal verification** — PRISM/Coq integration for constraint proofs

---

## Decision Gates

Before moving to next phase, the following gates must be signed off:

| Gate | Criteria | Owner |
|---|---|---|
| End Phase 1 | `tupa-core` + `tupa-engine` published; examples compile | Tech lead |
| End Phase 2 | `cargo tupa` stable; error messages didactic; lint ≥10 rules | DX lead |
| End Phase 3 | FFI ABI locked; Python example works; ABI doc published | FFI lead |
| 1.0 RC | 6 months production; zero P0 bugs; API stable | Steering committee |

---

## Implementation Status (Quick Reference)

**Phase 1 — Completed**

| Feature | Status | File |
|---|---|---|
| `pipeline!` macro | ✅ | `crates/tupa-core-macros/src/lib.rs` |
| Constraint solver | ✅ | `crates/tupa-core/src/constraints.rs` |
| Executor (sequential) | ✅ | `crates/tupa-engine/src/lib.rs` |
| Parallel execution | ✅ | `crates/tupa-engine/src/scheduler.rs` |
| Plugin registry | ✅ | `crates/tupa-plugin/src/lib.rs` |
| CLI (`cargo-tupa`) | ✅ | `crates/cargo-tupa/src/main.rs` |
| Project template | ✅ | `crates/tupa-template/` |

**Phase 2 — In Progress (0.9.4 Sprint)**

| Feature | Status | Target |
|---|---|---|
| Diagnostic codes (E####) | 🚧 | Week 16 |
| Expanded lint rules | ⏳ | Week 20 |
| `tupa-expand` tool | ⏳ | Week 18 |
| cargo-tupa subcommands (discover, fmt, lint) | ✅ | Week 13 |
| Engine per-step timeout | ✅ | Week 13 |
| Engine StepMetrics collection | 🚧 | Week 14 |
| Pipeline cancellation | 🚧 | Week 14 |
| Migration guide | 🚧 | Week 15 |
| Plugin tutorials (Rust/Python) | 🏗️ | Week 15 |
| Golden tests | 🚧 | Week 16 |

---

## Sprint Breakdown (Historical)

**Sprint 1 (Weeks 1–2):** `tupa-core` scaffold  
- Crate layout, dependencies, CI
- `pipeline!` macro parsing (syn)
- AST generation + trait impl generation
- Published 0.9.0 to crates.io

**Sprint 2 (Weeks 3–4):** Constraint solver  
- Constraint expression AST
- Constant folding engine
- Compile-time proving + E3002 errors
- Runtime constraint evaluator

**Sprint 3 (Weeks 5–6):** Engine executor  
- Channel-based step execution
- `Executor::run()` implementation
- Constraint evaluation pipeline

**Sprint 4 (Weeks 9–12):** Parallel execution + plugins
- `produces`/`requires` annotations
- Tokio manager-worker scheduler
- Dependency graph (in-degree) scheduling
- `cargo-tupa` CLI
- Plugin system migration

**Sprint 5 (Weeks 13–16):** 0.9.4 release — CLI maturation & engine enhancements
- cargo-tupa `discover`, `fmt`, `lint` subcommands + unit tests
- Engine `Executor::from_env()` with `TUPA_STEP_TIMEOUT` and `TUPA_CHANNEL_CAPACITY`
- Per-step timeout via `tokio::time::timeout`
- StepMetrics collection (timestamps, state)
- Pipeline cancellation (Ctrl+C, `Executor::cancel()`)
- Migration guide (.tp → Rust-DSL)
- Plugin tutorials (Rust + Python) with working examples
- Golden tests for cargo-tupa fmt/lint
- Legacy `.tp` crates permanently removed from workspace

---

## Getting Involved

- **Contributing guide:** `CONTRIBUTING.md`
- **Code of Conduct:** `CODE_OF_CONDUCT.md`
- **Issues:** [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- **Discussions:** [GitHub Discussions](https://github.com/marciopaiva/tupalang/discussions)

Areas needing help:

1. Port more ViperTrade pipelines to Rust DSL (real-world validation)
2. Expand `tupa-lint` rule set
3. Write benchmark suite (`criterion`)
4. FFI implementation (`tupa-sys`, `tupa-pyffi`)

---

*This roadmap is a living document. It is updated as implementation progresses.*
