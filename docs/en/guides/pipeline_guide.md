# Pipeline Guide: Building Production Policy Pipelines in Rust

**Prerequisite:** [Getting Started](../guides/getting_started.md)

This guide covers the full capabilities of the Tupã pipeline DSL as a Rust crate.

---

## Anatomy of a Pipeline

```rust
use tupa_core::{pipeline, step, constraint, metric};

pipeline! {
    /// Human-readable name (optional doc comment)
    name: MyPolicy,
    
    /// Input type — any Rust type that is `Send + Sync + 'static`
    input: Trade,
    
    /// Steps execute sequentially by default (or concurrently with `@parallel`)
    steps: [
        /// Each step has an identifier and a pure expression
        step("validate") { validate_trade(input) },
        step("score") { compute_score(input) },
        step("check_limits") { input.size <= MAX_SIZE }
    ],
    
    /// Compile-time and runtime constraints
    constraints: [
        /// `metric("name").operator(constant_value)`
        metric("risk_score").le(0.8),
        metric("throughput").ge(100.0)
    ],
    
    /// Optional: attributes applied to the pipeline itself
    /// (e.g., @deterministic, @audit, @version("1.0"))
    /// Attributes go after the name but before body
}
```text

**Expands to:**

```rust
struct MyPolicy;
impl Pipeline for MyPolicy {
    type Input = Trade;
    fn name(&self) -> &'static str { "MyPolicy" }
    fn steps(&self) -> &'static [StepDescriptor] { ... }
    fn constraints(&self) -> &'static [ConstraintDescriptor] { ... }
    fn run_step(&self, ctx: &mut StepContext, step: &str, input: &Trade) -> Result<Value, Error> { ... }
}
```text

---

## Step Functions

### Pure Steps

By default, step expressions must be **pure** (no side effects). They can:

- Call other pure functions
- Perform arithmetic, comparisons, pattern matching
- Construct values (tuples, enums, tensors)

```rust
fn compute_fee(amount: f64) -> f64 {
    amount * 0.001  // 0.1% fee
}
```text

Should not:

- Print to stdout (use logging via `@side_effects(io)` if needed)
- Read files/network
- Use `rand()` or current time

### Impure Steps

Mark with `#[tupa::side_effects(io)]`:

```rust
#[tupa::side_effects(io)]
fn log_decision(decision: &str) {
    println!("[AUDIT] {}", decision);
}
```text

Use sparingly — impure steps cannot be used in gradient computations.

### Async Steps

Mark with `async`:

```rust
async fn fetch_price(symbol: &str) -> f64 {
    let resp = reqwest::get(&format!("https://api.example.com/price/{}", symbol)).await?;
    resp.json().await
}
```text

Pipeline executor supports async steps automatically (requires `tokio` runtime).

### Step Attributes

```rust
step("expensive") {
    heavy_computation(input)
} @parallel  // run in parallel with other @parallel steps (experimental)
```text

Attributes:

- `@parallel` — can execute concurrently with other parallel steps (no data dependencies assumed)
- `@timeout(ms)` — fail if step takes longer than X ms
- `@retry(n)` — automatically retry up to N times on transient errors

---

## Constraints

Constraints enforce policy rules. They are checked **both at compile time (when possible) and at runtime**.

### Basic Operators

| Operator | Meaning | Example |
|---|---|---|
| `.lt(val)` | less than | `metric("latency").lt(50.0)` |
| `.le(val)` | ≤ | `metric("drawdown").le(0.15)` |
| `.eq(val)` | == | `metric("version").eq(1)` |
| `.ge(val)` | ≥ | `metric("sharpe").ge(1.5)` |
| `.gt(val)` | > | `metric("profit").gt(0.0)` |

### Compile-Time Proving

If the constraint expression is a constant or constant-foldable, the `pipeline!` macro proves it at compile time:

```rust
pipeline! {
    name: ConstantCheck,
    input: (),
    steps: [ step("x") { 42 } ],
    constraints: [
        metric("x").eq(42)  // ✅ proven at compile time
    ]
}
```text

If you change `42` to `43`, compilation fails:

```text
error[E3002]: cannot prove constraint at compile time: x == 43
    --> src/lib.rs:12:5
     |
12  |         metric("x").eq(43)
     |         ^^^^^^^^^^^^^^^^^^
```text

This is the same mechanism as `Safe<f64, !nan>` — the compiler tries to prove the predicate via constant folding.

### Runtime Checking

Dynamic values (from input or step outputs) are checked at runtime:

