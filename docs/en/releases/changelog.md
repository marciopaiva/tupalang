# Changelog

## Purpose

This document records relevant changes per version.

## 0.8.2 (2026-05-08)

- Release theme: extensions system, plugins, and hot reload.
- Planning reference:
  - `.kilo/TUPALANG_EVOLUTION.md`

### Delivered Scope

- **Built-in Functions (Phase 1)**:
  - `tupa::weighted(score, weight, reason)` — weighted score with reason
  - `tupa::warn(reason)` — pass with warning
  - `tupa::pass(reason)` — pure pass with reason
  - `tupa::confirm(observed, consecutive, required, reason)` — consecutive confirmation policy
  - `tupa::cooldown(active, remaining_seconds, reason)` — temporal cooldown block
  - Backward compatibility: un-prefixed calls still work
- **Schema Registry (Phase 2)**:
  - `SchemaRegistry` in `tupa-codegen/src/schema_registry.rs`
  - Versioned schemas with migrations
  - `SchemaDiff` for type evolution
  - Runtime field insertion with deprecation warnings
- **Hot Reload (Phase 2)**:
  - `Runtime::watch_and_reload()` in `tupa-runtime/src/hot_reload.rs`
  - File watching for `.tp` via `notify` crate
  - `Runtime::reload_pipeline()` applies new plan without restart
  - Feature flag: `--features hot-reload`
- **Extension API (Phase 3)**:
  - `TupaExtension` trait in `tupa-runtime/src/extensions.rs`
  - `register()` and `name()` for external project integration
  - ViperTrade implements `ViperExtensions` in `vipertrade/services/strategy/src/tupa_extensions.rs`
  - `viper_smart_copy.tp` updated to use `tupa::` prefix
- **Plugin System (Phase 4)**:
  - `tupa-plugin` crate for dynamic `.so`/`.dll` loading
  - C entry points: `_tupa_plugin_name` and `_tupa_plugin_register`
  - `PluginManager::load_plugin()`, `register_all()`, `list_functions()`
  - `StepFunction`: `Arc<dyn Fn(Value) -> Result<Value, String> + Send + Sync>`
- **Config DSL (Phase 4)**:
  - `ConfigDecl` and `ConfigField` nodes in parser (`tupa-parser/src/lib.rs`)
  - `config Name { type field, ... }` syntax as first-class AST
  - Declarative pre-conditions for pipelines
- **Crates updated**:
  - All 10 Tupa-Lang crates to `0.8.2`

### Engineering and CI Completed

- Features validated end-to-end via ViperTrade integration.
- `tupa-plugin` added to workspace.
- Unit tests for `ViperExtensions` (name, trailing_status, position_sizing).
- Documentation parity maintained across PT-BR and EN.

### Validation Snapshot (workspace)

- Release status: `v0.8.2` tag cut, crates published and standalone artifacts released.
- Validation status:
  - docs parity green
  - markdownlint green
  - CI green for merged language/runtime changes
  - CI local of ViperTrade green against the release line
  - ViperTrade runtime aligned with the official standalone `v0.8.2` CLI release

### Technical Debt

- crates.io publication blocked by `path =` dependencies in manifests.
- Config DSL documentation could use more practical examples.
- Hot reload opt-in via feature flag; default disabled for throughput.

## 0.8.1 (2026-03-21)

- Release theme: production strategy support for real policy systems.
- Planning reference:
  - `docs/en/releases/rfc_v0.8.1_trading_strategy_support.md`

### Delivered Scope

- Language and runtime support for production strategy systems.
- Declarative strategy modeling improvements:
  - structured step outputs
  - first-class policy reasons
  - weighted score support
  - typed config input pattern via nested records
  - declarative temporal policy support
- Type system and runtime slices delivered:
  - record types
  - record literals
  - typed field access
  - runtime schema validation for structured inputs and outputs
- Temporal policy builtins delivered:
  - `confirm(...)`
  - `cooldown(...)`

### Engineering and CI Completed

