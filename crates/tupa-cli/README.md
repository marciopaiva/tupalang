# tupa-cli

⚠️ **LEGACY.** This CLI is for `.tp` files only. New Rust DSL projects should use `cargo check` and `cargo test` directly.

## Purpose

Standalone command-line interface for legacy Tupã `.tp` files. Supports: `check`, `parse`, `lex`, `run`, `codegen`, `fmt`, `lint`, `audit`.

**New projects:** Use `tupa-core` macro DSL; no separate CLI needed. See [PROPOSAL.md](../../docs/en/PROPOSAL.md).

## Usage (Legacy)

```bash
# Typecheck a .tp file
tupa check path/to/strategy.tp

# Parse and print AST
tupa parse path/to/strategy.tp

# Run a pipeline
tupa run --pipeline=MyPipeline path/to/strategy.tp
```text

All commands support `--format json`.

## Installation (Legacy)

```bash
cargo install --locked tupa-cli
# or download binary from GitHub Releases
```text

## Status

- Maintained for backward compatibility with `.tp` pipelines
- No new features; bug fixes only
- Will be replaced by `cargo tupa` wrapper (coming in Phase 1)
- End-of-life: 2027-01-01

## Crate

- Binary name: `tupa`
- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
