# Migration Examples: `.tp` → Rust DSL

This directory contains side-by-side examples showing how to migrate legacy Tupã (`.tp`) pipelines to the new Rust DSL (`pipeline!` macro).

Each example includes:

- `*_before.tp` — the original `.tp` source
- `*_after.rs` — equivalent Rust DSL program

## Running the Examples

To run a migrated example:

```bash
# Build and run the Rust DSL version
cargo run --example minimal_after

# Or execute via tupa-engine directly
cargo tupa run examples/migration/minimal_after.rs
```text

The `.tp` versions can be run with the legacy toolchain:

```bash
cargo tupa run examples/pipeline/minimal.tp
```text

## Included Examples

- **minimal** — two-step pipeline with simple functions (enrich, score)
- **credit_decision** — pipeline with constraints and validation context

See also: [`../docs/en/TRANSITION.md`](../docs/en/TRANSITION.md) for a complete migration guide.
