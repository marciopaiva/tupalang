# Performance Tuning

Guide to optimizing Tupã pipeline execution.

## Parallel Execution

### Enable Tokio Runtime

Parallel step execution (`Executor::run_parallel`) requires a Tokio runtime. Ensure your binary uses `#[tokio::main]` or manually creates a runtime:

```rust
#[tokio::main]
async fn main() {
    let plan = MyPipeline::new();
    let engine = Executor::new();
    let result = engine.run_parallel(&plan, &input).await?;
}
```

### Step Dependency Annotation

Accurate `produces` and `requires` annotations enable maximal parallelism. Over-specifying dependencies serializes execution.

```rust
pipeline! {
    steps: [
        step("fetch")   { fetch_data(input) }  produces ["raw"],
        step("parse")   { parse(&raw) }       requires ["raw"] produces ["parsed"],
        step("validate"){ validate(&parsed) } requires ["parsed"],
        // independent metrics can run in parallel with parsing
        step("log_count") { count_logs(input) }
    ]
}
```

### Parallelism Degree

Tokio's default worker threads = number of CPU cores. Override via `tokio::runtime::Builder` if needed:

```rust
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()?;
```

## Memory Efficiency

### Avoid Unnecessary Cloning

Step functions that take `&Input` and return owned values are already optimal. Avoid cloning large structures inside steps; use references when possible.

```rust
fn process(data: &LargeStruct) -> Metric {
    // borrow, don't clone
    data.compute_metric()
}
```

### Reuse Input Data

If multiple steps need the same derived data, compute once in an early step and produce a metric for downstream steps.

## Plugin FFI Overhead

Dynamic plugin calls (`PluginManager::call`) incur FFI transition cost (serialization/deserialization + C call). For high-throughput scenarios:

- Batch operations: design plugins that accept arrays of inputs and return arrays of outputs.
- Keep plugin logic lightweight; offload heavy work to Rust-native steps when possible.
- Profile with `cargo bench --bench plugin_bench` to measure overhead.

Expected overhead: ~0.5–2µs per call on x86_64 (varies by input size). If this is significant, consider in-process step functions instead of FFI.

## Benchmarking

Use `criterion` benchmarks to measure performance:

```bash
# Engine benchmarks
cargo bench --bench engine_bench -p tupa-engine

# Plugin FFI benchmarks
cargo bench --bench plugin_bench -p tupa-plugin
```

Key metrics to monitor:

- Sequential step throughput (steps/sec)
- Parallel speedup vs sequential (ideal: near-linear for independent steps)
- Constraint checking overhead (per constraint)
- Plugin call latency (µs)

## Constraint Optimization

Constraints are evaluated after all steps complete. For pipelines with many steps and constraints:

- Place constraints on step-produced metrics directly (avoid recomputing derived values).
- Use simple comparison operators (`ge`, `le`, `eq`, `ne`, `gt`, `lt`) — they are optimized.
- Avoid expensive computations inside constraint expressions; compute once in a step and reference the metric.

## Profile-Guided Optimization (PGO)

For production-critical pipelines:

```bash
# 1. Build with instrumentation
cargo build --release -p tupa-engine --profile=pgo-instrument

# 2. Run representative workload to collect profiles
./target/release/my_pipeline < input.json

# 3. Build with PGO data
cargo build --release -p tupa-engine --profile=pgo-opt
```

## Channel Configuration (Advanced)

The engine uses unbounded MPSC channels for step completion notifications. For extremely high step counts (1000+), consider:

- Tuning Tokio channel semantics (currently unbounded, backpressure not applied).
- Batching metric writes in steps that produce many values.

## Common Pitfalls

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| Parallel run no faster than sequential | Over-specified dependencies (false dependencies) | Audit `requires`/`produces`; keep minimal |
| High memory usage | Steps retain large allocations after execution | Drop heavy values at step end; use scoped allocations |
| Plugin calls slow | Frequent small FFI calls | Batch calls or move logic into native steps |

## Further Reading

- Tokio runtime docs: <https://docs.rs/tokio/latest/tokio/runtime/>
- Criterion book: <https://bheisler.github.io/criterion.rs/book/>
- Rust Performance Book: <https://nnethercote.github.io/perf-book/>
