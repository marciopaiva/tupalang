# tupa-lints

**Lint definitions for Tupã pipelines** — static analysis rules for pipeline quality.

## Overview

Provides lint constants that can be used with `#[deny]` / `#[warn]` attributes to enforce pipeline quality constraints.

**Status:** Alpha (0.9.x). Part of the crate-first Tupã architecture.

## Usage

```rust
#![deny(tupa_lints::PIPELINE_UNUSED_METRIC)]
#![warn(tupa_lints::PIPELINE_TOO_LARGE)]

use tupa_core::pipeline;
```

## Available Lints

| Lint | Description |
|---|---|
| `PIPELINE_UNUSED_METRIC` | Warn when a produced metric is not consumed by any constraint |
| `PIPELINE_TOO_LARGE` | Warn when pipeline exceeds 20 steps |
| `CONSTRAINT_LITERAL` | Warn when constraint uses a literal value instead of named constant |

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-lints)