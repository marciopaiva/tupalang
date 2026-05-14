# cargo-tupa

Cargo subcommand for Tupã Rust-DSL pipelines.

## Usage

```bash
# Check pipeline type correctness
cargo tupa check

# Run pipeline with input
cargo tupa run --input data.json

# Run pipeline in parallel
cargo tupa run --parallel --input data.json

# Generate plugin scaffold
cargo tupa plugin new my_plugin.rs

# Run integration tests
cargo tupa test
```

## Compatibility

This tool works with Tupã 0.9.x Rust-DSL pipelines defined via the `pipeline!` macro in `tupa-core`.
