# FAQ

## Purpose

This document answers common questions about the project and the language.

## Frequently Asked Questions

### 1) Is the project production ready?

Not yet. The v0.1 specification is complete, but the compiler is still being implemented.

### 2) What is the main focus of the language?

Governance and determinism for AI pipelines in critical systems, with formal safety and predictable performance.

### 3) How do I contribute?

See [CONTRIBUTING.md](../../CONTRIBUTING.md) and open an issue with context.

### 4) Where can I find examples?

In [examples](../../examples/README.md) and in [SPEC](../reference/spec.md#10-validated-examples).

### 4.1) Are there safe/alignment examples?

Yes. See the `safe_*` examples in [examples](../../examples/README.md) and the `Safe<string, ...>` section in [SPEC](../reference/spec.md#alignment-types-ethical-constraints).

### 5) What are `Safe<T, ...>` types?

Types with constraints proven at compile time, for example `Safe<f64, !nan>` or `Safe<string, !misinformation>`. See details in [SPEC](../reference/spec.md#alignment-types-ethical-constraints).

### 6) How do I run the CLI?

Use `cargo run -p tupa-cli -- <command>` and check [Getting Started](getting_started.md).

### 7) Is there a roadmap?

Yes: [Release Plan](../releases/release_plan.md) and [Changelog](../releases/changelog.md).

### 8) Can I propose SPEC changes?

Yes. Open an issue with the `[RFC]` prefix.

### 9) How does interoperability with other languages work?

The design includes FFI (Foreign Function Interface) for integration with Rust, C, and Python. See [SPEC](../reference/spec.md#7-modules--ffi).

### 10) How is performance compared to other languages?

The goal is predictable performance, close to Rust/C for critical code. Benchmarks and examples will be published in future releases.

### 11) How do I debug or get detailed diagnostics?

See [Diagnostics Checklist](../reference/diagnostics_checklist.md) and the [SPEC diagnostics section](../reference/spec.md#11-diagnostics) for message examples and tips.

### 12) Are there usage tips or best practices?

See [SPEC](../reference/spec.md#comparison) for examples and comparisons, and [Index](../index.md) for quick links.

### 13) How do I contribute examples or documentation?

See [CONTRIBUTING.md](../../CONTRIBUTING.md) and [Docs Contributing](docs_contributing.md).
