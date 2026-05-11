# tupa-engine

**Tupã pipeline executor** — runs pipelines built with `tupa-core`.

## Purpose

Provides the runtime that executes `pipeline!`-defined pipelines: step orchestration, constraint checking, and result collection. Supports both sequential and parallel step execution based on declared dependencies.

**Status:** Alpha (0.9.x). API subject to change before 1.0.

## Usage

### Sequential execution

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Input {
    amount: f64,
    risk_score: f64,
}

fn enrich(input: &Input) -> Input {
    Input { ..*input }
}

fn score(input: &Input) -> f64 {
    input.risk_score * 100.0
}

pipeline! {
    name: MyPipeline,
    input: Input,
    steps: [
        step("enrich") { enrich(input) },
        step("score")  { score(input) }
    ],
    constraints: [
        metric("score").ge(0.0)
    ]
}

fn main() {
    let plan = MyPipeline::new();
    let executor = Executor::new();
    let input = Input { amount: 100.0, risk_score: 0.5 };
    let result = executor.run(&plan, &input).expect("execution failed");
    println!("Passed: {}", result.passed);
}
```

### Parallel execution (Sprint 4)

Steps can run in parallel when they don't share metric dependencies. Declare which metrics each step `produces` and `requires`:

```rust
pipeline! {
    name: ParallelPipeline,
    input: Input,
    steps: [
        // enrich runs first and produces 'enriched' metric
        step("enrich")  { enrich(input) } produces ["enriched"],
        // score and audit both require 'enriched', so they can run in parallel after enrich
        step("score")   { score(input) }  requires ["enriched"] produces ["score_val"],
        step("audit")   { audit(input) }  requires ["enriched"] produces ["audit_ok"],
        // decide waits for both score and audit
        step("decide")  { decide(input) } requires ["score_val", "audit_ok"]
    ],
    constraints: [
        metric("score_val").ge(0.0),
        metric("audit_ok").eq(1.0)
    ]
}
```

Use `Executor::run_parallel` (requires Tokio runtime):

```rust
#[tokio::main]
async fn main() {
    let plan = ParallelPipeline::new();
    let executor = Executor::new();
    let result = executor.run_parallel(&plan, &input).await?;
}
```

The engine automatically schedules steps based on their declared `produces`/`requires` metric dependencies, executing independent steps concurrently.

## Annotation syntax

- `produces["metric1", "metric2"]` — metrics this step outputs (used by later steps)
- `requires["metricA", "metricB"]` — metrics this step needs from earlier steps

Steps without annotations are considered independent and may run at any time.

## CLI Integration

Use `cargo-tupa` to run your pipeline:

```bash
# Install
cargo install cargo-tupa

# Check your pipeline
cargo tupa check

# Run with JSON input
TUPA_INPUT='{"amount":1000}' cargo tupa run

# Run an example
cargo tupa run --example minimal

# Run with parallel execution
TUPA_PARALLEL=1 cargo tupa run --example fraud_complete
```

Your binary should read `TUPA_INPUT` from the environment:

```rust
use std::env;
let input: Input = if let Ok(json) = env::var("TUPA_INPUT") {
    serde_json::from_str(&json).unwrap_or_else(|_| Input::default())
} else {
    Input::default()
};
```

## Crates

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
- Docs: [docs.rs/tupa-engine](https://docs.rs/tupa-engine)
