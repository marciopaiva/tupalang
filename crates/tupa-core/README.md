# tupa-core

**Tupã core library** — types, traits, and the `pipeline!` macro.

## Purpose

This crate provides the main user-facing API for writing Tupã policies in Rust:

- `pipeline!` macro — defines a typed pipeline
- Types: `Safe<T, C>`, `Tensor<T, shape, density>`
- Trait: `Pipeline`
- Constraint builder: `metric!` (future)

**Status:** Alpha (0.9.x). API may change before 1.0.

## Quick Example

```rust
use tupa_core::{pipeline, types::Safe};

#[derive(Debug, Clone)]
struct Trade { size: f64, price: f64 }

fn risk(trade: &Trade) -> f64 { trade.size * trade.price / 1e6 }

pipeline! {
    name: MyPolicy,
    input: Trade,
    steps: [
        step("risk") { risk(input) }
    ],
    constraints: [
        metric("risk").le(10.0)
    ]
}
```text

## Crates

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
- Docs: [docs.rs/tupa-core](https://docs.rs/tupa-core)

## Modules

- `types`: `Safe`, `Tensor`
- `pipeline`: re-export of `pipeline!` macro (from `tupa-core-macros`)
