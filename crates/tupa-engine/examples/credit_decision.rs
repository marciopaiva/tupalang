// Port of examples/pipeline/credit_decision.tp to Rust DSL.

use serde::{Deserialize, Serialize};
use std::env;
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Input(pub i64);

fn assess_risk(x: i64) -> i64 {
    x
}

fn enrich(input: &Input) -> i64 {
    input.0
}

fn score(input: &Input) -> i64 {
    assess_risk(input.0)
}

fn decide(input: &Input) -> i64 {
    input.0
}

fn approval_rate(_input: &Input) -> f64 {
    0.9
}

fn avg_score(_input: &Input) -> f64 {
    720.0
}

fn risk_rate(_input: &Input) -> f64 {
    0.1
}

pipeline! {
    name: CreditDecision,
    input: Input,
    steps: [
        step("enrich")        { enrich(input) } produces ["enriched"],
        step("score")         { score(input) }  requires ["enriched"] produces ["score_val"],
        step("decide")        { decide(input) } requires ["score_val"] produces ["decision"],
        step("approval_rate") { approval_rate(input) },
        step("avg_score")     { avg_score(input) },
        step("risk_rate")     { risk_rate(input) }
    ],
    constraints: [
        metric("approval_rate").ge(0.80),
        metric("avg_score").ge(650.0),
        metric("risk_rate").le(0.20)
    ]
}

#[tokio::main]
async fn main() {
    let input: Input = if let Ok(json_str) = env::var("TUPA_INPUT") {
        if json_str.is_empty() {
            Input(42)
        } else {
            serde_json::from_str(&json_str).unwrap_or(Input(42))
        }
    } else {
        Input(42)
    };

    let plan = CreditDecision::new();
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
        let plan = CreditDecision::new();
        assert_eq!(
            plan.step_ids(),
            &[
                "enrich",
                "score",
                "decide",
                "approval_rate",
                "avg_score",
                "risk_rate"
            ]
        );
        assert_eq!(plan.produces("enrich"), &["enriched"]);
        assert_eq!(plan.requires("score"), &["enriched"]);
        assert_eq!(plan.produces("score"), &["score_val"]);
        assert_eq!(plan.requires("decide"), &["score_val"]);
        assert_eq!(plan.produces("decide"), &["decision"]);
        // Steps without explicit produces default to their step id as metric name
        assert_eq!(plan.produces("approval_rate"), &["approval_rate"]);
        assert_eq!(plan.produces("avg_score"), &["avg_score"]);
        assert_eq!(plan.produces("risk_rate"), &["risk_rate"]);
    }

    #[tokio::test]
    async fn test_credit_decision_passes() {
        let plan = CreditDecision::new();
        let engine = Executor::new();
        let input = Input(100);
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        assert!(result.passed);
    }

    #[tokio::test]
    async fn test_credit_decision_values() {
        let plan = CreditDecision::new();
        let engine = Executor::new();
        let input = Input(50);
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        println!("All values: {:?}", result.values);
        // Check all expected keys exist
        let expected_keys = [
            "enriched",
            "score_val",
            "decision",
            "approval_rate",
            "avg_score",
            "risk_rate",
        ];
        for key in &expected_keys {
            assert!(result.values.contains_key(*key), "missing key {}", key);
        }
        assert_eq!(
            result.values.get("score_val").and_then(|v| v.as_i64()),
            Some(50)
        );
        assert_eq!(
            result.values.get("decision").and_then(|v| v.as_i64()),
            Some(50)
        );
        assert_eq!(
            result.values.get("enriched").and_then(|v| v.as_i64()),
            Some(50)
        );
    }

    #[tokio::test]
    async fn test_credit_decision_constraint_failure() {
        // Define a variant with a failing approval_rate
        fn failing_approval_rate(_input: &Input) -> f64 {
            0.5 // below 0.80 threshold
        }

        pipeline! {
            name: CreditDecisionFail,
            input: Input,
            steps: [
                step("enrich")        { enrich(input) } produces ["enriched"],
                step("score")         { score(input) }  requires ["enriched"] produces ["score_val"],
                step("decide")        { decide(input) } requires ["score_val"] produces ["decision"],
                step("approval_rate") { failing_approval_rate(input) },
                step("avg_score")     { avg_score(input) },
                step("risk_rate")     { risk_rate(input) }
            ],
            constraints: [
                metric("approval_rate").ge(0.80),
                metric("avg_score").ge(650.0),
                metric("risk_rate").le(0.20)
            ]
        }

        let plan = CreditDecisionFail::new();
        let engine = Executor::new();
        let input = Input(100);
        let result = engine.run_parallel(&plan, &input).await.unwrap();
        assert!(!result.passed, "constraint on approval_rate should fail");
    }
}
