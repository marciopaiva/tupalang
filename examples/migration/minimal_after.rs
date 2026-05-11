// Migrated from examples/pipeline/minimal.tp
// Demonstrates the Rust DSL equivalent of a simple two-step pipeline.

use serde::{Serialize, Deserialize};
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {}

fn enrich(input: Transaction) -> Transaction {
    // Original: fn enrich(input: Transaction): Transaction { return match input { _ => input }; }
    input
}

fn score(_input: Transaction) -> i64 {
    // Original: fn score(input: Transaction): i64 { return 42; }
    42
}

// Rust DSL pipeline definition
pipeline! {
    name: FraudDetection,
    input: Transaction,
    steps: [
        step("enrich") { enrich(input) },
        step("score")  { score(input) },
    ],
    constraints: []
}

fn main() {
    let plan = FraudDetection::new();
    let engine = Executor::new();
    let tx = Transaction {};
    let result = engine.run(&plan, &tx).expect("execution failed");
    println!("Pipeline passed: {}", result.passed);
    for (metric, value) in &result.values {
        println!("{} = {:?}", metric, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::Executor;

    #[test]
    fn test_metadata() {
        let plan = FraudDetection::new();
        assert_eq!(plan.step_ids(), &["enrich", "score"]);
        assert_eq!(plan.produces("enrich"), &["enrich"]);
        assert_eq!(plan.requires("enrich"), &[] as &[&str]);
        assert_eq!(plan.produces("score"), &["score"]);
        assert_eq!(plan.requires("score"), &["enrich"]);
    }

    #[test]
    fn test_execution() {
        let plan = FraudDetection::new();
        let engine = Executor::new();
        let tx = Transaction {};
        let result = engine.run(&plan, &tx).unwrap();
        assert!(result.passed);
        assert_eq!(result.values.get("enrich"), None); // enrich returns Transaction, not a metric
        assert_eq!(result.values.get("score"), Some(&serde_json::Value::from(42)));
    }
}
