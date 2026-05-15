# Testing Guide

## Purpose

Standard test commands and failure triage tips for Tupã 0.9.x (Rust-DSL era).

---

## Main Commands

```bash
# Full workspace test suite
cargo test --workspace --locked

# Per crate (active crates only)
cargo test -p tupa-core
cargo test -p tupa-core-macros
cargo test -p tupa-engine
cargo test -p tupa-plugin
cargo test -p tupa-pyffi
cargo test -p cargo-tupa
```

---

## cargo-tupa Tests

```bash
# Unit tests for CLI subcommands
cargo test -p cargo-tupa

# Run integration test (metrics output)
cargo test -p cargo-tupa --test run_metrics
```

---

## Performance Benchmarks

The `tupa-engine` benchmark suite (using `criterion`) is planned for Phase 4. Currently:

- Target: < 200ms per step for medium pipelines
- Measure with release builds: `cargo bench -p tupa-engine`

For rigorous measurements, use `hyperfine` with warmup.

---

## Ethical Constraints Tests

```bash
# Run constraint-related tests in engine
cargo test -p tupa-engine constraints
```

---

## Triage Tips

- Run isolated tests before full suite: `cargo test -p <crate> -- <filter>`
- Errors in macro expansion point to `pipeline!` block spans
- Check doc-tests for API examples
- Compare output with expected fixtures in `crates/*/tests/`
- Use `RUST_BACKTRACE=1` for full stack traces

---

## CI Local Validation

Before pushing, run:

```bash
./scripts/ci-local.sh
```

This runs fmt, clippy, tests, markdownlint, docs parity, link check, and (if applicable) golden output comparison.
