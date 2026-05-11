# Implementation Plan: Roadmap to v1.0.0

**Last updated:** 2026-05-10  
**Status:** Phase 0 complete — implementation starting  
**Architecture:** Crate-first (see [PROPOSAL.md](PROPOSAL.md))

## Overview

This document details the concrete implementation tasks, milestones, and acceptance criteria for reaching Tupã v1.0.0 under the crate-first architecture. It is the **execution plan** derived from [adoption_plan.md](governance/adoption_plan.md) and [roadmap.md](releases/roadmap.md).

---

## Phase 0 — Baseline (✅ COMPLETE)

**Status:** Delivered

| Item | Status | Location |
|---|---|---|
| Core subset (syntax + types) | ✅ | `docs/reference/spec.md` §1.5 |
| Minimal formal SPEC | ✅ | `docs/reference/spec.md` (1083 lines) |
| EBNF grammar | ✅ | `docs/reference/grammar.ebnf` |
| Type semantics | ✅ | `docs/reference/type_semantics.md` |
| Conformance suite | ✅ | `crates/tupa-conformance` (29 tests, 100% pass) |
| JSON diagnostics | ✅ | `tupa-cli check --format json` proven |

**Locked:** These artifacts are **normative** and frozen. No breaking changes without RFC and major version bump.

---

## Phase 1 — Basic Toolchain (Weeks 1–8)

**Goal:** Publish `tupa-core` 0.9.0 and `tupa-engine` 0.9.0 to crates.io. Provide `cargo tupa` wrapper.

### Timeline

| Week | Task | Owner | Deliverable |
|------|------|-------|-------------|
| 1–2 | **Scaffold `tupa-core`** | @compiler-team | Crate with `pipeline!` macro skeleton, types (`Safe`, `Tensor` stubs) |
| 3–4 | **Implement constraint solver (compile-time)** | @typecheck-team | Constraint expressions parsed and constant-folded; E3002 errors on unproven constants |
| 5–6 | **Build `tupa-engine` executor** | @runtime-team | Channel-based step executor; `run()` returns `PipelineResult` |
| 7–8 | **Integrate + publish** | @infra | `cargo publish tupa-core 0.9.0`, `tupa-engine 0.9.0` |

### Detailed Tasks

#### 1.1 Create `tupa-core` crate

- Add `crates/tupa-core/` directory
- `Cargo.toml` dependencies: `syn`, `quote`, `proc-macro2`, `serde`, `thiserror`
- Define AST for pipeline DSL (internal representation)
- Implement `pipeline!` procedural macro:
  - Parse input: `name: Ident, input: Type, steps: [...], constraints: [...]`
  - Generate struct implementing `Pipeline` trait
  - Wire steps to function calls (resolve by name)
  - Inject compile-time constraint checks (constant folding)
- Define types: `Safe<T, C>`, `Tensor<T, const SHAPE: [usize], const DENSITY: f32>`
- Publish 0.9.0 to crates.io (beta)

**Acceptance:**

```bash
cargo add tupa-core --pre
cargo build  # succeeds
```text

#### 1.2 Create `tupa-engine` crate

- Add `crates/tupa-engine/`
- Dependencies: `tokio`, `serde_json`, `tupa-core`
- Implement `Executor` struct:
  - `new()` — initialize channel pool
  - `run(pipeline, input)` — synchronous execution
  - `run_async(pipeline, input)` — async execution (if pipeline has async steps)
- Constraint evaluator:
  - Collect metric values after each step
  - Compare against constraint thresholds
  - Return `PipelineResult { passed, values, failures }`
- Error types: `StepPanic`, `ConstraintFailed`, `TypeError`
- Publish 0.9.0 to crates.io

**Acceptance:**

```bash
cargo add tupa-engine --pre
cargo test  # unit tests pass
```text

#### 1.3 Create `cargo-tupa` wrapper (optional but planned)

- Binary crate `tupa-cli` will be replaced by `cargo-tupa` subcommand
- Alternatively, extend existing `tupa-cli` to detect Rust DSL and invoke `cargo check`
- Decision: **new crate `cargo-tupa`** to avoid confusion
- Subcommands: `cargo tupa check`, `cargo tupa fmt`, `cargo tupa lint`, `cargo tupa run`

**Acceptance:**

```bash
cargo install cargo-tupa
cargo tupa check --help
```text

#### 1.4 Port 3 example pipelines from ViperTrade

- Select: `fraud_complete.tp`, `credit_decision.tp`, `minimal.tp`
- Convert each to Rust DSL in `examples/` directory
- Validate: compiles, runs, produces same results as `.tp` version

**Acceptance:**

```bash
cargo run --example fraud_complete
cargo run --example credit_decision
```text

---

## Phase 2 — Developer Experience (Weeks 9–20)

**Goal:** Polish tooling, templates, error messages.

### Timeline

