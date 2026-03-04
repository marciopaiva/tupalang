use serde_json::{json, Value};
use tupa_runtime::{register_step, run_pipeline_async, evaluate_constraints};
use tupa_codegen::execution_plan::{ExecutionPlan, StepPlan, ConstraintPlan, TypeSchema};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("ViperTrade POC - Runtime Example");

    // 1. Fetch Data (Sync Step)
    register_step("viper::fetch_data", |_input: Value| {
        println!("Step: Fetching market data (Sync)...");
        // Simulate network delay (blocking)
        thread::sleep(Duration::from_millis(100));
        
        Ok(json!({
            "symbol": "BTC/USD",
            "price": 65000.0,
            "volume": 100.5
        }))
    });

    // 2. Analyze Strategy (Sync Step)
    register_step("viper::analyze", |input: Value| {
        println!("Step: Analyzing market data...");
        // Access data from previous step
        let price = input.get("viper::fetch_data")
            .and_then(|data| data.get("price"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        
        // Simple strategy: if price < 70000, buy
        let signal = if price < 70000.0 { "BUY" } else { "SELL" };
        
        Ok(json!({
            "signal": signal,
            "confidence": 0.95
        }))
    });

    // 3. Execute Order (Sync Step)
    register_step("viper::execute", |input: Value| {
        println!("Step: Executing order...");
        // Access data from previous step
        let signal = input.get("viper::analyze")
            .and_then(|data| data.get("signal"))
            .and_then(|v| v.as_str())
            .unwrap_or("HOLD");
        
        if signal == "HOLD" {
            println!("No trade executed.");
            return Ok(json!({ "status": "skipped" }));
        }

        println!("Executing {} order!", signal);
        Ok(json!({
            "status": "filled",
            "order_id": "ord_12345",
            "side": signal
        }))
    });

    // Create Execution Plan
    let plan = ExecutionPlan {
        name: "viper_poc_pipeline".to_string(),
        version: "1.0.0".to_string(),
        seed: None,
        input_schema: TypeSchema {
            kind: "any".to_string(),
            elem: None,
            len: None,
            name: None,
            tensor_shape: None,
            tensor_dtype: None,
        },
        output_schema: None,
        steps: vec![
            StepPlan { name: "viper::fetch_data".to_string(), function_ref: "viper::fetch_data".to_string(), effects: vec!["io".to_string()] },
            StepPlan { name: "viper::analyze".to_string(), function_ref: "viper::analyze".to_string(), effects: vec![] },
            StepPlan { name: "viper::execute".to_string(), function_ref: "viper::execute".to_string(), effects: vec!["io".to_string()] },
        ],
        constraints: vec![
            ConstraintPlan { metric: "viper::analyze.confidence".to_string(), comparator: "gt".to_string(), threshold: 0.9 },
        ],
        metrics: std::collections::HashMap::new(),
        metric_plans: vec![],
    };

    println!("\n--- Pipeline Execution ---");
    
    // Initial State
    let initial_input = json!({});
    
    // Execute Pipeline
    let result = run_pipeline_async(&plan, initial_input).await;
    
    match result {
        Ok(output) => {
            println!("Pipeline Output: {}", output);
            let report = evaluate_constraints(&plan, &output);
            println!("Constraint Report: {}", report);
        }
        Err(e) => {
            println!("Pipeline Failed: {:?}", e);
            return Err(e.to_string());
        }
    }
    
    println!("\nPOC Complete.");
    Ok(())
}
