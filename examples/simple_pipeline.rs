// Simple end-to-end example of Tupã pipeline using the Rust DSL.

use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
struct Trade {
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
    let trade = Trade { size: 100.0, price: 50.0 };
    let result = engine.run(&plan, &trade).expect("execution failed");
    println!("Pipeline result: passed = {}", result.passed);
    for (metric, value) in &result.values {
        println!("{} = {:?}", metric, value);
    }
}
