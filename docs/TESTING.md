# Testing Guide

## Purpose

Describe standard test commands and failure triage tips.

## Main commands

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

## Triage tips

- Run the isolated test before the full suite.
- Check whether the error is in parsing or typecheck.
- Compare spans and messages with expected output.
- Reproduce via `tupa-cli -- parse|check`.
