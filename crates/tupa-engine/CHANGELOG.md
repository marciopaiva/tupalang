# Changelog

All notable changes to `tupa-engine` will be documented in this file.

## [0.9.0] - 2026-05-11

### Added

- `Executor` struct for running pipelines (sequential via `run`, parallel via `run_parallel`)
- Tokio-based parallel execution engine with work-stealing task spawning
- DAG topological validation before execution
- Cycle detection with `EngineError::CycleDetected` variant
- Unsatisfiable dependency detection (missing producers)
- Constraint evaluation after pipeline completion (pass/fail result with metric values)
- Support for `#[tokio::main]` async runtime requirement on `run_parallel`

### Changed

- `run` method remains synchronous for backward compatibility
- `run_parallel` requires async context and returns `Result<ExecutionResult, EngineError>`

### Examples

- `minimal` — simple two-step pipeline
- `simple` — comprehensive test suite (metadata, pass, fail, cycle detection, unsatisfiable dependency)
- `fraud_complete` — full fraud detection with stateful metrics
- `credit_decision` — constraint-driven decision pipeline

### Notes

- This crate replaces the legacy `tupa-runtime` and `tupa-codegen`.
- It is the execution engine for both Rust DSL pipelines and future plugin-based steps.
- Breaking changes are expected before 1.0; current API is alpha.

## [0.9.1] - 2026-05-11

### Added

- `PipelineResult` now annotated with `#[must_use]` to prevent accidental discarding
- `Display` implementation for `ConstraintFailure` with formatted error messages
- README documentation for engine examples (`examples/README.md`)
- Plugin FFI overhead benchmark (`crates/tupa-plugin/benches/plugin_bench.rs`)
- Performance tuning guide (`docs/guides/performance_tuning.md`) for parallel execution, memory efficiency, and profiling

### Fixed

- Template Cargo.toml dependencies pinned to `=0.9.0` to avoid accidental incompatible versions

### Changed

- Removed `#[must_use]` from `Executor::run` and `Executor::run_parallel` methods (Result already has must_use)
- Updated cargo-tupa guide with subcommand documentation (`fmt`, `lint`, `test`, `plugin new`)

## [Unreleased]

- Planned: Plugin step execution integration
- Planned: Bounded channel for large DAGs (backpressure)
- Planned: Per-step timeout configuration
