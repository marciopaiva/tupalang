# Changelog

All notable changes to `tupa-core` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-05-11

### Added

- `pipeline!` macro for defining type-safe pipelines in Rust
- Core types: `Plan`, `Step`, `Metric`, `Input`, `Output`
- Dependency graph (DAG) representation and validation
- Support for `produces`, `requires`, and `constraints` in pipeline definitions
- Re-export of `serde_json::Value` for dynamic metric values

### Notes

- This is the first alpha release of the new Rust-first Tupã architecture.
- The legacy `.tp` DSL and related crates (`tupa-parser`, `tupa-typecheck`, etc.) are deprecated but still supported until 2027-01-01.

## [0.9.1] - 2026-05-11

### Documentation

- Updated cargo-tupa guide with subcommand details (`fmt`, `lint`, `test`, `plugin new`)
- Added performance tuning guide (EN/ES/PT-BR)

### Fixed

- Template Cargo.toml dependencies pinned to `=0.9.0`

## [Unreleased]

- Planned: `tupa-migrate` CLI tool for automated `.tp` → Rust DSL conversion
