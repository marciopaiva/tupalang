# tupa-engine

**Tupã pipeline executor** — runs pipelines built with `tupa-core`.

## Overview

This crate provides the runtime for executing `pipeline!`-defined pipelines:

- Step orchestration with dependency-based scheduling
- Sequential execution via `Executor::run`
- Parallel execution via `Executor::run_parallel` (Tokio-based)
- Constraint checking with metric evaluation
- Step metrics collection and cancellation support

**Status:** Alpha (0.9.x). API subject to change before 1.0.

## Installation

```toml
[dependencies]
tupa-engine = "0.9"
```

## Quick Example

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

## Parallel Execution

Steps can run in parallel when they don't share metric dependencies:

```rust
pipeline! {
    name: ParallelPipeline,
    input: Input,
    steps: [
        step("enrich") { enrich(input) } produces ["enriched"],
        step("score")  { score(input) }  requires ["enriched"] produces ["score_val"],
        step("audit")  { audit(input) }  requires ["enriched"] produces ["audit_ok"],
        step("decide") { decide(input) } requires ["score_val", "audit_ok"]
    ],
    constraints: [
        metric("score_val").ge(0.0),
        metric("audit_ok").eq(1.0)
    ]
}
```

```rust
#[tokio::main]
async fn main() {
    let plan = ParallelPipeline::new();
    let executor = Executor::new();
    let result = executor.run_parallel(&plan, &input).await?;
}
```

## Annotation Syntax

- `produces["metric1", "metric2"]` — metrics this step outputs
- `requires["metricA", "metricB"]` — metrics this step needs

Steps without annotations are considered independent.

## API

- `Executor::new()` — create executor
- `Executor::run(&plan, &input)` — sequential execution
- `Executor::run_parallel(&plan, &input)` — parallel execution (async)
- `Executor::cancel()` — cancel running pipeline
- `Executor::from_env()` — create from environment variables

## License

Apache-2.0

## Links

- [Source](https://github.com/marciopaiva/tupalang)
- [Documentation](https://docs.rs/tupa-engine)
