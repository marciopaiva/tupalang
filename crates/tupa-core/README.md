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

A value of type `T` tagged with a zero-sized constraint marker `C`. The marker is
a compile-time proof carrier and is erased at runtime.

```rust
use tupa_core::Safe;

// `C` is any zero-sized marker type you define for the constraint.
struct NonNeg;

let value = Safe::<f64, NonNeg>::new(42.0);

// Read the inner value with `get()` (Copy) or `into_inner()`.
assert_eq!(value.get(), 42.0);

// Arithmetic operators are supported between two `Safe<T, C>` values.
let a = Safe::<f64, NonNeg>::new(10.0);
let b = Safe::<f64, NonNeg>::new(5.0);
assert_eq!((a + b).get(), 15.0);
```

### `Tensor<T>`

A thin newtype wrapper for tensor-like payloads, with `new`, `get`, and
`into_inner`:

```rust
use tupa_core::Tensor;

let t = Tensor::new(vec![1.0, 2.0, 3.0]);
assert_eq!(t.into_inner(), vec![1.0, 2.0, 3.0]);
```

## Public API

- `Safe<T, C>`, `Tensor<T>` — exported from the crate root.
- `pipeline!` — procedural macro re-exported from `tupa-core-macros`.

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-core)
