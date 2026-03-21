# Trading Bot Support in Tupã

This document details the features implemented in the Tupã Runtime (v0.8.1-dev) specifically to support algorithmic trading applications like `ViperTrade`.

See also the `0.8.1` strategy-modeling RFC:

- [RFC: v0.8.1 Trading Strategy Support](../releases/rfc_v0.8.1_trading_strategy_support.md)

## Overview

The Tupã language and runtime have been enhanced to support critical financial operations, ensuring safety, resilience, and auditability.

## Key Features

### 1. Backtesting Engine

The `run_backtest` function provides a built-in simulation environment for trading strategies.

- **Purpose**: Validate strategy logic against historical data before live deployment.
- **Mechanism**:
  - Iterates through a dataset of historical candles/ticks.
  - Executes the pipeline for each time step.
  - Evaluates risk constraints (e.g., `MaxDrawdown`, `PositionSize`).
  - Tracks Portfolio PnL (Profit and Loss) based on `BUY`/`SELL` signals and `close` prices.
- **Audit**: Every trade and blocked action is logged with a structured audit trail.

### 2. Circuit Breaker

A resilience pattern to prevent cascading failures during market volatility or API outages.

- **Configuration**:
  - `failure_threshold`: Number of consecutive errors allowed (e.g., 3).
  - `reset_timeout`: Time to wait before testing the connection again (e.g., 30s).
- **Behavior**:
  - **Closed**: Normal operation.
  - **Open**: Blocks execution immediately when threshold is reached.
  - **Half-Open**: Allows a single test request to check for recovery.

### 3. Python AI Integration (`tupa-pyffi`)

Seamless integration with Python-based ML models (PyTorch/TensorFlow) for signal generation.

- **Syntax**: Steps defined as `py:module.func` (e.g., `py:viper_model.predict`).
- **Safety**: Inputs and outputs are validated against strict schemas (e.g., Tensor shapes `[1, 60, 4]`).
- **Performance**: Zero-copy (where possible) data transfer via FFI.

### 4. Structured Audit Logging

Compliance-ready logging using the `tracing` crate.

- **Format**: JSON-structured logs.
- **Events**:
  - `pipeline_start` / `pipeline_complete`
  - `trade_executed` (with price, type, and index)
  - `trade_blocked_by_risk` (when constraints fail)
  - `circuit_breaker_tripped`

### 5. Typed host-provided config via structured input

Tupã already supports a practical config-binding pattern for production strategy systems:

- declare pipeline input as a nested record
- pass market data and config in the same typed input object
- use ordinary field access inside policy functions

This is already enough for many strategy cases such as:

- per-symbol thresholds
- mode/profile overlays
- trailing parameters
- confirmation thresholds

Example shape:

```text
input: {
  symbol: string,
  signal: { spread_pct: f64, trend_score: f64 },
  config: {
    entry: {
      max_spread_pct: f64,
      min_trend_score_long: f64
    }
  }
}
```

See:

- `examples/pipeline/config_driven_strategy.tp`
- `examples/pipeline/config_driven_strategy.json`

## Usage Example

```rust
// Configuring the runtime for a trading bot
let runtime = Runtime::new();
runtime.configure_circuit_breaker(3, Duration::from_secs(10));

// Running a backtest
let result = runtime.run_backtest(&plan, historical_data).await?;
println!("Final PnL: {}", result["final_pnl"]);
```
