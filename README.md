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
- **Secondary**: public Rust crates for embeddability (`tupa-lexer`, `tupa-effects`, `tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-pyffi`, `tupa-runtime`).

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
cargo install --locked tupa-cli
```

## Quick Usage

If you installed via release binary:

```bash
tupa --help
```

If you installed via Cargo:

```bash
tupa --help
```

Pipeline example:

```bash
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

## Crates.io (Rust Embedding)

Starting with the `0.8.x` line, Tupã core crates are published to `crates.io` in dependency order during release.

Core crates for embedding:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

`Cargo.toml` example:

```toml
[dependencies]
tupa-parser = "0.8"
tupa-typecheck = "0.8"
tupa-runtime = "0.8"
```

Release automation:

- [.github/workflows/release.yml](.github/workflows/release.yml): multi-platform `tupa` binary artifacts
- [.github/workflows/publish-crates.yml](.github/workflows/publish-crates.yml): ordered crates publication (`dry_run` and optional CLI publish)

## Crate Docs

Each publishable crate now includes a crate-local `README.md` and manifest metadata (`readme = "README.md"`) for crates.io package pages.

Direct crate docs:

- [`tupa-audit`](crates/tupa-audit/README.md)
- [`tupa-cli`](crates/tupa-cli/README.md)
- [`tupa-codegen`](crates/tupa-codegen/README.md)
- [`tupa-effects`](crates/tupa-effects/README.md)
- [`tupa-fmt`](crates/tupa-fmt/README.md)
- [`tupa-lexer`](crates/tupa-lexer/README.md)
- [`tupa-lint`](crates/tupa-lint/README.md)
- [`tupa-parser`](crates/tupa-parser/README.md)
- [`tupa-pyffi`](crates/tupa-pyffi/README.md)
- [`tupa-runtime`](crates/tupa-runtime/README.md)
- [`tupa-typecheck`](crates/tupa-typecheck/README.md)

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
