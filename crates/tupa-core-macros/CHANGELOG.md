# Changelog

All notable changes to `tupa-core-macros` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-05-11

### Added

- `pipeline!` procedural macro: defines typed policy pipelines in Rust
- AST parsing via `syn` for DSL syntax (`name`, `input`, `steps`, `constraints`)
- Code generation for `Pipeline`, `ExecutorPipeline`, and `ParallelPipeline` traits
- Step metadata methods: `produces_*`, `requires_*` for dependency scheduling
- Constraint expression parser: `metric("name").ge/le/eq/ne/gt/lt(value)`
- Compile-time constant folding for simple constraints (proof-of-concept)

### Notes

- This is the procedural macro crate that powers the `tupa-core` DSL.
- It is a proc-macro crate and cannot be used directly; import via `tupa_core::pipeline`.
- API subject to change before 1.0.

## [0.9.1] - 2026-05-11

### Documentation

- Updated cargo-tupa guide with subcommand details
- Added performance tuning guide (multi-language)

### Fixed

- Template dependencies pinned in generated projects

## [Unreleased]

- Planned: Improved error messages with span information and hints
- Planned: `tupa-expand` tool to show macro expansion output
