//! # ViperTrade Circuit Breaker Example
//!
//! This example demonstrates the Circuit Breaker mechanism in the Tupã Runtime.
//! It simulates a flaky external API (e.g., an exchange) and shows how the runtime
//! automatically stops execution to prevent cascading failures.
//!
//! ## Features Demonstrated
//! - **Circuit Breaker**: Automatic failure detection and blocking.
//! - **State Transitions**: Closed -> Open -> Half-Open -> Closed.
//! - **Recovery**: Testing connectivity after a timeout.
//!
//! ## Usage
//! ```bash
//! cargo run -p tupa-runtime --example viper_circuit_breaker
//! ```

use serde_json::{json, Value};
use std::time::Duration;
use tracing_subscriber::fmt::format::FmtSpan;
use tupa_codegen::execution_plan::{ExecutionPlan, StepPlan, TypeSchema};
use tupa_runtime::{configure_circuit_breaker, register_step, run_pipeline_async};

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

// Simulates a flaky external API (e.g., Binance)
fn flaky_exchange_api(input: Value) -> Result<Value, String> {
    let should_fail = input
        .get("simulate_fail")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if should_fail {
        Err("Connection timeout (Simulated)".to_string())
    } else {
        Ok(json!({ "price": 50000.0, "status": "ok" }))
    }
}

#[tokio::main]
async fn main() {
    // 1. Initialize Logging
    tracing_subscriber::fmt()
        .with_target(false)
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(false)
        .init();

    tracing::info!(system = "viper_circuit_breaker", event = "startup");

    // 2. Register Steps
    register_step("exchange::get_price", flaky_exchange_api);

    // 3. Configure Circuit Breaker: Trip after 2 failures, reset after 5 seconds
    configure_circuit_breaker(2, Duration::from_secs(5));

    let plan = ExecutionPlan {
        name: "SafePriceFetcher".to_string(),
        version: "1.0.0".to_string(),
        seed: None,
        input_schema: default_schema(),
        output_schema: None,
        steps: vec![StepPlan {
            name: "price_data".to_string(),
            function_ref: "exchange::get_price".to_string(),
            effects: vec!["price".to_string()],
        }],
        constraints: vec![],
        metric_plans: vec![],
        metrics: std::collections::HashMap::new(),
    };

    // 4. Scenario:
    // - Request 1: OK
    // - Request 2: FAIL
    // - Request 3: FAIL (Threshold Reached -> Breaker TRIPS)
    // - Request 4: BLOCKED immediately (Fast Fail)

    tracing::info!(event = "test_start", scenario = "failure_cascade");

    // Req 1: OK
    let _ = run_pipeline_async(&plan, json!({"simulate_fail": false})).await;

    // Req 2: Fail
    let _ = run_pipeline_async(&plan, json!({"simulate_fail": true})).await;

    // Req 3: Fail -> Trip!
    let _ = run_pipeline_async(&plan, json!({"simulate_fail": true})).await;

    // Req 4: Should be blocked by Breaker (not even calling step)
    match run_pipeline_async(&plan, json!({"simulate_fail": false})).await {
        Ok(_) => tracing::error!(event = "breaker_failed", msg = "Should have been blocked!"),
        Err(e) => {
            tracing::info!(event = "breaker_worked", error = %e);
            println!("Request 4 blocked as expected: {}", e);
        }
    }

    // 5. Wait for Reset (Simulated)
    tracing::info!(event = "waiting_for_reset", seconds = 6);
    tokio::time::sleep(Duration::from_secs(6)).await;

    // Req 5: Should work again (Half-Open -> Closed)
    match run_pipeline_async(&plan, json!({"simulate_fail": false})).await {
        Ok(_) => {
            tracing::info!(event = "breaker_reset_success");
            println!("Request 5 allowed after reset!");
        }
        Err(e) => tracing::error!(event = "breaker_reset_failed", error = %e),
    }
}
