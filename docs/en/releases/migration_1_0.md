# Migration Guide: 0.x → 1.0

This guide covers the changes between Tupã 0.9.x and the upcoming 1.0.0 release.

## Overview

Tupã 1.0.0 freezes the DSL macro syntax and constraint API. No breaking changes to public APIs are permitted after this release.

## Crate Changes

### `tupa-core` (frozen)

| 0.9.x | 1.0.0 | Notes |
|-------|-------|-------|
| `Safe<T, C>::new(v)` | unchanged | Constructor stable |
| `Safe<T, C>::into_inner()` | unchanged | Getter stable |
| `Safe<T, C>::map(f)` | **NEW** | Transform value with marker preservation |
| `Safe<T, C> +,-,*,/,neg` | **NEW** | Arithmetic operators |
| `Tensor<T>::new(v)` | **NEW** | Constructor |
| `Tensor<T>::get()` | **NEW** | Getter |
| `Tensor<T>::into_inner()` | **NEW** | Unwrap |

### `tupa-engine` (frozen)

| 0.9.x | 1.0.0 | Notes |
|-------|-------|-------|
| `Executor::run()` | unchanged | Sequential execution |
| `Executor::run_parallel()` | unchanged | Parallel execution |
| `Executor::cancel()` | unchanged | Cancellation API |
| `Executor::from_env()` | unchanged | Environment config |
| `TUPA_STEP_TIMEOUT` | unchanged | Timeout env var |
| `TUPA_CHANNEL_CAPACITY` | unchanged | Channel config |
| `TUPA_METRICS_OUTPUT` | unchanged | Metrics output |

## Macro Syntax (frozen)

The `pipeline!` macro syntax is frozen. Example:

```rust
pipeline! {
    name: MyPipeline,
    input: Input,
    steps: [
        step("step_a") { expr } produces ["metric_a"],
        step("step_b") { expr } requires ["metric_a"] produces ["metric_b"],
    ],
    constraints: [
        metric("metric_b").ge(0.0).le(1.0),
    ]
}
```

## Constraint Operators (frozen)

| Operator | Meaning |
|----------|---------|
| `.ge(x)` | greater than or equal |
| `.gt(x)` | greater than |
| `.le(x)` | less than or equal |
| `.lt(x)` | less than |
| `.eq(x)` | equal |
| `.ne(x)` | not equal |

## Migration Steps

1. Update `Cargo.toml`:

    ```toml
    [dependencies]
    tupa-core = "1.0"
    tupa-engine = "1.0"
    ```

2. Verify constraint syntax matches frozen operators

3. Run test suite to confirm behavior is preserved

## Deprecation Policy

After 1.0.0, breaking changes require:

1. Major version bump (2.0.0)
2. 6-month deprecation period
3. Migration guide in release notes
