# Compatibility Guide

## Purpose

Supported Rust versions, platforms, and crate compatibility matrix.

## Rust Version

- **MSRV (Minimum Supported Rust Version):** 1.83 for `tupa-core` 0.2+
- Future 1.0 release will lock MSRV at 1.83 or higher

## Platforms

- Linux x86_64, aarch64
- macOS x86_64, aarch64
- Windows (via MSVC or GNU) — native, not just WSL

## Crate Compatibility

All Tupã crates follow SemVer independently. Use matching major versions:

| tupa-core | tupa-engine | tupa-audit | Rust MSRV |
|---|---|---|---|
| 0.2.x | 0.2.x | 0.8.x | 1.83 |
| 0.1.x | 0.1.x | 0.8.x | 1.75 (legacy) |

## Deprecated Crates

The following crates are maintained for backward compatibility only and will be removed after 2027:

- `tupa-parser` (use `tupa-core` macro instead)
- `tupa-typecheck` (integrated into macro expansion)
- `tupa-codegen` (no longer used)
- `tupa-cli` (replaced by `cargo tupa` wrapper)
- `tupa-runtime` (merged into `tupa-engine`)
- `tupa-effects` (merged into `tupa-core`)

They are not recommended for new projects.
