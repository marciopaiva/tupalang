# Examples

These examples use the **Rust DSL** (`pipeline!` macro from the `tupa-core` crate).
The legacy `.tp` file format was removed in 0.9.0 — see [TRANSITION.md](../docs/en/TRANSITION.md).

## Purpose

Curated, runnable Rust-DSL examples that mirror the patterns used in production
(e.g. [ViperTrade](https://github.com/marciopaiva/vipertrade)). For the canonical,
`cargo run`-able examples, see the `tupa-engine` crate.

## Files

### Standalone

- `simple_pipeline.rs` — end-to-end `pipeline!` definition with constraints, run via `tupa-engine::Executor`.
- `expected/expand_simple_pipeline.txt` — golden output for `cargo tupa expand --file examples/simple_pipeline.rs`.

### Migration (`.tp` → Rust DSL)

The [`migration/`](migration/README.md) directory shows former `.tp` pipelines re-expressed
as idiomatic Rust DSL programs.

### Canonical engine examples

The runnable, `cargo`-integrated examples live in the `tupa-engine` crate:

```bash
cargo run --package tupa-engine --example minimal
cargo run --package tupa-engine --example simple
cargo run --package tupa-engine --example credit_decision
cargo run --package tupa-engine --example fraud_complete
cargo run --package tupa-engine --example vipertrade_smoke
```

## Plugin System

Tupã supports dynamic loading of step functions via the **Plugin System**
(`tupa-plugin` crate). Plugins are shared libraries written in Rust that export C
entry points. See `crates/tupa-plugin/README.md` for a full example.

## Writing your own

Create a Rust crate that depends on `tupa-core` and `tupa-engine`, write a
`pipeline! { ... }` block, and run it with `tupa_engine::Executor`. See
[Getting Started](../docs/en/guides/getting_started.md).
