# tupa-lints

**Lint identifiers for Tupã pipelines** — named constants for pipeline-quality rules.

## Overview

This crate exposes a small set of `&'static str` constants that name recommended
pipeline-quality lints. They are stable string identifiers meant to be shared
across Tupã tooling and reports.

> **Note:** These are plain string constants, **not** `rustc`/Clippy lints. They
> cannot be used with `#[deny(...)]` / `#[warn(...)]` attributes. Enforcement of
> these rules against pipeline code is performed by external tooling, not by the
> compiler.

**Status:** Alpha (0.10.x). Part of the crate-first Tupã architecture.

## Usage

```rust
// Reference the identifier when emitting or filtering diagnostics.
assert_eq!(tupa_lints::PIPELINE_UNUSED_METRIC, "tupa_pipeline_unused_metric");
```

## Available Lints

| Constant | Identifier | Meaning |
|---|---|---|
| `PIPELINE_UNUSED_METRIC` | `tupa_pipeline_unused_metric` | A produced metric is not consumed by any constraint |
| `PIPELINE_TOO_LARGE` | `tupa_pipeline_too_large` | Pipeline exceeds 20 steps (maintainability) |
| `CONSTRAINT_LITERAL` | `tupa_constraint_literal` | Constraint uses a literal value instead of a named constant |

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-lints)
