use serde_json::{json, Value};
use tupa_runtime::{register_step, run_pipeline_async, evaluate_constraints};
use tupa_codegen::execution_plan::{ExecutionPlan, StepPlan, ConstraintPlan, TypeSchema};
use tracing_subscriber::fmt::format::FmtSpan;

// This Rust step prepares data for the AI model
fn step_normalize_candles(input: Value) -> Result<Value, String> {
    let prices = input.get("input").unwrap().get("raw_prices").unwrap().as_array().unwrap();
    
    // Simple normalization: (p - p0) / p0
    if prices.is_empty() {
        return Ok(json!([]));
    }
    
    let base = prices[0].as_f64().unwrap();
    let normalized: Vec<f64> = prices.iter()
        .map(|p| (p.as_f64().unwrap() - base) / base)
        .collect();
        
    Ok(json!({ "candles": normalized }))
}

fn default_schema() -> TypeSchema {
    TypeSchema {
        kind: "any".to_string(),
        elem: None,
        len: None,
        name: None,
        tensor_shape: None,
        tensor_dtype: None,
    }
}

#[tokio::main]
async fn main() {
    // 1. Initialize Structured Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(false)
        .init();

    tracing::info!(system = "viper_bot", module = "inference", event = "startup");

    // 2. Register Native Rust Steps
    register_step("viper::normalize", step_normalize_candles);
    
    // Note: Python steps are auto-resolved via tupa-pyffi if named "module::function"
    // So "viper_model::predict_signal" will call python!

    // 3. Define Pipeline
    let plan = ExecutionPlan {
        name: "ViperAIStrategy".to_string(),
        version: "2.0.0-alpha".to_string(),
        seed: None,
        input_schema: TypeSchema { kind: "json".into(), ..default_schema() },
        output_schema: None,
        steps: vec![
            StepPlan {
                name: "normalized_data".to_string(),
                function_ref: "viper::normalize".to_string(),
                effects: vec![],
            },
            StepPlan {
                name: "ai_signal".to_string(),
                function_ref: "viper_model::predict_signal".to_string(),
                effects: vec!["signal".to_string()],
            }
        ],
        metric_plans: vec![],
        constraints: vec![
            ConstraintPlan {
                metric: "ai_signal.signal_strength".to_string(), // Uses dot notation!
                comparator: "gt".to_string(),
                threshold: 0.6, // Only trade if confidence > 60%
            }
        ],
        metrics: std::collections::HashMap::new(),
    };

    // 4. Simulate Market Data (Price History)
    let market_data = json!({
        "input": {
            "raw_prices": [100.0, 101.5, 102.0, 101.8, 103.5] // Rising trend
        }
    });

    // 5. Run Async Pipeline
    tracing::info!(event = "processing_tick", symbol = "BTCUSDT");
    
    match run_pipeline_async(&plan, market_data).await {
        Ok(result) => {
            // 6. Evaluate Risk Constraints
            let report = evaluate_constraints(&plan, &result);
            
            // Check specific constraint result
            let signal_strength = result["ai_signal"]["signal_strength"].as_f64().unwrap_or(0.0);
            
            if report["success"].as_bool().unwrap_or(false) {
                let action = result["ai_signal"]["action"].as_str().unwrap_or("UNKNOWN");
                tracing::info!(event = "signal_accepted", action = %action, confidence = signal_strength);
            } else {
                tracing::warn!(event = "signal_rejected", reason = "low_confidence", confidence = signal_strength);
            }
        },
        Err(e) => {
            tracing::error!(event = "inference_failed", error = %e);
        }
    }
}
