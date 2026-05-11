# tupa-typecheck

⚠️ **DEPRECATED.** This crate is maintained for backward compatibility only and will be removed after **2027-01-01**.

## Purpose

Static checks for legacy `.tp` programs (types, determinism, constraints). For new Rust DSL projects, type checking is performed by the Rust compiler via `tupa-core` macro expansion — no separate typechecker needed.

## Migration

New pipelines written with `pipeline!` macro are type-checked at compile time by rustc. See [TRANSITION.md](../../docs/en/TRANSITION.md).

## Status

- Last release: `0.8.x` (security fixes only)
- No new features

## Crate

- Works with `tupa-parser` AST (legacy)
- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
