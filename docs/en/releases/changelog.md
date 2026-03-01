
# Changelog

## Purpose

This document records relevant changes per version.

## 0.8.0 (Unreleased)

- Release theme: controlled, auditable Python integration for production pipelines.
- Guiding principle: "Integrate without losing governance — every Python call is traced, validated, and auditable."
- Scope: PyTorch/TensorFlow orchestration via audited adapters.
- Focus: execution tracing, validation hooks, and audit log schema for Python calls.

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
    - `hyperfine --warmup 3 'tupa codegen --format=llvm examples/pipeline/minimal.tp' 'tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json'`
  - Conditions: Linux, Rust stable (>=1.75), release builds preferred when applicable
- Hardware & conditions:
  - Linux x86_64, Rust stable, local dev machine, cold run
- Test reference (prints timing):
  - `cargo test -p tupa-cli perf -- --nocapture`
  - Observed locally: `codegen fraud_complete ≈ 1ms`, `run fraud_complete ≈ 3ms` (non-CI, illustrative)

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