```rust
pipeline! {
    name: DynamicCheck,
    input: Trade,
    steps: [ step("size_ok") { input.size <= MAX } ],
    constraints: [
        metric("size_ok").eq(true)  // checked at runtime
    ]
}
```text

If constraint fails, pipeline returns `ConstraintFailure` result.

### Computed Thresholds

Thresholds can be any Rust expression — the `input` variable is in scope:

```rust
pipeline! {
    name: DynamicThreshold,
    input: RiskParams,
    steps: [ step("equity_floor") { input.account_equity_usdt } ],
    constraints: [
        metric("equity_floor").ge(input.min_equity_threshold)
    ]
}
```

### Fail-Fast Constraints

Add `.fail_fast()` to abort immediately on violation — useful for hard invariants:

```rust
constraints: [
    metric("equity_floor").ge(0.0).fail_fast(),  // abort if negative
    metric("score").le(input.max_score),          // only checked if above passes
]
```

Without `.fail_fast()`, all constraints are evaluated and all failures collected. With it, the pipeline stops at the first violation.

---

## StepContext — Reading Prior Step Outputs

Every step body has access to `ctx: &StepContext`, which carries the outputs of upstream steps. This lets later steps read prior results without re-running them.

```rust
pipeline! {
    name: ScoringPipeline,
    input: MarketData,
    steps: [
        step("base_score") { compute_base(input) },
        step("adjusted_score") {
            // read the output of "base_score" from context
            let base = ctx.get_f64("base_score").unwrap_or(0.0);
            base * input.volatility_multiplier
        } requires ["base_score"],
    ],
    constraints: []
}
```

### StepContext API

| Method | Return | Description |
|--------|--------|-------------|
| `ctx.get("name")` | `Option<&Value>` | Raw JSON value |
| `ctx.get_f64("name")` | `Option<f64>` | Parse as f64 |
| `ctx.get_bool("name")` | `Option<bool>` | Parse as bool |
| `ctx.get_str("name")` | `Option<&str>` | Parse as &str |
| `ctx.get_as::<T>("name")` | `Option<T>` | Deserialize into T |

### Declaring Dependencies

Use `requires ["step_name"]` so the parallel executor knows to thread the output:

```rust
step("decision") {
    let score = ctx.get_f64("score").unwrap_or(0.0);
    let valid = ctx.get_bool("validate").unwrap_or(false);
    if valid && score > threshold { "ENTER" } else { "HOLD" }
} requires ["score", "validate"]
```

If `requires` is omitted, `ctx` may be empty (the step runs concurrently with others). The `ctx` parameter is always present — just use `_ctx` if you don't need it.

---

## Metrics

A **metric** is a named value produced by a step (or the main expression). It can be used in constraints.

```rust
pipeline! {
    name: FullMetrics,
    input: MarketData,
    steps: [
        step("sma") { sma(input.prices, 20) },
        step("rsi") { rsi(input.prices, 14) },
        step("signal") {
            // this value becomes metric "signal"
            if sma > rsi { 1 } else { -1 }
        }
    ],
    constraints: [
        // refer to metrics by name
        metric("sma").gt(0.0),
        metric("rsi").le(70.0)  // not overbought
    ]
}
```text

**Implicit metric naming:** The step identifier becomes the metric name automatically. Explicit override:

```rust
step("volatility") { compute_vol(input) } -> metric("risk")
```text

