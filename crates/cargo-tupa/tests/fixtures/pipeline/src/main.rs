use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input;

fn step1(_: &Input) -> i32 { 10 }
fn step2(_: &Input) -> i32 { 20 }

pipeline! {
    name: TestPipeline,
    input: Input,
    steps: [
        step("s1") { step1(input) },
        step("s2") { step2(input) }
    ],
    constraints: []
}

#[tokio::main]
async fn main() {
    let engine = Executor::from_env();
    let plan = TestPipeline::new();
    let result = engine.run_parallel(&plan, &Input).await.unwrap();
    println!("passed: {}", result.passed);
}
