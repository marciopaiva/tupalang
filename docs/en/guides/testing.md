# Testing Guide

## Purpose

This document describes standard test commands and failure triage tips.

## Main Commands

```bash
# full suite
cargo test

# per crate
cargo test -p tupa-lexer
cargo test -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli
```

## CLI tests

```bash
# golden outputs
cargo test -p tupa-cli -- tests::cli_golden
```

## Performance Tests

- Goal: validate runtime for medium examples (target < 200ms).
- Run with logs:
  - `cargo test -p tupa-cli perf -- --nocapture`
- Validated signals:
  - Codegen for `fraud_complete` below 500ms (non-fragile limit).
  - `tupa run` for `FraudDetection` below 500ms.
- Notes:
  - Printed values are illustrative and vary by machine.
  - For rigorous measurements, use `hyperfine` with warmup (`--warmup`).
  - Prefer stable Rust and release builds for product measurements.

## Ethical Constraints

```bash
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation.tp
cargo run -p tupa-cli -- check examples/invalid_safe_misinformation_base.tp
```

## Triage Tips

- Run the isolated test before the full suite.
- Check whether the error is in parsing or typecheck.
- Compare spans and messages with expected output.
- Reproduce via `tupa-cli -- parse|check`.
