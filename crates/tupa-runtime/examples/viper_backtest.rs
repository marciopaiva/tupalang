//! # ViperTrade Backtesting Example
//!
//! This example demonstrates how to use the Tupã Runtime to backtest a trading strategy.
//! It simulates a Simple Moving Average (SMA) Crossover strategy against synthetic market data.
//!
//! ## Features Demonstrated
//! - **Backtest Engine**: Running a strategy over historical data with PnL tracking.
//! - **Risk Constraints**: Validating trade signals against risk rules.
//! - **Audit Logging**: Emitting structured JSON logs for every trade.
//!
//! ## Usage
//! ```bash
//! cargo run -p tupa-runtime --example viper_backtest
//! ```

use serde_json::{json, Value};
use tracing_subscriber::fmt::format::FmtSpan;
use tupa_codegen::execution_plan::{ConstraintPlan, ExecutionPlan, StepPlan, TypeSchema};
use tupa_runtime::{register_step, run_backtest};

fn default_schema() -> TypeSchema {
    TypeSchema {
        kind: "any".to_string(),
        elem: None,
        fields: None,
        len: None,
        name: None,
        tensor_shape: None,
        tensor_dtype: None,
    }
}

// Strategy: Simple Moving Average Crossover (simulated)
fn simple_strategy(input: Value) -> Result<Value, String> {
    let price = input.get("close").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let ma_short = input
        .get("ma_short")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let ma_long = input.get("ma_long").and_then(|v| v.as_f64()).unwrap_or(0.0);

    let action = if ma_short > ma_long && price > ma_short {
        "BUY"
    } else if ma_short < ma_long {
        "SELL"
    } else {
        "HOLD"
    };

    Ok(json!({
        "action": action,
        "signal_strength": (ma_short - ma_long).abs()
    }))
}

#[tokio::main]
async fn main() {
    // 1. Initialize Audit Logs
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(false)
        .init();

    tracing::info!(system = "viper_backtest", event = "startup");

    // 2. Register Strategy
    register_step("strategy::sma_cross", simple_strategy);

    // 3. Define Plan
    let plan = ExecutionPlan {
        name: "ViperSMA".to_string(),
        version: "2.0.0-backtest".to_string(),
        seed: None,
        input_schema: default_schema(),
        output_schema: None,
        steps: vec![StepPlan {
            name: "signal".to_string(),
            function_ref: "strategy::sma_cross".to_string(),
            effects: vec!["action".to_string()],
        }],
        // Risk Management: Don't trade if signal is weak
        constraints: vec![ConstraintPlan {
            metric: "signal.signal_strength".to_string(),
            comparator: "gt".to_string(),
            threshold: 0.5, // Minimum spread required
        }],
        metric_plans: vec![],
        metrics: std::collections::HashMap::new(),
    };

    // 4. Generate Synthetic Market Data
    let mut dataset = Vec::new();
    let mut price = 100.0;
    for i in 0..50 {
        price += (i as f64).sin() * 2.0; // Wavy price
        dataset.push(json!({
            "close": price,
            "ma_short": price + (i % 5) as f64 - 2.0, // Noisy MA
            "ma_long": price + (i % 10) as f64 - 5.0
        }));
    }

    // 5. Run Backtest
    match run_backtest(&plan, dataset).await {
        Ok(report) => {
            tracing::info!(event = "backtest_report", pnl = %report["final_pnl"]);
            println!("Final PnL: ${:.2}", report["final_pnl"].as_f64().unwrap());
        }
        Err(e) => tracing::error!(event = "backtest_failed", error = %e),
    }
}
