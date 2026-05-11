// Port of examples/pipeline/fraud_complete.tp to Rust DSL.

use serde::{Deserialize, Serialize};
use std::env;
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    Transfer {
        amount: f64,
        from: String,
        to: String,
    },
    Withdrawal {
        amount: f64,
        location: String,
    },
}

fn enrich(input: &Transaction) -> (f64, String) {
    match input {
        Transaction::Transfer { amount, from, .. } => (*amount, from.clone()),
        Transaction::Withdrawal { amount, location } => (*amount, location.clone()),
    }
}

fn score(input: &Transaction) -> i64 {
    match input {
        Transaction::Transfer { amount, .. } => (*amount * 0.01) as i64,
        Transaction::Withdrawal { amount, .. } => (*amount * 0.02) as i64,
    }
}

fn decide(input: &Transaction) -> bool {
    match input {
        Transaction::Transfer { .. } => false,
        Transaction::Withdrawal { .. } => true,
    }
}

fn false_negative_rate(_input: &Transaction) -> f64 {
    0.049
}

fn false_positive_rate(_input: &Transaction) -> f64 {
    0.009
}

pipeline! {
    name: FraudDetection,
    input: Transaction,
    steps: [
        step("enrich")              { enrich(input) } produces ["amount", "entity"],
        step("score")               { score(input) }  requires ["amount"] produces ["fraud_score"],
        step("decide")              { decide(input) } requires ["fraud_score"] produces ["is_fraud"],
        step("false_negative_rate") { false_negative_rate(input) },
        step("false_positive_rate") { false_positive_rate(input) }
    ],
    constraints: [
        metric("false_negative_rate").le(0.05),
        metric("false_positive_rate").le(0.01)
    ]
}

#[tokio::main]
async fn main() {
    let input: Transaction = if let Ok(json_str) = env::var("TUPA_INPUT") {
        if json_str.is_empty() {
            Transaction::Transfer {
                amount: 1000.0,
                from: "Alice".into(),
                to: "Bob".into(),
            }
        } else {
            serde_json::from_str(&json_str).unwrap_or_else(|_| Transaction::Transfer {
                amount: 1000.0,
                from: "Alice".into(),
                to: "Bob".into(),
            })
        }
    } else {
        Transaction::Transfer {
            amount: 1000.0,
            from: "Alice".into(),
            to: "Bob".into(),
        }
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
        assert_eq!(
            plan.step_ids(),
            &[
                "enrich",
                "score",
                "decide",
                "false_negative_rate",
                "false_positive_rate"
            ]
        );
        assert_eq!(plan.produces("enrich"), &["amount", "entity"]);
        assert_eq!(plan.requires("score"), &["amount"]);
        assert_eq!(plan.produces("score"), &["fraud_score"]);
        assert_eq!(plan.requires("decide"), &["fraud_score"]);
        assert_eq!(plan.produces("decide"), &["is_fraud"]);
        assert_eq!(
            plan.produces("false_negative_rate"),
            &["false_negative_rate"]
        );
        assert_eq!(
            plan.produces("false_positive_rate"),
            &["false_positive_rate"]
        );
    }

    #[tokio::test]
    async fn test_fraud_passes() {
        let plan = FraudDetection::new();
        let engine = Executor::new();
        let input = Transaction::Transfer {
            amount: 500.0,
            from: "A".into(),
            to: "B".into(),
        };
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        assert!(result.passed);
    }

    #[tokio::test]
    async fn test_fraud_values() {
        let plan = FraudDetection::new();
        let engine = Executor::new();
        let input = Transaction::Withdrawal {
            amount: 100.0,
            location: "ATM".into(),
        };
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        // enrich produces (amount, entity) as a tuple array
        let amount_val = result.values.get("amount").unwrap();
        let entity_val = result.values.get("entity").unwrap();
        // Should be array [amount, location]
        if let Some(arr) = amount_val.as_array() {
            assert_eq!(arr[0].as_f64(), Some(100.0));
        } else {
            panic!("amount is not an array");
        }
        if let Some(arr) = entity_val.as_array() {
            assert_eq!(arr[1].as_str(), Some("ATM"));
        } else {
            panic!("entity is not an array");
        }
        // fraud_score from score step
        assert_eq!(
            result.values.get("fraud_score").and_then(|v| v.as_i64()),
            Some(2)
        ); // 100*0.02
           // is_fraud from decide
        assert_eq!(
            result.values.get("is_fraud").and_then(|v| v.as_bool()),
            Some(true)
        );
    }
}