- RFC added in English, PT-BR, and Spanish to preserve docs parity.
- Docs parity maintained across the release planning and implementation cycle.
- Containerized local CI added to reduce drift between host environments and GitHub Actions.
- Trading support docs and examples expanded with:
  - config-driven pipeline example
  - temporal policy example
- ViperTrade integration used as a functional proving ground for the 0.8.1 slices.

### Validation Snapshot (workspace)

- Release status: `v0.8.1` tag cut, crates published, and standalone artifacts released.
- Validation status:
  - docs parity green
  - markdownlint green
  - CI green for merged language/runtime changes
  - ViperTrade local CI green against the release line
  - ViperTrade runtime aligned with the official standalone `v0.8.1` CLI release

### Technical Debt

- Typed config access is solved pragmatically through structured `input`, not dedicated config-binding syntax.
- Temporal policy remains declarative at the policy layer; host-managed state still lives outside the language runtime.
- Reusable policy ergonomics still depend mostly on normal functions and explicit record composition.

## 0.8.0-rc.5 (2026-03-07)

- Parser compatibility fixes for ViperTrade pipeline adoption:
  - tolerate top-level `type` declarations
  - tolerate top-level `extern fn ...;` declarations
  - accept unquoted step names (`step(name)`) in pipelines
- Crate publication docs improved:
  - added `README.md` to all publishable crates
  - added `readme = "README.md"` in all crate manifests

## 0.8.0 (2026-03-05)

- Release theme: controlled, auditable Python integration for production pipelines.
- Guiding principle: "Integrate without losing governance - every Python call is traced, validated, and auditable."

### Delivered Scope

- Python interoperability (`tupa-pyffi`) for safe invocation of `py:module.func` steps.
- Runtime resilience with circuit breaker and async/await support.
- Backtesting flow with PnL/risk evaluation and structured audit logging.
- Validation improvements for tensor shapes, pipeline attributes, and parser/typechecker robustness.

### Engineering and CI Completed

