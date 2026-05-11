use serde::{Deserialize, Serialize};
use tupa_core::pipeline;
use tupa_engine::Executor;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Trade {
    size: f64,
    price: f64,
}

fn risk(trade: &Trade) -> f64 {
    trade.size * trade.price / 1e6
}

pipeline! {
    name: RiskPipeline,
    input: Trade,
    steps: [
        step("risk") { risk(input) }
    ],
    constraints: [
        metric("risk").le(10.0)
    ]
}

fn main() {
    let plan = RiskPipeline::new();
    let engine = Executor::new();
    let trade = Trade {
        size: 100.0,
        price: 50.0,
    };
    // Execute via engine
    let result = engine.run(&plan, &trade).expect("execution failed");
    println!("Pipeline result: passed = {}", result.passed);
    for (metric, value) in &result.values {
        println!("{} = {:?}", metric, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tupa_core::pipeline;
    use tupa_engine::{EngineError, Executor, ParallelPipeline};

    #[test]
    fn test_metadata() {
        let plan = RiskPipeline::new();
        assert_eq!(plan.step_ids(), &["risk"]);
        assert_eq!(plan.produces("risk"), &["risk"]);
        assert_eq!(plan.requires("risk"), &[] as &[&str]);
    }

    #[test]
    fn test_risk_passes() {
        let plan = RiskPipeline::new();
        let engine = Executor::new();
        let trade = Trade {
            size: 10.0,
            price: 50.0,
        }; // risk = 500/1e6 = 0.0005 < 10
        let result = engine.run(&plan, &trade).unwrap();
        assert!(result.passed);
        assert_eq!(
            result.values.get("risk").and_then(|v| v.as_f64()),
            Some(0.0005)
        );
    }

    #[test]
    fn test_risk_fails() {
        let plan = RiskPipeline::new();
        let engine = Executor::new();
        // risk = (1_000_000 * 20) / 1e6 = 20 > 10
        let trade = Trade {
            size: 1_000_000.0,
            price: 20.0,
        };
        let result = engine.run(&plan, &trade).unwrap();
        assert!(!result.passed);
        assert_eq!(
            result.values.get("risk").and_then(|v| v.as_f64()),
            Some(20.0)
        );
    }

    #[tokio::test]
    async fn test_cycle_detected() {
        // Pipeline with circular dependency: a requires b, b requires a
        pipeline! {
            name: CyclePipeline,
            input: (),
            steps: [
                step("a") { 1 } requires ["b"],
                step("b") { 2 } requires ["a"]
            ],
            constraints: []
        }

        let plan = CyclePipeline::new();
        let engine = Executor::new();
        let input = ();
        let result = engine.run_parallel(&plan, &input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            EngineError::CycleDetected { steps } => {
                assert!(steps.contains("a") || steps.contains("b"));
            }
            _ => panic!("expected CycleDetected error"),
        }
    }

    #[tokio::test]
    async fn test_unsatisfiable_dependency() {
        // Step requires a metric that no other step produces
        pipeline! {
            name: UnsatisfiablePipeline,
            input: (),
            steps: [
                step("orphan") { 42 } requires ["missing"]
            ],
            constraints: []
        }

        let plan = UnsatisfiablePipeline::new();
        let engine = Executor::new();
        let input = ();
        let result = engine.run_parallel(&plan, &input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            EngineError::CycleDetected { steps } => {
                assert!(steps.contains("orphan"), "orphan step should be stuck");
            }
            _ => panic!("expected CycleDetected error for unsatisfiable dependency"),
        }
    }
}
