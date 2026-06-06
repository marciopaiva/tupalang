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

## [0.9.6] - 2026-06-06

### Changed

- Version bump to 0.9.6. No functional changes; this release focuses on removing the legacy `.tp` toolchain artifacts from the repository.

## [0.9.5] - 2026-05-16

### Added

- **Fixed test suite** (TC-51, TC-46, TC-52): Corrected `produces` implementation for single-step pipelines, increased sleep duration for timeout tests, and fixed panic display assertion
- **New unit tests**: Added 32 unit tests in `tupa-core-macros/tests.rs` and 30 unit tests in `tupa-core/src/tests.rs` (TC-C54..TC-C81)
- **Executor cancellation tests**: TC-55 and TC-56 for `Executor::cancel()` behavior
- **Criterion benchmarks**: `engine_bench.rs` with sequential, parallel, DAG, constraint, metrics, and executor_new benchmarks
- **Safe arithmetic operators**: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` for `Safe<T,C>`
- **Tensor methods**: `new()`, `get()`, `into_inner()`, `PartialEq` implementation
- **tupa-pyffi improvements**: `call_with_multiple_args()` for multi-argument Python calls, `reset_python_bridge()` for global state reset, extended type support (i32, u64, u32, f32, Vec<u8>, Vec<Value>)

### Fixed

- `SlowP` sleep in timeout tests changed from 10ms to 200ms for reliable detection
- `SingleP::produces` now returns empty array for unknown steps instead of panicking
- Panic error message assertion corrected (removed incorrect `source()` check)

## [0.9.4] - 2026-05-14

### Added

- **Step metrics collection**: `PipelineResult::metrics` now includes per-step `StepMetrics` with start/end timestamps, duration, and execution state
- **Cancellation support**: `Executor::cancel()` sets internal flag; `run_parallel` checks periodically and returns `EngineError::Cancelled` when detected
- **Environment-driven configuration**: `Executor::from_env()` reads `TUPA_STEP_TIMEOUT` (duration string: e.g., "30s", "1m", "500ms") and `TUPA_CHANNEL_CAPACITY` (usize)
- **`parse_duration` utility**: parses duration strings with units (ms, s, m) for convenient timeout configuration
- **`EngineError::Cancelled`** error variant for pipeline cancellation scenarios

### Changed

- `Executor` now always has `cancel_token: Arc<AtomicBool>` (removed optionality)
- Parallel worker tasks wrapped in `tokio::time::timeout` when `step_timeout` is configured
- `StepState::Cancelled` added to `StepState` enum
- Manager task aggregates `StepMetrics` in shared `Arc<Mutex<HashMap>>` during execution

---
