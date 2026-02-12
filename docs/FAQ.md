
# FAQ

## Purpose

Answer common questions about the project and the language.

## Frequently asked questions

### 1) Is the project production-ready?

Not yet. The v0.1 specification is complete, but the compiler is still being implemented.

### 2) What is the main focus of the language?

AI and critical systems with formal safety, alignment, and predictable performance.

### 3) How do I contribute?

See [CONTRIBUTING.md](../CONTRIBUTING.md) and open an issue with context.

### 4) Where can I find examples?

In [examples](../examples/README.md) and in [docs/SPEC.md](SPEC.md#exemplos).

### 5) What are `Safe<T, ...>` types?

Types with constraints proven at compile time, for example `Safe<f64, !nan>`. See details in [docs/SPEC.md](SPEC.md#alignment-types-ethical-constraints).

### 6) How do I run the CLI?

Use `cargo run -p tupa-cli -- <command>` and check [docs/GETTING_STARTED.md](GETTING_STARTED.md).

### 7) Is there a roadmap?

Yes: [docs/MVP_PLAN.md](MVP_PLAN.md) and [docs/ADOPTION_PLAN.md](ADOPTION_PLAN.md).

### 8) Can I propose spec changes?

Yes. Open an issue with the `[RFC]` prefix.

### 9) How does interoperability with other languages work?

The design includes FFI (Foreign Function Interface) for integration with Rust, C, and Python. See [docs/SPEC.md](SPEC.md#7-modules--ffi).

### 10) How is performance compared to other languages?

The goal is predictable performance, close to Rust/C for critical code. Benchmarks and examples will be published in future releases.

### 11) How do I debug or get detailed diagnostics?

See [docs/DIAGNOSTICS_CHECKLIST.md](DIAGNOSTICS_CHECKLIST.md) and [docs/COMMON_ERRORS.md](COMMON_ERRORS.md) for message examples and tips.

### 12) Are there usage tips or best practices?

See [docs/SPEC.md](SPEC.md#comparison) for examples and comparisons, and [docs/README.md](README.md) for quick links.

### 13) How do I contribute examples or documentation?

See [CONTRIBUTING.md](../CONTRIBUTING.md) and [docs/DOCS_CONTRIBUTING.md](DOCS_CONTRIBUTING.md).
