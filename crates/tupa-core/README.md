# tupa-core

**Tupã core library** — types, traits, and the `pipeline!` macro for defining policies in Rust.

## Overview

This crate provides the main user-facing API for writing Tupã policies in Rust:

- `pipeline!` macro — defines a typed pipeline
- Types: `Safe<T, C>`, `Tensor<T, shape, density>`
- Trait: `Pipeline`
- Constraint builder: constraint DSL in pipeline definitions

**Status:** Alpha (0.9.x). API may change before 1.0.

## Installation

```toml
[dependencies]
tupa-core = "0.9"
```

## Quick Example

```rust
use tupa_core::pipeline;

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
```

## Types

### `Safe<T, C>`

Constrained numeric types with compile-time and runtime safety checks:

```rust
use tupa_core::types::Safe;

// Basic Safe with non-negative constraint
let value: Safe<f64, _> = Safe::new(42.0).unwrap();

// Arithmetic operators supported
let a = Safe::new(10.0).unwrap();
let b = Safe::new(5.0).unwrap();
assert_eq!(*a + *b, 15.0);
```

### `Tensor<T, Shape, Density>`

Dense or sparse tensor types for multidimensional data.

## Module Structure

- `types` — `Safe`, `Tensor` type definitions
- `pipeline` — re-export of `pipeline!` macro (from `tupa-core-macros`)

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-core)