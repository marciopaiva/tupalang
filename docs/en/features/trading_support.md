# Trading Support in Tupã

This document describes how Tupã's current crate-based architecture (0.9.x)
supports algorithmic trading policy use cases such as
[ViperTrade](https://github.com/marciopaiva/vipertrade).

> **History:** Earlier 0.8.x releases shipped a standalone `.tp` runtime with a
> built-in backtesting engine, circuit breaker, hot reload, and schema registry
> (the `tupa-runtime` / `tupa-codegen` crates). That standalone toolchain was
> removed in 0.9.0. The patterns below reflect what the current Rust-DSL crates
> (`tupa-core`, `tupa-engine`, `tupa-pyffi`, `tupa-plugin`) actually provide;
> removed features are called out explicitly under
> [Discontinued 0.8.2 runtime features](#discontinued-082-runtime-features).

## Modeling a strategy as a typed pipeline

Express a trading policy as a `pipeline!` over a typed input. Market data and
configuration travel together in one nested input struct, and policy steps use
ordinary Rust field access.

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Entry { max_spread_pct: f64, min_trend_score_long: f64 }
#[derive(Debug, Clone, Serialize)]
struct Config { entry: Entry }
#[derive(Debug, Clone, Serialize)]
struct Signal { spread_pct: f64, trend_score: f64 }
#[derive(Debug, Clone, Serialize)]
struct StrategyInput { symbol: String, signal: Signal, config: Config }

fn long_ok(i: &StrategyInput) -> f64 {
    let within_spread = i.signal.spread_pct <= i.config.entry.max_spread_pct;
    let trend_ok = i.signal.trend_score >= i.config.entry.min_trend_score_long;
    if within_spread && trend_ok { 1.0 } else { 0.0 }
}

pipeline! {
    name: EntryPolicy,
    input: StrategyInput,
    steps: [
        step("long_ok") { long_ok(input) }
    ],
    constraints: [
        metric("long_ok").ge(0.0)
    ]
}

fn main() {
    let plan = EntryPolicy::new();
    let engine = Executor::new();
    let input = StrategyInput {
        symbol: "BTCUSDT".into(),
        signal: Signal { spread_pct: 0.04, trend_score: 0.8 },
        config: Config { entry: Entry { max_spread_pct: 0.10, min_trend_score_long: 0.5 } },
    };
    let result = engine.run(&plan, &input).expect("run failed");
    println!("passed={} long_ok={}", result.passed, result.values["long_ok"]);
}
```

This typed-config pattern covers many strategy cases — per-symbol thresholds,
mode/profile overlays, trailing parameters, confirmation thresholds — all as
fields on the input.

## Risk constraints

Risk limits are expressed as `constraints` on metrics produced by steps. The
executor evaluates them and reports the outcome in `PipelineResult::passed`. For
example, cap a computed position-size or drawdown metric:

```rust
constraints: [
    metric("position_size").le(1_000_000.0),
    metric("max_drawdown_bps").le(1500.0)
]
```

## Temporal policy via host-provided state

Tupã keeps host state (counters, timers) outside the language: the host
maintains them and passes the current temporal state as part of the typed input.
Steps then express confirmation / cooldown decisions with ordinary Rust logic.

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Confirmation { observed: bool, consecutive_hits: i64, required_hits: i64 }
#[derive(Debug, Clone, Serialize)]
struct Cooldown { active: bool, remaining_seconds: i64 }
#[derive(Debug, Clone, Serialize)]
struct Temporal { signal_confirmation: Confirmation, cooldown_guard: Cooldown }
```

These cover signal confirmation after *N* consecutive observations, stop-loss
cooldown gates, and thesis-persistence thresholds driven by host-maintained
counters. (The 0.8.x `confirm(...)` / `cooldown(...)` built-ins were part of the
removed `.tp` runtime; express the same logic directly in Rust step functions.)

## Python / AI model integration (`tupa-pyffi`)

Call Python models (PyTorch / TensorFlow / NumPy) from a step via `tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

fn predict(input: &StrategyInput) -> Result<serde_json::Value, String> {
    call_python_function("viper_model", "predict", json!({ "symbol": input.symbol }))
}
```

> The old `py:module.func` step syntax was a `.tp` feature; in the Rust DSL you
> call `tupa-pyffi` functions directly from a step body.

## Dynamic extension via plugins (`tupa-plugin`)

Step functions can be loaded from shared libraries at runtime using `tupa-plugin`
(`PluginManager`), enabling extension without recompiling the host. See the
`tupa-plugin` crate README for the plugin ABI and a complete example.

## Discontinued 0.8.2 runtime features

The following were part of the standalone `.tp` runtime removed in 0.9.0 and are
**not** available in the current crates:

- Built-in backtesting engine (`run_backtest`)
- Circuit breaker (`configure_circuit_breaker`)
- Hot reload (`Runtime::watch_and_reload`)
- Schema registry / migrations (`tupa-codegen`)
- `tupa::*` built-ins and the `py:` step syntax

For execution observability today, use the executor's per-step metrics
(`PipelineResult::metrics`) plus your application's own logging.
