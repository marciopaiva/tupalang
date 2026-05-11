# tupa-fmt

Formatter for Tupã source code.

## Purpose

Canonicalizes layout, indentation, and line breaks for **legacy `.tp` files**. New Rust DSL projects use `cargo fmt` (rustfmt) — no separate formatter needed.

## Status

- ✅ Stable for `.tp` files
- 🚧 No plans to format Rust DSL (use rustfmt)

## Usage (Legacy)

```bash
cargo run -p tupa-fmt -- input.tp > formatted.tp
# or
tupa fmt input.tp  # if installed via tupa-cli (deprecated)
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-fmt` to maintain consistent style in legacy `.tp` strategy files during the transition period.
