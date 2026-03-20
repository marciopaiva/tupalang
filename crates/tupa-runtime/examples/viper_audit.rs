use serde_json::{json, Value};
use tracing_subscriber::fmt::format::FmtSpan;
use tupa_codegen::execution_plan::{ConstraintPlan, ExecutionPlan, StepPlan, TypeSchema};
use tupa_runtime::{evaluate_constraints, register_step, run_pipeline};

// Define Mock Steps
fn step_validate_market_regime(input: Value) -> Result<Value, String> {
    let vol = input
        .get("input")
        .unwrap()
        .get("volatility")
        .unwrap()
        .as_f64()
        .unwrap();
    // Logic: If volatility is too high, return false
    let safe = vol < 5.0;
    Ok(json!(safe))
}

fn step_calculate_position_size(input: Value) -> Result<Value, String> {
    let balance = input
        .get("input")
        .unwrap()
        .get("balance")
        .unwrap()
        .as_f64()
        .unwrap();
    let risk_pct = 0.01; // 1% risk
    let size = balance * risk_pct;
    Ok(json!(size))
}

fn main() {
    // 1. Configure Structured Audit Logging (JSON format)
    // In production, this would pipe to a file or log collector (e.g. Datadog agent)
    tracing_subscriber::fmt()
        .with_target(false) // Clean output
        .json() // JSON format for machine parsing
        .with_span_events(FmtSpan::CLOSE) // Log duration of spans
        .with_current_span(false)
        .init();

    tracing::info!(
        system = "viper_bot",
        version = "0.0.1",
        event = "startup",
        "Initializing Tupã Audit System"
    );

    // 2. Register Steps
    register_step("viper::validate_market_regime", step_validate_market_regime);
    register_step(
        "viper::calculate_position_size",
        step_calculate_position_size,
    );

    // 3. Create Execution Plan
    let plan = ExecutionPlan {
        name: "ViperSmartCopyStrategy".to_string(),
        version: "1.2.0".to_string(),
        seed: Some(12345),
        input_schema: TypeSchema {
            kind: "ident".to_string(),
            elem: None,
            fields: None,
            len: None,
            name: Some("TradeSignal".to_string()),
            tensor_shape: None,
            tensor_dtype: None,
        },
        output_schema: None,
        steps: vec![
            StepPlan {
                name: "is_safe_market".to_string(),
                function_ref: "viper::validate_market_regime".to_string(),
                effects: vec![],
            },
            StepPlan {
                name: "position_size".to_string(),
                function_ref: "viper::calculate_position_size".to_string(),
                effects: vec!["wallet".to_string()],
            },
        ],
        metric_plans: vec![],
        constraints: vec![ConstraintPlan {
            metric: "is_safe_market".to_string(),
            comparator: "eq".to_string(),
            threshold: 1.0, // true
        }],
        metrics: std::collections::HashMap::new(),
    };

    // 4. Run Pipeline with High Volatility (Should Fail Constraint)
    tracing::info!(event = "trade_signal_received", symbol = "BTCUSDT");

    let input = json!({
        "input": {
            "symbol": "BTCUSDT",
            "volatility": 8.5, // High volatility
            "balance": 10000.0
        }
    });

    match run_pipeline(&plan, input) {
        Ok(output) => {
            let report = evaluate_constraints(&plan, &output);
            if report["success"].as_bool().unwrap_or(false) {
                tracing::info!(event = "trade_executed", size = %output["position_size"]);
            } else {
                tracing::warn!(event = "trade_rejected", reason = "constraints_failed");
            }
        }
        Err(e) => {
            tracing::error!(event = "pipeline_error", error = %e);
        }
    }
}