(if step returns a value, it's bound to `"risk"` metric name)

---

## Input & Output Types

### Input

```rust
pipeline! {
    input: MyInputStruct,  // any Rust type
    ...
}
```text

The input type must be:

- `Send + Sync + 'static` (or implement `PipelineInput` trait)
- Usually a struct or enum with `derive(Serialize, Deserialize)` if coming from JSON

### Output

Pipeline execution returns:

```rust
struct PipelineResult {
    values: HashMap<String, Value>,   // all metric values
    passed: bool,                     // all constraints satisfied
    failures: Vec<ConstraintFailure>, // details if any failed
}
```text

You can extract typed values:

```rust
// Typed accessors (0.11.0+)
let profit: Option<f64>  = result.get_f64("profit");
let passed: Option<bool> = result.get_bool("validate");
let label: Option<&str>  = result.get_str("label");
let decision: Option<MyDecision> = result.get_as::<MyDecision>("decision");

// Direct map access still works
let raw: &Value = &result.values["profit"];
```text

Better: define a custom result type via associated type on `Pipeline` trait (advanced).

---

## Async Steps & Runtime

If your pipeline uses async steps, you need a runtime:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let policy = MyPolicy::new();
    let mut executor = tupa_engine::Executor::new()
        .with_runtime(tokio::runtime::Handle::current()); // use current tokio runtime
    
    let input = Trade::default();
    let result = executor.run_async(policy, &input).await?;
    Ok(())
}
```text

Without async steps, `Executor::run()` (sync) is fine.

---

## Concurrency Model

### Step Ordering

By default, steps execute **sequentially** in declaration order.

### Parallel Steps

Use `@parallel` attribute to enable concurrent execution (no data dependencies between steps):

```rust
pipeline! {
    name: ParallelExample,
    input: LargeBatch,
    steps: [
        step("fetch_a") { fetch_a(input) } @parallel,
        step("fetch_b") { fetch_b(input) } @parallel,
        // This step sees results from both A and B
        step("combine") { combine(fetch_a_result, fetch_b_result) }
    ],
    constraints: []
}
```text

The executor schedules `fetch_a` and `fetch_b` concurrently; `combine` waits for both.

**Note:** Parallel steps must not mutate shared state — they are `&self` pure functions.

---

## Tensors & Numeric Types

Tupã provides tensor types with compile-time shape information.

```rust
use tupa_core::Tensor;

type Image = Tensor<f32, { shape: [28, 28], density: 1.0 }>;

pipeline! {
    name: Inference,
    input: Image,
    steps: [
        step("conv") { conv2d(input, kernel) },
        step("pool") { max_pool(conv) }
    ],
    constraints: []
}
```text

**Features:**

- Compile-time shape checking when dimensions are constants
- Runtime validation for dynamic shapes (`...` in `.tp` — future)
- Sparsity hint via `density` (guides backend optimizations)

Currently uses CPU backend; GPU via `burn` integration coming in `tupa-ml` crate.

---

## Safe Types & Constraints

The `Safe<T, C>` type wraps a value and proves at compile time that a constraint holds.

Example: `Safe<f64, !nan>` proves the value is never NaN.

```rust
use tupa_core::{Safe, Constraint};

let x: Safe<f64, !nan> = Safe::new(3.14);  // OK
// let y: Safe<f64, !nan> = Safe::new(f64::NAN); // ❌ Compile error

// The compiler propagates Safe through functions:
fn process(x: Safe<f64, !nan>) -> Safe<f64, !nan> {
    x.map(|v| v * 2.0)  // preserves !nan
}
```text

**Available constraints** (see SPEC section 3.2.6):

| Constraint | Base type | Proven via |
|---|---|---|
| `!nan` | `f64` | Constant folding or interval analysis |
| `!inf` | `f64` | Bounds checking |
| `!hate_speech` | `string` | RLHF score (future) |
| `!misinformation` | `string` | RLHF score (future) |

---

## Gradients (Automatic Differentiation)

The `grad!` macro generates the backward pass for pure functions.

```rust
use tupa_core::grad;

// Simple scalar function
let f = |x: f64| x * x + 2.0 * x;
let df = grad!(f);

let (y, dy) = df(3.0);  // y=15.0, dy=8.0 (derivative)
```text

**Rules:**

- Function must be pure (no I/O, no global mutation, no randomness)
- Works for multi-argument functions: `grad!(|x, y| x * y)(2.0, 3.0)` → `(3.0, 2.0)`
- Returns tuple of partial derivatives matching argument count

Under the hood: `grad!` uses symbolic differentiation via `tupa-ad` crate (not yet published).

---

## Plugins & Dynamic Steps

For runtime-extensible steps, use `tupa-plugin`:

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/my_steps.so")?;  // shared library
```text

Plugin functions are invoked via `pm.call("function_name", input)` inside a step body:

```rust
pipeline! {
    steps: [
        step("plugin_step") {
            let result = pm.call("my_step", json!(input))?;
            result
        }
    ]
}
```text

See [tupa-plugin crate docs](../../crates/tupa-plugin/README.md) for the full API.

---

## Error Handling

### Pipeline Definition Errors

Caught at **compile time** by `pipeline!` macro:

```text
error[E2005]: step function not found: unknown_function
  --> src/lib.rs:15:20
   |
15 |         step("bad") { unknown_function(input) }
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
```text

### Runtime Errors

```rust
let res = executor.run(policy, &input);
match res {
    Ok(out) => println!("passed"),
    Err(tupa_engine::Error::ConstraintFailed { metric, expected, actual }) => {
        eprintln!("Constraint {} failed: expected {}, got {}", metric, expected, actual);
    }
    Err(tupa_engine::Error::StepPanic { step, reason }) => {
        eprintln!("Step {} panicked: {}", step, reason);
    }
}
```text

