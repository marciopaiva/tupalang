# Compatibility Guide

## Purpose

Supported Rust versions, platforms, and crate compatibility matrix for Tupã 0.9.x.

## Rust Version

- **MSRV (Minimum Supported Rust Version):** 1.83 for all active crates (0.9.x series)
- Future 1.0 release will lock MSRV at 1.83 or higher

## Platforms

- Linux x86_64, aarch64
- macOS x86_64, aarch64
- Windows (via MSVC or GNU) — native, not just WSL

## Crate Compatibility

All active Tupã crates follow SemVer independently. Use matching major versions:

| tupa-core | tupa-engine | tupa-plugin | tupa-pyffi | Rust MSRV |
|---|---|---|---|---|
| 0.9.x | 0.9.x | 0.9.x | 0.9.x | 1.83 |

**Note:** `cargo-tupa` CLI is a separate crate tightly coupled to the engine; ensure version matches.

## Removed Crates (pre-0.9.0)

The following crates were **removed** from the workspace in 0.9.0 and are no longer available:

- `tupa-parser` — standalone `.tp` parser (replaced by `pipeline!` macro)
- `tupa-lexer` — tokenizer for `.tp` (removed)
- `tupa-typecheck` — type checker for `.tp` (integrated into macro)
- `tupa-codegen` — code generator for `.tp` (no longer used)
- `tupa-runtime` (old) — merged into `tupa-engine`
- `tupa-cli` — standalone CLI replaced by `cargo-tupa`
- `tupa-fmt` — `.tp` formatter removed
- `tupa-lint` — `.tp` linter removed
- `tupa-audit` — audit functionality integrated into engine
- `tupa-conformance` — SPEC validator (no longer a separate crate)
- `tupa-effects` — effect system merged into core
- `tupa-sys` — C ABI bindings not yet published
- `tupa-lsp` — language server (never published)
- `tupa-ad` — automatic differentiation (planned for future)

These crates are **not maintained**. Do not depend on them for new projects.

## Migration from Legacy `.tp`

See [TRANSITION.md](../TRANSITION.md) for guidance on moving from legacy standalone `.tp` pipelines to the Rust-DSL (`pipeline!` macro).
