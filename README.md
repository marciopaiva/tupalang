# Tupã Language

[![Build Status](https://img.shields.io/github/actions/workflow/status/tupalang/tupa/ci.yml?branch=main)](https://github.com/tupalang/tupa/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.8.0-blue.svg)](docs/en/releases/changelog.md)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)](https://github.com/tupalang/tupa)

**Tupã** is a deterministic, type-safe pipeline orchestration language designed for mission-critical systems. It bridges the gap between static safety and dynamic runtime flexibility, making it the ideal choice for financial trading bots, AI inference workflows, and high-reliability data processing.

> **Tupã** (Guaraní mythology): The spirit of thunder and enlightenment.

---

## ✨ Key Features

- **🛡️ Deterministic by Design**: Pipelines are compiled to Rust with strict type checking, preventing runtime errors.
- **🚀 High Performance**: Zero-overhead abstraction. The compiler generates optimined Rust code comparable to hand-written implementations.
- **🧠 Python AI Integration**: Seamlessly call Python functions (e.g., PyTorch, TensorFlow, scikit-learn) from Rust pipelines with automatic data marshaling.
- **⚡ Trading-Ready**: Built-in support for backtesting, circuit breakers, and financial indicators.

## 📄 Language Example

Tupã's syntax is declarative and pipeline-oriented. A minimal valid example:

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

## 💹 Trading Ready

Tupã v0.8.0 introduces first-class support for financial systems, enabling robust trading bot development:

- **Circuit Breakers**: Built-in mechanism to halt execution when consecutive failures occur, preventing cascading losses.
- **Backtesting Engine**: Native support for historical simulation with PnL tracking and audit logging.
- **Audit Logging**: Structured JSON logs for every decision, compliant with financial auditing standards.
- **Risk Constraints**: Define strict limits (e.g., `max_drawdown`, `exposure`) directly in the pipeline definition.

See [Trading Support Documentation](docs/en/features/trading_support.md) for details.

## 🛠️ Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (1.75+).

### From Source

```bash
git clone https://github.com/tupalang/tupa.git
cd tupalang
cargo install --path crates/tupa-cli
```

### Verify Installation

```bash
tupa --version
```

## 💡 Quick Examples

### 1. Generate and Run a Pipeline

```bash
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

### 2. Run from ExecutionPlan

```bash
tupa codegen --plan-only examples/pipeline/fraud_complete.tp
tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json
```

### 3. Runtime Trading Demos

```bash
cargo run -p tupa-runtime --example viper_backtest
cargo run -p tupa-runtime --example viper_circuit_breaker
```

## 📦 Project Structure

The project is organized as a Rust workspace:

| Crate | Description |
| --- | --- |
| [`tupa-parser`](crates/tupa-parser) | Source code parser and AST generation |
| [`tupa-runtime`](crates/tupa-runtime) | Execution engine with trading support and Python FFI |
| [`tupa-cli`](crates/tupa-cli) | Command-line interface for compiling and running pipelines |
| [`tupa-codegen`](crates/tupa-codegen) | Rust code generation from Tupã pipelines |
| [`tupa-typecheck`](crates/tupa-typecheck) | Static analysis and type validation |

## 📚 Documentation

Complete documentation is available in the `docs/` directory:

- [Getting Started](docs/en/guides/getting_started.md)
- [Language Specification](docs/en/reference/spec.md)
- [Changelog](docs/en/releases/changelog.md)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/en/guides/contributing_faq.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.