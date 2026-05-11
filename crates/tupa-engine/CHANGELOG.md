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

## [Unreleased]
- Planned: Plugin step execution integration
- Planned: Bounded channel for large DAGs (backpressure)
- Planned: Per-step timeout configuration
