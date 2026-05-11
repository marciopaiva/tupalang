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
```

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
```

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
```

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
```

Use sparingly — impure steps cannot be used in gradient computations.

### Async Steps

Mark with `async`:

```rust
async fn fetch_price(symbol: &str) -> f64 {
    let resp = reqwest::get(&format!("https://api.example.com/price/{}", symbol)).await?;
    resp.json().await
}
```

Pipeline executor supports async steps automatically (requires `tokio` runtime).

### Step Attributes

```rust
step("expensive") {
    heavy_computation(input)
} @parallel  // run in parallel with other @parallel steps (experimental)
```

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
```

If you change `42` to `43`, compilation fails:

```
error[E3002]: cannot prove constraint at compile time: x == 43
    --> src/lib.rs:12:5
     |
12  |         metric("x").eq(43)
     |         ^^^^^^^^^^^^^^^^^^
```

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
```

If constraint fails, pipeline returns `ConstraintFailure` result.

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
```

**Implicit metric naming:** The step identifier becomes the metric name automatically. Explicit override:

```rust
step("volatility") { compute_vol(input) } -> metric("risk")
```

(if step returns a value, it's bound to `"risk"` metric name)

---

## Input & Output Types

### Input

```rust
pipeline! {
    input: MyInputStruct,  // any Rust type
    ...
}
```

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
```

You can extract typed values:

```rust
let profit: f64 = result.values["profit"].as_f64().unwrap();
```

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
```

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
```

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
```

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
```

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
```

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
```

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
```

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
```

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
```

---

## Testing Strategies

### Unit Test a Step

```rust
#[test]
fn test_compute_fee() {
    assert_eq!(compute_fee(100.0), 0.1);
}
```

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
```

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
```

---

## Performance Tips

1. **Executor reuse:** Creating `Executor` allocates channels; reuse when possible
2. **Avoid unnecessary clones:** Pass `&T` to steps when possible (input is `&Input`)
3. **Use `@parallel` for independent steps:** Overheads low; benefits high for I/O-bound steps
4. **Profile:** `cargo flamegraph --bin your-binary` shows engine hot spots

---

## FAQ

**Q: Can I mix `.tp` files with Rust DSL?**  
A: Yes. Use `tupa_parser::parse_file("legacy.tp")?` to load and embed `.tp` pipelines alongside Rust-defined ones. But new code should be Rust-only.

**Q: How do I debug a pipeline?**  
A: Set `RUST_LOG=tupa_engine=debug` to see step-by-step execution logs. Use `tupa-audit` to hash ASTs.

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