| Week | Task | Owner | Deliverable |
|------|------|-------|-------------|
| 9–10 | Project template | @dx-team | `cargo generate tupa-template` scaffold |
| 11–12 | Stable CLI (`cargo tupa`) | @cli-team | `check`, `fmt`, `lint`, `run` subcommands stable |
| 13–16 | Didactic error messages | @compiler-team | Diagnostic codes, spans, hints, `--explain` |
| 17–20 | Expand `tupa-lint` rules | @lint-team | 10+ warnings, all documented |

### Detailed Tasks

#### 2.1 Project template

- Create `tupa-template` crate (use `cargo-generate`)
- Template includes:
  - `Cargo.toml` with `tupa-core`, `tupa-engine` deps
  - Example `pipeline!` in `src/lib.rs`
  - `[[bin]]` target for executable
  - `Makefile` with common tasks
  - GitHub Actions CI (runs `cargo tupa check`)
- Publish to `crates.io` as `tupa-template`

#### 2.2 Stable CLI

- Implement `cargo-tupa` as subcommand via `cargo` plugin mechanism
- Or maintain `tupa-cli` that delegates to `cargo` for Rust projects
- Commands:
  - `cargo tupa check` — validate pipeline DSL (expands macro, typechecks)
  - `cargo tupa fmt` — format `.tp` files only (Rust code uses `cargo fmt`)
  - `cargo tupa lint` — run `tupa-lint` on AST (legacy `.tp` or DSL via expansion)
  - `cargo tupa run` — execute pipeline with JSON input
- All support `--format json|pretty`
- Stable API: no breaking changes in output format

#### 2.3 Error messages

- Ensure all `pipeline!` macro errors emit `error[E####]: ...` format
- Add `--explain E2001` etc. to `cargo tupa`
- Spans point to exact macro location (not expanded code)
- Hints suggest fixes (e.g., "did you mean to import the step function?")

#### 2.4 Expand `tupa-lint`

Current rules (legacy):

- Unused variable
- Shadowed variable
- Missing constraint annotation
- Impure in sensitive context
- Snake/PascalCase checking

New rules for DSL:

- Step function purity verification
- Constraint expression complexity (cyclomatic)
- Metric naming conventions
- Unreachable steps detection

---

## Phase 3 — Interoperability (Weeks 21–40)

**Goal:** FFI for C and Python.

### Timeline

| Week | Task | Owner | Deliverable |
|------|------|-------|-------------|
| 21–26 | C ABI design + implementation | @ffi-team | `tupa-sys` crate (C bindings) |
| 27–32 | Python bindings (`tupa-pyffi`) | @ffi-team | `pip install tupa-pyffi` |
| 33–40 | Documentation + examples | @docs | ABI spec, binding examples |

### Detailed Tasks

#### 3.1 Define C ABI (normative)

- Document in `docs/reference/abi.md`:
  - Opaque handle: `typedef struct tupa_pipeline tupa_pipeline_t;`
  - Functions: `tupa_pipeline_new`, `tupa_pipeline_add_step`, `tupa_pipeline_run`, `tupa_pipeline_free`
  - Calling convention: `extern "C"` on Rust side, `stdcall` on Windows
  - Memory ownership: caller frees returned strings
  - Thread safety: pipelines are `Send` but not `Sync` (per-pipeline)
- Generate C header via `cbindgen`

#### 3.2 Implement `tupa-sys`

- Crate `tupa-sys` with `extern "C"` blocks
- Build script detects platform, sets cfg flags
- Safe Rust wrappers around raw handles (RAII)
- Publish 0.9.0

#### 3.3 Implement `tupa-pyffi` (migrate from legacy)

- Current `tupa-pyffi` depends on `tupa-parser`/`typecheck`
- Refactor to use `tupa-core` + `tupa-engine` instead
- Provide Python module `tupa`:

  ```python
  import tupa
  pipeline = tupa.Pipeline.from_rust("my_pipeline")
  result = pipeline.run(input_data)
  ```

- Publish to PyPI (optional, Phase 3b)

---

## Phase 4 — Quality and Trust (Weeks 41–80)

**Goal:** Stabilize API, benchmarks, 6 months production.

### Timeline

| Week | Task | Owner | Deliverable |
|------|------|-------|-------------|
| 41–48 | Benchmark suite | @perf-team | `criterion` benchmarks for engine, constraint solver |
| 49–56 | Regression tracking | @infra | `cargo bench` baseline, alert on >5% regression |
| 57–60 | SemVer lock | @team | API freeze, 1.0.0-rc.1 tag |
| 61–80 | Production hardening | @all | Bug fixes, docs polish, migration guides |

### Detailed Tasks

#### 4.1 Benchmarks

- Use `criterion`:
  - `bench_step_execution`: single step ~100ns target
  - `bench_constraint_check`: 10 constraints < 1μs
  - `bench_pipeline_throughput`: 1000 steps/sec
- Publish results in `docs/en/reference/benchmarks.md`

#### 4.2 Regression suite

- CI runs `cargo bench` on every PR
- Store benchmarks as artifacts
- Alert if regression >5% vs. baseline

