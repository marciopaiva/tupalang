# Changelog

All notable changes to `cargo-tupa` will be documented in this file.

## [0.9.5] - 2026-05-16

### Changed

- Updated dependencies to `tupa-core`/`tupa-engine`/`tupa-plugin` 0.9.5

## [0.9.4] - 2026-05-14

### Added

- `cargo tupa run` now auto-discovers pipelines in Cargo.toml (builds binary and executes with JSON input)
- `cargo tupa fmt` — formats Rust-DSL pipeline code in-place (basic indentation for `pipeline!` blocks)
- `cargo tupa lint` — static analysis: duplicate steps, undefined requires/produces, missing name/input
- `--parallel` flag support (forwards to engine)
- `--metrics-output <file>` flag — write step metrics JSON after execution
- Unit tests for discovery, fmt, and lint modules
- Integration test for `cargo tupa run` with metrics export

### Changed

- `cargo-tupa` now depends on `tupa-core`/`tupa-engine`/`tupa-plugin` at 0.9.4
- Improved error messages and exit codes

## [0.9.3] - 2026-05-14

### Added

- Initial Rust-DSL only release: `cargo tupa check`, `run`, `test`, `plugin new`
- Workspace integration: depends on `tupa-core`, `tupa-engine`, `tupa-plugin` (0.9.3)
- Minimal implementation for pipeline execution and plugin scaffolding

### Removed

- All legacy `.tp` support (`fmt`, `lint` subcommands for `.tp` files)
- Dependencies on `tupa-parser`, `tupa-lexer`, `tupa-lint`, `tupa-fmt`
