// Migrated from examples/pipeline/credit_decision.tp
// Demonstrates pipeline with constraints, validation values, and attribute annotations.

use serde::{Serialize, Deserialize};
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditApplication {
    income: i64,
    credit_history: i64,
    // ... outros campos
}

fn assess_risk(score: i64) -> i64 {
    // Original: fn assess_risk(x: i64): i64 { return x; }
    score
}

// Rust DSL pipeline definition
// The @deterministic attribute is implicit in the Rust DSL; constraints are part of the macro
pipeline! {
    name: CreditDecision,
    input: i64,
    constraints: [
        metric("approval_rate").ge(0.80),
        metric("avg_score").ge(650),
        metric("risk_rate").le(0.20)
    ],
    steps: [
        step("enrich") { input },
        step("score")  { assess_risk(input) },
        step("decide") { input },
    ],
    // Validation values in .tp are replaced by a separate test setup in Rust DSL.
    // To test constraints, you set `validation` context in the test, not in pipeline definition.
}

fn main() {
    let plan = CreditDecision::new();
    let engine = Executor::new();
    let applicant_score = 720;
    let result = engine.run(&plan, &applicant_score).expect("execution failed");
    println!("Pipeline passed: {}", result.passed);
    for (metric, value) in &result.values {
        println!("{} = {:?}", metric, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::Executor;
    use serde_json::json;

    #[test]
    fn test_credit_decision_passes() {
        let plan = CreditDecision::new();
        let engine = Executor::new();
        // Simulate validation context: expected values set by test harness
        // In Rust DSL, constraints check result.metrics against thresholds.
        // Here: score output = 720 -> passes avg_score >= 650
        let input = 720;
        let result = engine.run(&plan, &input).unwrap();
        // As the pipeline just passes through input, `score` metric will be 720
        // Constraint avg_score >= 650 passes, approval_rate and risk_rate are not set by steps
        // So constraint evaluation depends on which metrics are present
        // The pipeline definition doesn't provide validation values; those are external
        assert!(result.passed);
    }

    #[test]
    fn test_constraint_failure() {
        // CreditDecision has constraint avg_score >= 650. If score outputs 600, should fail.
        pipeline! {
            name: TestFailure,
            input: i64,
            constraints: [
                metric("result").ge(100)
            ],
            steps: [
                step("low") { 50 }
            ]
        }

        let plan = TestFailure::new();
        let engine = Executor::new();
        let result = engine.run(&plan, &()).unwrap();
        assert!(!result.passed);
        assert_eq!(result.values.get("low"), Some(&json!(50)));
    }
}
