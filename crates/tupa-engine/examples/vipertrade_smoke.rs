// ViperTrade smoke test — equivalent to examples/pipeline/vipertrade_smoke.tp
// This example verifies that a typical ViperTrade pipeline compiles and runs.

use tupa_core::pipeline;
use tupa_engine::Executor;

fn compute_signal(price: &i64, ema_fast: i64) -> i64 {
    if *price > ema_fast {
        1
    } else {
        0
    }
}

fn compute_position_bps(signal: i64) -> i64 {
    if signal == 1 {
        1200
    } else {
        0
    }
}

pipeline! {
    name: ViperTradeValidation,
    input: i64,
    steps: [
        step("signal") { compute_signal(input, 90) } produces ["signal"],
        step("position") { compute_position_bps(compute_signal(input, 90)) } requires ["signal"] produces ["position", "max_drawdown_bps", "copy_success_rate_bps"]
    ],
    constraints: [
        metric("max_drawdown_bps").le(1500),
        metric("copy_success_rate_bps").ge(9500)
    ]
}

#[tokio::main]
async fn main() {
    let engine = Executor::from_env();
    let plan = ViperTradeValidation::new();
    // Use a sample input price
    let input: i64 = 100;
    let result = engine
        .run_parallel(&plan, &input)
        .await
        .expect("execution failed");
    println!("Smoke test: passed={}", result.passed);
    for (k, v) in &result.values {
        println!("{} = {:?}", k, v);
    }
}
