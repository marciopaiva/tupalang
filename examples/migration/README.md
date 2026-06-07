# Migration Examples: `.tp` → Rust DSL

This directory contains Rust DSL (`pipeline!` macro) programs derived from former
legacy Tupã (`.tp`) pipelines. They illustrate how the discontinued `.tp` syntax
maps onto idiomatic Rust.

> The standalone `.tp` toolchain was removed in 0.9.0. Only the Rust DSL
> equivalents (`*_after.rs`) are kept; the original `.tp` sources no longer exist.

## Files

- `minimal_after.rs` — two-step pipeline with simple functions (enrich, score).
- `credit_decision_after.rs` — pipeline with constraints and validation context.

These are illustrative source files. To run a pipeline as a `cargo`-integrated
example, see the canonical examples in the `tupa-engine` crate:

```bash
cargo run --package tupa-engine --example minimal
cargo run --package tupa-engine --example credit_decision
```

See also: [`../../docs/en/TRANSITION.md`](../../docs/en/TRANSITION.md) for the complete migration guide.