#### 4.3 API freeze

- After 6 months of no breaking changes to `tupa-core`/`tupa-engine` public API:
  - Tag 1.0.0-rc.1
  - Update all examples to 1.0 API
  - Finalize `CHANGELOG.md` for each crate
  - Release candidate period: 2 weeks
- If no critical bugs → 1.0.0 final

#### 4.4 Documentation complete

- All public APIs documented (`///` comments + `docs.rs` rendered)
- Migration guide from 0.x → 1.0 written
- SPEC v1.0 (if minor updates from 0.1)
- Update all ViperTrade references to 1.0 APIs

---

## Concurrent Ongoing Work (All Phases)

These tasks run in parallel:

| Task | Frequency | Owner |
|---|---|---|
| Conformance suite maintenance | Every PR must pass | CI |
| Security patches | As needed | Security team |
| Community support | Daily | Maintainers |
| CI/CD upkeep | As needed | Infra |

---

## Decision Gates

Before moving to next phase:

| Gate | Criteria | Who |
|---|---|---|
| End of Phase 1 | `tupa-core` 0.9.0 + `tupa-engine` 0.9.0 published; examples compile | Tech lead |
| End of Phase 2 | `cargo tupa` stable; error messages didactic; lint rules ≥10 | DX lead |
| End of Phase 3 | FFI ABI locked; Python example works; ABI doc published | FFI lead |
| 1.0 RC | 6 months production; zero P0 bugs; API stable | Steering committee |

---

## Risk Mitigation

| Risk | Mitigation | Owner |
|---|---|---|
| Macro API too complex | Keep expansions readable; `tupa-expand --pretty` tool | Compiler team |
| Performance < targets | Profile early; add Cranelift JIT later if needed | Perf team |

- FFI ABI instability | Freeze ABI at 1.0; use `cbindgen` for reproducible headers | FFI team |
| Community adoption slow | Early adopter program; ViperTrade migration; tutorials | Marketing |

---

## Success Metrics (1.0.0)

- ✅ All conformance tests passing (29/29)
- ✅ `tupa-core` + `tupa-engine` on crates.io with ≥100k downloads
- ✅ ViperTrade running ≥80% of strategies via Rust DSL
- ✅ Zero P0 bugs open for 30 days
- ✅ Benchmarks stable (variance <3%)
- ✅ FFI ABI documented and tested (C + Python)
- ✅ Documentation completeness: all public items have `///` docs

---

## Post-1.0 Roadmap (Preview)

After 1.0:

- **Cranelift JIT** for hot pipelines
- `tupa-ml` crate (PyTorch/Burn integration)
- `tupa-onnx` (export to ONNX)
- WASM target
- Multi-pipeline DAGs
- Formal verification integration

---

## Sprint 4 — Parallel Execution (Weeks 9–12) ✅ COMPLETE

**Goal:** Channel-based parallel step execution with dependency tracking.

**Deliverables:**

| Item | Status | Location |
|---|---|---|
| `produces`/`requires` annotations in `pipeline!` | ✅ | `crates/tupa-core-macros/src/lib.rs` |
| `ParallelPipeline` trait implementation | ✅ | `crates/tupa-engine/src/lib.rs` |
| `Executor::run_parallel` with Tokio manager-worker | ✅ | `crates/tupa-engine/src/lib.rs:71` |
| Dependency graph scheduling (in-degree) | ✅ | `crates/tupa-engine/src/lib.rs:85-102` |
| Constraint evaluation post-execution | ✅ | `crates/tupa-engine/src/lib.rs:192-198` |
| Examples with explicit dependencies | ✅ | `crates/tupa-engine/examples/*.rs` |
| Plugin system migrated to `tupa-engine` | ✅ | `crates/tupa-plugin` |
| `cargo-tupa` CLI (check, run) | ✅ | `crates/cargo-tupa` |
| Project template `tupa-template` | ✅ | `crates/tupa-template` |

**Acceptance:**

```bash
cargo run --example minimal --features parallel
cargo tupa check
cargo tupa run
```text

---

## Appendix: Weekly Sprint Breakdown (Example)

**Sprint 1 (Weeks 1–2): `tupa-core` scaffold**

- Day 1–3: crate layout, dependencies, CI setup
- Day 4–7: `pipeline!` macro parsing (syn)
- Day 8–10: AST generation, trait impl generation
- Day 11–14: Unit tests for macro expansion, publish 0.9.0

### Sprint 2 (Weeks 3–4): Constraint solver

- Day 1–3: Constraint expression AST (`metric("x").gt(5)`)
- Day 4–7: Constant folding engine
- Day 8–10: Compile-time proving + E3002 error
- Day 11–14: Runtime constraint evaluator

...

---

## Change Log

| Date | Change |
|---|---|
| 2026-05-10 | Initial version — Phase 0 complete, Phase 1 planning |
| 2026-05-11 | Sprint 4 complete: parallel execution, plugin migration, cargo-tupa CLI |

---

*This plan is living. Update as implementation progresses.*