---

## Testing Strategies

### Unit Test a Step

```rust
#[test]
fn test_compute_fee() {
    assert_eq!(compute_fee(100.0), 0.1);
}
```text

### Integration Test Full Pipeline

```rust
#[tokio::test]
async fn test_policy_end_to_end() {
    let policy = MyPolicy::new();
    let engine = Executor::new();
    let input = build_test_input();
    let res = engine.run(policy, &input).unwrap();
    assert!(res.passed);
}
```text

### Property-Based Testing

Combine with `proptest`:

```rust
proptest! {
    #[test]
    fn fee_always_positive(amount in 1..=1_000_000u64) {
        let fee = compute_fee(amount as f64);
        prop_assert!(fee >= 0.0);
    }
}
```text

---

## Performance Tips

1. **Executor reuse:** Creating `Executor` allocates channels; reuse when possible
2. **Avoid unnecessary clones:** Pass `&T` to steps when possible (input is `&Input`)
3. **Use `@parallel` for independent steps:** Overheads low; benefits high for I/O-bound steps
4. **Profile:** `cargo flamegraph --bin your-binary` shows engine hot spots

---

## Executor Configuration

### Environment Variables

The executor can be configured via environment variables:

- `TUPA_STEP_TIMEOUT` — maximum duration for any single step (e.g., `"30s"`, `"1m"`, `"500ms"`). Steps exceeding this limit will return `EngineError::StepTimeout`.
- `TUPA_CHANNEL_CAPACITY` — capacity of the bounded channel used for step coordination (default: 1000). Increase for high-throughput pipelines with many concurrent steps.

Example:
```rust
use tupa_engine::Executor;

// Reads TUPA_STEP_TIMEOUT and TUPA_CHANNEL_CAPACITY from environment
let executor = Executor::from_env()?;
```

### Metrics Export

Pipeline execution metrics can be exported as JSON for observability and debugging:

```bash
TUPA_METRICS_OUTPUT=metrics.json cargo tupa run
```

Or via CLI flag:

```bash
cargo tupa run --metrics-output metrics.json
```

The output file contains an array of `StepMetrics` objects:

```json
[
  {
    "step_id": "risk",
    "start_nanos": 1700000000000000000,
    "end_nanos": 1700000000005000000,
    "duration_nanos": 5000000,
    "state": "Completed"
  }
]
```

### Cancellation

Long-running pipelines can be cancelled programmatically or via Ctrl+C:

```rust
use tupa_engine::Executor;
use std::time::Duration;

let executor = Executor::from_env()?;
let handle = executor.handle();  // obtains cancellation token

// Spawn a thread that cancels after 5 seconds
std::thread::spawn(move || {
    thread::sleep(Duration::from_secs(5));
    handle.cancel();
});

let result = executor.run(pipeline, &input);
// result will be Err(EngineError::Cancelled) if cancellation triggered
```

When running via `cargo tupa run`, Ctrl+C is automatically handled — the engine will attempt graceful shutdown.

---

## FAQ

**Q: Can I mix `.tp` files with Rust DSL?**  
A: No. The `.tp` toolchain (including `tupa-parser`) was removed in 0.9.0. Port legacy `.tp` pipelines to the `pipeline!` macro — see [TRANSITION.md](../TRANSITION.md).

**Q: How do I debug a pipeline?**  
A: Set `RUST_LOG=tupa_engine=debug` for execution logs, and inspect `PipelineResult::metrics` for per-step timing.

**Q: Are steps really pure?**  
A: The compiler enforces purity for steps marked as `pure` (default). You can override with `#[tupa::side_effects(...)]`, but then the step cannot be used in gradient calculations.

**Q: Can I have nested pipelines?**  
A: Not yet. Pipeline-in-pipeline is planned for v1.1.

**Q: What about stateful steps (counters, caches)?**  
A: Use `@stateful` attribute (future) or externalize state to a shared struct passed as `&mut` to steps (experimental).

---

## Next

- [Reference: SPEC](spec.md) — formal semantics
- [Type Semantics](type_semantics.md) — formal type rules
- [Error Reference](../reference/common_errors.md) — diagnostic codes
- [TRANSITION.md](../TRANSITION.md) — migrating from `.tp`

**You now know how to build production policy pipelines in Rust with Tupã.**
