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
- This is the first alpha release of the new Rust-first TupaLang architecture.
- The legacy `.tp` DSL and related crates (`tupa-parser`, `tupa-typecheck`, etc.) are deprecated but still supported until 2027-01-01.

## [Unreleased]
- Planned: `tupa-migrate` CLI tool for automated `.tp` → Rust DSL conversion
