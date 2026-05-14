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

## [0.9.2] - 2026-05-13

### Added

- **Executor configuration**: `ExecutorConfig` and `Executor::with_config` for timeouts and channel capacity
- **Bounded channel** for step completion notifications (configurable capacity, default 1000) — applies backpressure
- **Per-step timeout**: `ExecutorConfig::with_step_timeout(Duration)` — steps exceeding the limit return `EngineError::StepTimeout`
- **`EngineError::StepTimeout`** variant for step timeout errors

### Changed

- Parallel scheduler now uses bounded `mpsc::channel` instead of unbounded

## [0.9.3] - 2026-05-14

### Changed

- Workspace cleanup: removed legacy `.tp` compiler crates and tooling from workspace
- Removed dev-dependencies on workspace crates to enable `--locked` publishing
- Bump all active crate versions to 0.9.3

