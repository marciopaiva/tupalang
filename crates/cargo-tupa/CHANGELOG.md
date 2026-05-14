# Changelog

All notable changes to `cargo-tupa` will be documented in this file.

## [0.9.3] - 2026-05-14

### Added

- Initial Rust-DSL only release: `cargo tupa check`, `run`, `test`, `plugin new`
- Workspace integration: depends on `tupa-core`, `tupa-engine`, `tupa-plugin` (0.9.3)
- Minimal implementation for pipeline execution and plugin scaffolding

### Removed

- All legacy `.tp` support (`fmt`, `lint` subcommands for `.tp` files)
- Dependencies on `tupa-parser`, `tupa-lexer`, `tupa-lint`, `tupa-fmt`
