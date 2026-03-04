﻿# Tupã Language

Tupã is a safe, efficient, and deterministic pipeline orchestration language designed for high-reliability systems, including financial trading bots and AI inference workflows.

## Key Features

- **Deterministic Execution**: Pipelines are guaranteed to produce the same output for the same input (unless explicitly marked otherwise).
- **Type Safety**: Strong static analysis prevents runtime errors before execution.
- **Polyglot Runtime**: Seamlessly orchestrate Rust functions and Python AI models (via FFI).
- **Zero-Cost Abstractions**: Compiles to efficient Rust execution plans.

## Trading Bot Support (New in v0.8.0)

Tupã now includes specialized features for building robust crypto trading bots (like ViperTrade):

### 1. Circuit Breaker
Prevents cascading failures by automatically stopping execution when error thresholds are reached.
- **Configurable**: Set failure counts and reset timeouts.
- **Resilient**: Automatically enters "Half-Open" state to test recovery.
- **Example**: `cargo run -p tupa-runtime --example viper_circuit_breaker`

### 2. Backtesting Engine
Simulate trading strategies against historical data with built-in PnL tracking.
- **High Performance**: Runs strategies in the optimized Rust runtime.
- **Risk Management**: Validates constraints (e.g., max drawdown) on every step.
- **Example**: `cargo run -p tupa-runtime --example viper_backtest`

### 3. Structured Audit Logging
Compliance-ready JSON logging via `tracing`. Tracks every decision, trade, and risk check.

## Quick Start

### Building
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Running Examples
```bash
# Run the Backtest simulation
cargo run -p tupa-runtime --example viper_backtest

# Run the Circuit Breaker demo
cargo run -p tupa-runtime --example viper_circuit_breaker
```
