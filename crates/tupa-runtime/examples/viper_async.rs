use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tupa_runtime::{register_async_step, run_pipeline_async, evaluate_constraints};
use tupa_codegen::execution_plan::{ExecutionPlan, StepPlan, ConstraintPlan, TypeSchema};
use tokio::time::{sleep, Duration};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[allow(dead_code)]
struct SmartCopyConfig {
    profile: String,
    risk_level: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct Db {
    win_rates: std::collections::HashMap<String, f64>,
}

fn step_fetch_market_data(input: Value) -> futures::future::BoxFuture<'static, Result<Value, String>> {
    Box::pin(async move {
        println!("Step: Fetch Market Data (Async I/O)");
        sleep(Duration::from_millis(50)).await;
        let mut out = input.as_object().unwrap().clone();
        out.insert("market_data_fetched".to_string(), json!(true));
        Ok(Value::Object(out))
    })
}

fn step_validate_entry(input: Value) -> futures::future::BoxFuture<'static, Result<Value, String>> {
    Box::pin(async move {
        println!("Step: Validate Entry (Async I/O)");
        sleep(Duration::from_millis(30)).await;
        let mut out = input.as_object().unwrap().clone();
        out.insert("entry_validated".to_string(), json!(true));
        Ok(Value::Object(out))
    })
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create execution plan
    let plan = ExecutionPlan {
        name: "viper_async_pipeline".to_string(),
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
            StepPlan { name: "viper::fetch_market_data".to_string(), function_ref: "viper::fetch_market_data".to_string(), effects: vec!["io".to_string()] },
            StepPlan { name: "viper::check_smart_copy_constraints".to_string(), function_ref: "viper::check_smart_copy_constraints".to_string(), effects: vec!["io".to_string()] },
            StepPlan { name: "viper::validate_entry".to_string(), function_ref: "viper::validate_entry".to_string(), effects: vec![] },
        ],
        constraints: vec![
            ConstraintPlan { metric: "viper::validate_entry.entry_validated".to_string(), comparator: "eq".to_string(), threshold: 1.0 },
            ConstraintPlan { metric: "viper::fetch_market_data.market_data_fetched".to_string(), comparator: "eq".to_string(), threshold: 1.0 }, // true -> 1.0
        ],
        metrics: std::collections::HashMap::new(),
        metric_plans: vec![],
    };

    // 2. Setup Shared State (DB)
    let db = Arc::new(Mutex::new(Db::default()));
    db.lock().unwrap().win_rates.insert("BTC/USD".to_string(), 0.65);

    // 3. Register Async Steps
    register_async_step("viper::fetch_market_data", step_fetch_market_data);
    register_async_step("viper::validate_entry", step_validate_entry);

    // Register Closure Async Step (Stateful)
    let db_clone = db.clone();
    register_async_step("viper::check_smart_copy_constraints", move |input: Value| {
        let db = db_clone.clone();
        Box::pin(async move {
            println!("Step: Check Smart Copy Constraints (Async DB)");
            sleep(Duration::from_millis(20)).await;
            
            // Mock input structure if needed, or just assume input has symbol
            // For this example, let's assume input has "symbol"
            let symbol = input.get("input").and_then(|i| i.get("symbol")).and_then(|s| s.as_str()).unwrap_or("BTC/USD");
            
            let rate = {
                let locked = db.lock().unwrap();
                locked.win_rates.get(symbol).cloned().unwrap_or(0.0)
            };
            
            let valid = rate > 0.5;
            
            // Return result matching schema or whatever logic
            // Assuming this step returns boolean or modifies state
            // Let's say it returns a boolean result
             Ok(json!(valid))
        })
    });

    // 4. Run Pipeline
    let input = json!({
        "symbol": "BTC/USD",
        "amount": 100.0
    });

    println!("Running Async Pipeline...");
    let result = run_pipeline_async(&plan, input).await;
    
    match result {
        Ok(output) => {
             // 4. Validate constraints
             let result = evaluate_constraints(&plan, &output);
             if result["success"].as_bool().unwrap_or(false) {
                 println!("Pipeline Success: {:?}", output);
             } else {
                 println!("Pipeline Constraint Violation: {:?}", result);
             }
        }
        Err(e) => println!("Pipeline Failed: {:?}", e),
    }

    Ok(())
}
