// Port of examples/pipeline/minimal.tp to Rust DSL (with explicit produces/requires for parallel testing).

use serde::{Deserialize, Serialize};
use std::env;
use tupa_core::pipeline;
use tupa_engine::Executor;

// Minimal input: no data
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Unit;

fn enrich(_input: &Unit) -> Unit {
    Unit
}

fn score(_input: &Unit) -> i64 {
    42
}

pipeline! {
    name: FraudDetection,
    input: Unit,
    steps: [
        // enrich produces a dummy metric
        step("enrich") { enrich(input) } produces ["dummy"],
        // score requires the dummy metric produced by enrich
        step("score")  { score(input) } requires ["dummy"]
    ],
    constraints: []
}

#[tokio::main]
async fn main() {
    // Support TUPA_INPUT environment variable for cargo-tupa integration
    let input: Unit = if let Ok(json_str) = env::var("TUPA_INPUT") {
        if json_str.is_empty() {
            Unit
        } else {
            serde_json::from_str(&json_str).unwrap_or(Unit)
        }
    } else {
        Unit
    };

    let plan = FraudDetection::new();
    let engine = Executor::new();
    let result = engine
        .run_parallel(&plan, &input)
        .await
        .expect("execution failed");
    println!("Pipeline passed: {}", result.passed);
    for (k, v) in &result.values {
        println!("{} = {:?}", k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::{Executor, ParallelPipeline};

    #[test]
    fn test_metadata() {
        let plan = FraudDetection::new();
        assert_eq!(plan.step_ids(), &["enrich", "score"]);
        assert_eq!(plan.produces("enrich"), &["dummy"]);
        assert_eq!(plan.requires("enrich"), &[] as &[&str]);
        assert_eq!(plan.produces("score"), &["score"]);
        assert_eq!(plan.requires("score"), &["dummy"]);
    }

    #[tokio::test]
    async fn test_execution() {
        let plan = FraudDetection::new();
        let engine = Executor::new();
        let input = Unit;
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        assert!(result.passed);
        assert_eq!(
            result.values.get("score"),
            Some(&serde_json::Value::from(42))
        );
    }
}