- CI now enforces PR title convention (`type(scope): subject`) and commit message convention.
- PR auto-labeling by change type (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`, `breaking`).
- Release drafter enabled with automatic categorization.
- Branch protection on `main` hardened:
  - required status checks (`pr-title-convention`, `commit-message-convention`, `lint`, `test`)
  - strict up-to-date branch requirement
  - required conversation resolution
  - required CODEOWNERS review and 1 approval
  - stale review dismissal enabled
- CODEOWNERS added for governance and workflow-critical files.
- Backport governance implemented:
  - `backport-X.Y` label validation workflow
  - automatic tracking issue creation on merged PRs with backport labels
- Release operations documented with `release_guide.md` and `release_cut_checklist.md`.
- Local validation standardized through `scripts/ci-local.sh` (code + docs/link lint).

### Validation Snapshot (workspace)

- Local full check executed on 2026-03-05: `./scripts/ci-local.sh`.
- Result: pass (`fmt`, `clippy`, `test`, `markdownlint`, `lychee`).
- Working tree status during validation: clean on `main`.

### Technical Debt

- Commit convention enforcement still depends on PR context; direct pushes to protected branches remain policy-dependent and should stay blocked by branch protection.
- Docs quality gates are strong in CI, and multilingual changelog parity for structure and latest version is automated; semantic parity of full translated content is still manual.
- Backport workflow creates tracking issues, but backport cherry-pick automation is not implemented yet.
- Performance goals are documented, but there is no CI trend dashboard storing historical latency and throughput metrics.

## 0.7.0 (2026-02-20)

- Release: hybrid engine with native pipeline governance
- CLI: `tupa run` with `--plan`, `--plan-only`, `--output`
- Runtime: JSON report with metrics and constraints (pass/fail), audit hash
- Determinism: `@deterministic(seed=...)` parsed and seed propagated to PRNG
- Codegen: `ExecutionPlan` JSON with `steps`, `constraints`, `metrics`, `metric_plans`
- Validation: input JSON validated against `TypeSchema` before run

### Added

- Hybrid backend:
  - ExecutionPlan JSON for pipelines
  - CLI `tupa codegen --format=llvm` emits `.ll` and `.plan.json`
  - Pipeline runtime (`tupa-runtime`) and `tupa run` command
- Pipeline validator:
  - `@deterministic` rejects `Random`/`Time` (E2005)
  - Constraints with undefined metrics (E2006)
- No breaking changes

### Performance

- Compile time (medium example): target < 200ms
- Status: not explicitly benchmarked in CI; tracked as a product target
- How to measure locally:
  - Build the CLI: `cargo build --quiet`
  - Benchmark commands (example):
    - `tupa codegen --format=llvm examples/pipeline/minimal.tp`
    - `tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json`
  - Optional: use `hyperfine` to benchmark:
    - `hyperfine --warmup 3 "tupa codegen --format=llvm examples/pipeline/minimal.tp" "tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json"`
  - Conditions: Linux, Rust stable (>=1.75), release builds preferred when applicable
- Hardware and conditions:
  - Linux x86_64, Rust stable, local dev machine, cold run
- Test reference (prints timing):
  - `cargo test -p tupa-cli perf -- --nocapture`
  - Observed locally: `codegen fraud_complete ~= 1ms`, `run fraud_complete ~= 3ms` (non-CI, illustrative)

## 0.6.0 (2026-02-13)

- Enum constructor inference with generics and Safe constraints in variants.
- Match patterns now support constructor destructuring with tuple patterns.
- Match guard binding usage validated in typechecker.
- Non-exhaustive match diagnostics now point to scrutinee spans.
- Added tests for enum constructor constraints and match destructuring/guards.
- Audit engine prototype with deterministic hash for AST and inputs.
- CLI audit command with JSON output for hashes.
- Audit CLI now uses SHA3-256 and `--input` flag.
- Added support for `@safety` annotations in parsing.
- Audit example `fraud_pipeline.tp` aligned with current `Safe` constraints.
- Typechecker warning `private_interfaces` resolved for `Ty::Enum`.

## 0.5.0 (2026-02-12)

- Typechecker constraints completion and validation fixes.
- Safe<string, ...> constraints: !hate_speech and !misinformation diagnostics.
- Diagnostics clarity improvements and consistency pass.
- Expanded test coverage with negative cases.
- Added misinformation examples and goldens for Safe<string, ...>.
- Docs updated with safe examples and diagnostics references.
- Docs aligned with README positioning and roadmap updates.
- Docs include a draft pipeline orchestration example.
- Release plan aligned with pipeline governance roadmap.
- Match diagnostics now point to invalid pattern spans; added negative test coverage.
- Safe annotations now validate base constraints; added invalid param/return examples.
- Negative lex/parse cases and JSON error outputs added to goldens.
- Golden update script now covers all negative examples.

## 0.4.0 (2026-02-11)

- Closure codegen improvements and environment capture fixes.
- Typechecker constraint improvements and better lambda inference.
- CLI flow updates for the typecheck/codegen pipeline.
- SPEC and common errors refreshed for the new behavior.
- Documentation cleanup: canonical English, consolidated indices, and PT-BR entrypoint.

## 0.3.0 (2026-02-07)

- Closure support with real variable capture (environment structures, heap allocation).
- Improvements in type inference for lambdas with Unknown parameters.
- Support for Func type compatibility with Unknown parameters in function calls.
- Code quality improvements: Clippy and rustfmt in CI, warning fixes.
- Basic support for traits (parsing, typechecking, codegen).
- Basic support for enums (parsing, typechecking, codegen).
- Unit tests added to codegen.
- Enum example added to documentation.
- Centralized index/SUMMARY and internal doc links.
- Sync of CHANGELOG, VERSIONING, and RELEASE_GUIDE.
- Variable capture detection in lambdas (closures in development).
- Fixes for residual TODOs in codegen for better robustness.
- Implementation of type inference for lambda parameters.
- Basic closure support in codegen (without environment capture yet).
- Golden test fixes for error cases (removed cargo messages).

## 0.2.0 (2026-02-06)

- Closure support with real variable capture (environment structures, heap allocation).
- Improvements in type inference for lambdas with Unknown parameters.
- Support for Func type compatibility with Unknown parameters in function calls.
- Code quality improvements: Clippy and rustfmt in CI, warning fixes.
- Basic support for traits (parsing, typechecking, codegen).
- Basic support for enums (parsing, typechecking, codegen).
- Unit tests added to codegen.
- Enum example added to documentation.
- Centralized index/SUMMARY and internal doc links.
- Sync of CHANGELOG, VERSIONING, and RELEASE_GUIDE.
- Variable capture detection in lambdas (closures in development).
- Fixes for residual TODOs in codegen for better robustness.
- Implementation of type inference for lambda parameters.
- Basic closure support in codegen (without environment capture yet).
- Golden test fixes for error cases (removed cargo messages).

## 0.9.6 (2026-06-06)

- Release theme: legacy `.tp` cleanup and coordinated version bump.

### Delivered Scope

- **Legacy `.tp` removal**: deleted all `.tp` example sources (~100 files) and their support assets (Python FFI helpers, JSON inputs, generator scripts) from `examples/`.
- **Golden cleanup**: removed obsolete golden outputs in `examples/expected/` produced by the discontinued `.tp` CLI; kept only the Rust-DSL golden (`expand_simple_pipeline.txt`).
- **Repository tidy-up**: removed stray legacy root artifacts (`update_golden.py`, `data.json`, `tx.json`, `my_test_plugin.rs`, `my_fixed_plugin.rs`, `integration_test.tupa`, `test_pipe.tupa`, `vipertrade_smoke.plan.json`, `test_find.md`).
- **Examples reorganized**: `examples/` now contains Rust-DSL material only; updated `examples/README.md` and `examples/migration/README.md`; removed obsolete `pipeline/`, `production/`, and `playground/` subdirectories.
- **Version bump**: all active crates bumped to 0.9.6 (no functional or API changes).
- **Feature docs rewritten to Rust DSL**: `features/trading_support.md` (EN/ES/PT-BR) now reflects the current crates with a runnable `pipeline!` + `Executor` example and explicitly marks the removed 0.8.2 runtime features (backtest, circuit breaker, hot reload, schema registry); `governance/audit_engine.md` (ES/PT-BR) replaced with a discontinuation note pointing to `tupa-engine` step metrics.
- **Crate READMEs corrected** for crates.io accuracy (API mismatches in `tupa-core`, `tupa-pyffi`, `tupa-plugin`, `tupa-engine`; `tupa-lints` reframed as string constants, not rustc lints).

### Engineering and CI Completed

- Fixed `examples-golden.yml` workflow to diff freshly generated goldens against the committed ones (it previously compared the directory against itself, masking drift).
- `cargo fmt`, `cargo clippy --workspace --all-targets`, `cargo test --workspace` all green.

### Validation Snapshot (workspace)

- Build: `cargo build --workspace` ok.
- Tests: `cargo test --workspace` green (167 tests).
- Smoke: `scripts/vipertrade-smoke.sh` ok.
- Goldens: `scripts/update-goldens.sh` produces no diff against `examples/expected/`.

### Technical Debt

- Several instructional docs still invoke the removed `tupa-cli` (`reference/codegen.md`, `guides/testing.md`, `guides/tutorials.md`, `guides/faq.md`, `governance/issues_guide.md`, `guides/examples_guide.md`); these should be migrated to `cargo-tupa` / Rust-DSL workflows in a follow-up. Historical references in `ARCHITECTURE.md`, `PROPOSAL.md`, `roadmap.md`, archives, and prior changelog entries are intentional and left as-is.

## 0.9.5 (2026-05-16)

- Release theme: test coverage, Safe/Tensor operations, cargo-tupa paths, and tupa-pyffi stability.

### Delivered Scope

- **Test coverage completion** (TC-51, TC-54, TC-55, TC-56):
  - Fixed `tc41_step_panic_display` — removed incorrect `source()` assertion
  - Fixed `tc46_step_timeout` / `tc52_from_env_timeout_caught_by_executor` — changed SlowP sleep from 10ms to 200ms and used `spawn_blocking`
  - Fixed `tc51_no_produces_for_single_step` — updated `SingleP::produces` to return empty array for unknown step
- **New unit tests**: Added 32 unit tests in `tupa-core-macros/tests.rs` and 30 unit tests in `tupa-core/src/tests.rs` (TC-C54..TC-C81)
- **Executor cancellation tests**: TC-55 and TC-56 for `Executor::cancel()` behavior
- **Criterion benchmarks**: `engine_bench.rs` with sequential, parallel, DAG, constraint, metrics, and executor_new benchmarks
- **Safe arithmetic operators**: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` for `Safe<T,C>`
- **Tensor methods**: `new()`, `get()`, `into_inner()`, `PartialEq` implementation
- **tupa-pyffi improvements**: `call_with_multiple_args()` for multi-argument Python calls, `reset_python_bridge()` for global state reset, extended type support (i32, u64, u32, f32, Vec<u8>, Vec<Value>)

### Engineering and CI Completed

- All 162 tests passing across workspace
- `cargo fmt`, `cargo clippy`, `cargo test --workspace` all green
- Version bump to 0.9.5 across all active crates

## 0.9.0 (2026-05-11)

### Delivered Scope

- **New crate-first architecture**: `tupa-core` (pipeline! macro + types), `tupa-engine` (parallel executor), `tupa-plugin` (dynamic loading), `cargo-tupa` (CLI)
- **Parallel execution**: Channel-based DAG scheduler with cycle detection (`Executor::run_parallel`)
- **Constraint system**: Compile-time + runtime metric checking with `metric("name").op(value)` DSL
- **Plugin FFI**: C ABI for step function registration (`libloading` + `extern "C"`)
- **Migration tooling**: Examples and guides for `.tp` → Rust DSL conversion
- **Documentation parity**: EN, ES, PT-BR with full cross-linking

### Engineering and CI Completed

- CI workflows: lint (clippy, rustfmt), test (workspace), docs-lint (markdownlint, parity, lychee), vipertrade-smoke gate
- Golden tests regenerated with `RUSTFLAGS="-Awarnings"` to suppress deprecation noise
- All broken relative links fixed across docs (grammar.ebnf, type_semantics, PROPOSAL, TRANSITION, etc.)
- External URLs updated (ViperTrade paths, GitHub Discussions → Issues)
- `tupa-cli` preserved for legacy `.tp` workflow; `cargo-tupa` for Rust DSL
- Version bump: `tupa-core` 0.9.0, `tupa-core-macros` 0.9.0, `tupa-engine` 0.9.0, `tupa-plugin` 0.9.0, `cargo-tupa` 0.9.0, `tupa-template` 0.9.0

### Validation Snapshot (workspace)

- **Release status**: Tag `v0.9.0` created; crates published to crates.io (core, engine, plugin, cargo-tupa)
- **Validation status**:
  - docs parity: green (all required files present in EN/ES/PT-BR)
  - markdownlint: green
  - link-check (lychee): 0 errors
  - CI: all jobs passing (lint, test, vipertrade-smoke)
  - ViperTrade smoke gate validates `tupa-cli` check + codegen for `vipertrade_smoke.tp`
- **Crates published**: `tupa-core@0.9.0`, `tupa-engine@0.9.0`, `tupa-plugin@0.9.0`, `cargo-tupa@0.9.0`
- **Legacy crates pinned**: `tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-runtime`, `tupa-effects`, `tupa-audit`, `tupa-fmt`, `tupa-lint` at 0.8.x

### Technical Debt

- `tupa-conformance` crate not yet published (SPEC validator — Phase 0 artifact, may be kept as dev-dependency only)
- `tupa-core-macros` missing CHANGELOG.md (should be added)
- `crates/tupa-template` uses path dependencies in template Cargo.toml — needs patch for generated projects
- PyFFI (`tupa-pyffi`) still at 0.8.2 — migration to 0.9.0 API pending (Phase 3)
- LSP (`tupa-lsp`) not implemented (deferred; rust-analyzer covers DSL)
- Benchmark suite (`criterion`) not yet created (Phase 4)
- Some public items in `tupa-core`/`tupa-engine` lack `///` docs (need API doc pass before 1.0)
