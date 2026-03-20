# Changelog

## Purpose

This document records relevant changes per version.

## 0.8.1 (planned)

- Planned release theme: production strategy support for real policy systems.
- Planned scope:
  - structured step outputs
  - first-class policy reasons
  - reusable predicates
  - weighted score support
  - declarative temporal policy support
- Planning reference:
  - `docs/en/releases/rfc_v0.8.1_trading_strategy_support.md`

### Delivered Scope

- Planned language and runtime support for production strategy systems.
- Planned improvements for declarative strategy modeling:
  - structured step outputs
  - first-class policy reasons
  - reusable predicates
  - weighted score support
  - declarative temporal policy support

### Engineering and CI Completed

- RFC added in English, PT-BR, and Spanish to preserve docs parity.
- Roadmap, changelog, and docs indices updated to point to the `0.8.1` planning track.

### Validation Snapshot (workspace)

- Current status: planning RFC only, no implementation merged yet.
- Validation expectation before release:
  - docs parity green
  - markdownlint green
  - CI green for language and runtime changes included in the final release scope

### Technical Debt

- The RFC defines planning direction, but no implementation slices are committed yet.
- Final release scope still depends on implementation cost and review outcomes.

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

## 0.1.0

- Specification v0.1 published.
- Basic lexer, parser, typechecker, and CLI.
