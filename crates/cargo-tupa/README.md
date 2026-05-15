# cargo-tupa

Cargo subcommand for Tupã Rust-DSL pipelines (0.9.x).

## Usage

```bash
# Check pipeline type correctness
cargo tupa check

# Format pipeline! blocks in source files
cargo tupa fmt

# Lint for common issues (duplicate steps, missing produces/requires)
cargo tupa lint

# Run pipeline with input
cargo tupa run --input data.json

# Run pipeline in parallel
cargo tupa run --parallel --input data.json

# Auto-discover the binary target in the current project
cargo tupa discover

# Generate plugin scaffold
cargo tupa plugin new my_plugin.rs
```

## Compatibility

Works with Tupã 0.9.x Rust-DSL pipelines defined via the `pipeline!` macro in `tupa-core`.

## Installation

```bash
cargo install cargo-tupa
```
