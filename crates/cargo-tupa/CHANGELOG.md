# Changelog

All notable changes to `cargo-tupa` will be documented in this file.

## [0.9.0] - 2026-05-11

### Added
- `tupa check` — typecheck Rust DSL pipelines without execution
- `tupa test` — run pipeline test cases (conformance validation)
- `tupa run` — execute a pipeline with given input JSON
- `tupa plugin new [FILENAME]` — generate a new plugin scaffold (uses `tupa-plugin` template)
- Support for `TUPA_PARALLEL` environment flag to enable parallel execution mode

### Changed
- Command structure: subcommands replace single-command options
- `plugin new` writes to stdout by default; redirect to file to create plugin crate

### Notes
- Beta release; CLI may change before 1.0
- The `plugin` commands require `tupa-plugin` 0.9.0+ installed
- Legacy `.tp` file support is provided via deprecated crates; new pipelines should use Rust DSL

## [Unreleased]
- Planned: `tupa migrate` — convert legacy `.tp` pipelines to Rust DSL
- Planned: `tupa format` — format pipeline definitions
- Planned: `tupa audit` — integrate with `tupa-audit` for traceability
