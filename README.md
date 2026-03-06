# Tupã Language

[![Build Status](https://img.shields.io/github/actions/workflow/status/marciopaiva/tupalang/ci.yml?branch=main)](https://github.com/marciopaiva/tupalang/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.8.0-blue.svg)](docs/en/releases/changelog.md)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)](https://github.com/marciopaiva/tupalang)

**Tupã** is a deterministic, type-safe pipeline orchestration language designed for mission-critical systems.
It bridges static safety and dynamic runtime flexibility for trading workflows, AI inference, and high-reliability data processing.

> **Tupã** (Guaraní mythology): the spirit of thunder and enlightenment.

## Distribution Model (v0.8.0-rc)

Tupã follows a **hybrid distribution** strategy:

- **Primary**: standalone CLI binary artifacts for end users and operations.
- **Secondary**: public Rust crates for embeddability (`tupa-parser`, `tupa-typecheck`, `tupa-runtime`).

Details:

- [Hybrid Distribution Decision](docs/en/governance/hybrid_distribution_decision.md)
- [Installation Guide](docs/en/guides/installation.md)
- [Embedding in Rust](docs/en/reference/embedding.md)

## Key Features

- Deterministic by design with static type checking.
- Native performance with Rust-based compilation/runtime.
- Python integration path for AI workflows.
- Trading-ready primitives (constraints, backtesting, circuit breakers, audit logs).

## Language Example

```tupa
enum MarketSignal {}

fn score(input: MarketSignal): i64 {
  return 42;
}

pipeline Strategy @deterministic(seed=42) {
  input: MarketSignal,
  steps: [
    step("score") { score(input) },
  ],
}
```

## Installation

### Recommended (release binary)

See the full matrix in [Installation Guide](docs/en/guides/installation.md).

Linux example:

```bash
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-linux-x86_64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```

### Rust developer path

```bash
cargo install tupa-cli
```

## Quick Usage

If you installed via release binary:

```bash
tupa --help
```

If you installed via Cargo:

```bash
tupa-cli --help
```

Pipeline example:

```bash
tupa-cli codegen --format=json examples/pipeline/fraud_complete.tp
tupa-cli run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

## Project Structure

| Crate | Description |
| --- | --- |
| [`tupa-parser`](crates/tupa-parser) | Source parser and AST generation |
| [`tupa-typecheck`](crates/tupa-typecheck) | Static analysis and type validation |
| [`tupa-codegen`](crates/tupa-codegen) | Code generation from Tupã pipelines |
| [`tupa-runtime`](crates/tupa-runtime) | Runtime execution engine |
| [`tupa-cli`](crates/tupa-cli) | Command-line interface |

## Documentation

- [Getting Started](docs/en/guides/getting_started.md)
- [Installation Guide](docs/en/guides/installation.md)
- [Embedding in Rust](docs/en/reference/embedding.md)
- [Language Specification](docs/en/reference/spec.md)
- [Changelog](docs/en/releases/changelog.md)

## Local CI

```bash
./scripts/ci-local.sh
```

Strict links mode:

```bash
CI_LOCAL_STRICT_LINKS=1 ./scripts/ci-local.sh
```

## Contributing

See [Contributing FAQ](docs/en/guides/contributing_faq.md).

## License

MIT. See [LICENSE](LICENSE).
