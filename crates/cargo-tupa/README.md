# cargo-tupa

Cargo subcommand for Tupã Rust-DSL pipelines.

## Overview

`cargo-tupa` provides Cargo subcommands for working with Tupã pipelines written in the Rust DSL.

**Status:** Alpha (0.9.x). Part of the crate-first Tupã architecture.

## Installation

```bash
cargo install cargo-tupa
```

## Commands

```bash
# Check pipeline type correctness
cargo tupa check

# Format pipeline! blocks
cargo tupa fmt

# Lint for common issues (duplicate steps, missing produces/requires)
cargo tupa lint

# Run pipeline with JSON input
cargo tupa run --input data.json

# Run pipeline in parallel
cargo tupa run --parallel --input data.json

# Auto-discover and run the pipeline binary in current project
cargo tupa discover

# Generate plugin scaffold
cargo tupa plugin new my_plugin.rs
```

## Environment Variables

- `TUPA_INPUT` — JSON input for pipeline execution
- `TUPA_STEP_TIMEOUT` — timeout duration (e.g., "30s", "1m")
- `TUPA_CHANNEL_CAPACITY` — async channel capacity

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/cargo-tupa)
