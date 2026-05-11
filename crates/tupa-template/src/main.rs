use {{crate_name}}::{Input, MyPolicy};
use tupa_engine::{Executor, ParallelPipeline};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read pipeline input from TUPA_INPUT environment variable (JSON)
    // Example: TUPA_INPUT='{"amount":1000.0,"risk_score":0.5}' cargo tupa run
    let input_json = env::var("TUPA_INPUT")
        .unwrap_or_else(|_| "{}".to_string());

    let input: Input = if input_json.is_empty() {
        Input { amount: 0.0, risk_score: 0.0 }
    } else {
        serde_json::from_str(&input_json)
            .expect("Failed to parse TUPA_INPUT as JSON. Expected format: {\"amount\": 1000.0, \"risk_score\": 0.5}")
    };

    let pipeline = MyPolicy::new();
    let executor = Executor::new();

    // Use parallel execution if TUPA_PARALLEL=1
    let parallel = env::var("TUPA_PARALLEL").map(|v| v == "1").unwrap_or(false);

    let result = if parallel {
        executor.run_parallel(&pipeline, &input).await?
    } else {
        executor.run(&pipeline, &input)?
    };

    if result.passed {
        println!("✅ Pipeline passed all constraints");
    } else {
        println!("❌ Pipeline failed constraints:");
        for failure in &result.failures {
            println!("  - {}: expected {} {} {}, got {}",
                failure.metric, failure.operator, failure.expected, failure.actual);
        }
    }

    // Print metric values (useful for debugging)
    for (key, value) in &result.values {
        println!("{} = {}", key, value);
    }

    Ok(())
}
