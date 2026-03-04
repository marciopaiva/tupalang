﻿# Tupã Language

[![Build Status](https://img.shields.io/github/actions/workflow/status/tupalang/tupa/ci.yml?branch=main)](https://github.com/tupalang/tupa/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.8.0-blue.svg)](docs/en/releases/changelog.md)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)

**Tupã** is a deterministic, type-safe pipeline orchestration language designed for mission-critical systems. It bridges the gap between static safety and dynamic runtime flexibility, making it the ideal choice for financial trading bots, AI inference workflows, and high-reliability data processing.

## 🚀 Key Features

- **Deterministic Execution**: Guarantee reproducible outputs for the same inputs, eliminating "it works on my machine" issues.
- **Type Safety**: Strong static analysis prevents runtime errors before execution, catching bugs early.
- **Polyglot Runtime**: Seamlessly orchestrate Rust functions and Python AI models (via FFI) in a single pipeline.
- **Zero-Cost Abstractions**: Compiles to efficient Rust execution plans with minimal overhead.
- **Trading Ready**: Built-in support for **Circuit Breakers**, **Backtesting**, and **Audit Logging** (see [Trading Support](docs/en/features/trading_support.md)).

## 📚 Documentation

Complete documentation is available in the `docs/` directory:

- **Getting Started**: [Installation & First Steps](docs/en/guides/getting_started.md)
- **Architecture**: [System Overview](docs/en/overview/architecture.md)
- **Language Reference**: [Syntax & Semantics](docs/en/reference/spec.md)
- **Trading Features**: [Circuit Breaker & Backtesting](docs/en/features/trading_support.md)
- **Release Notes**: [Changelog](docs/en/releases/changelog.md)

## 🛠️ Installation

Ensure you have Rust installed. Then, clone the repository and build:

```bash
git clone https://github.com/tupalang/tupa.git
cd tupalang
cargo build --release
```

## 💡 Quick Example

### Trading Strategy Backtest

Run a backtest simulation using the built-in runtime example:

```bash
cargo run -p tupa-runtime --example viper_backtest
```

### Circuit Breaker Demo

Simulate a failing external API and observe the circuit breaker in action:

```bash
cargo run -p tupa-runtime --example viper_circuit_breaker
```

## 📦 Project Structure

The project is organized as a Rust workspace:

- [`tupa-parser`](crates/tupa-parser): Source code parser and AST generation.
- [`tupa-runtime`](crates/tupa-runtime): Execution engine with trading support and Python FFI.
- [`tupa-cli`](crates/tupa-cli): Command-line interface for compiling and running pipelines.
- [`tupa-codegen`](crates/tupa-codegen): Rust code generation from Tupã pipelines.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/en/guides/contributing_faq.md) for details.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
