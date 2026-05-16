# Getting Started: Your First Tupã Pipeline in Rust

**Time:** ~5 minutes

**Goal:** Create a new Rust project, add Tupã crates, and run a typed policy pipeline.

---

## Prerequisites

- **Rust 1.83+** — install via [rustup.rs](https://rustup.rs)
- **cargo** (comes with Rust)

No other dependencies.

---

## Step 1: Create a New Rust Library

```bash
cargo new my-policy --lib
cd my-policy
```text

**Why `--lib`?** Tupã pipelines are libraries that can be embedded in any Rust application (CLI, server, trading system, etc.).

---

## Step 2: Add Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.9"      # DSL macros and types
tupa-engine = "0.9"    # Pipeline executor
```text

Check crates.io for latest version: [crates.io/crates/tupa-core](https://crates.io/crates/tupa-core)

---

## Step 3: Write a Pipeline

Replace `src/lib.rs` with:

```rust
use tupa_core::{pipeline, step, constraint, metric};

/// Domain type — a trading decision.
#[derive(Debug, Clone)]
struct Trade {
    symbol: String,
    side: Side,
    size_usd: f64,
    price: f64,
}

#[derive(Debug, Clone)]
enum Side { Buy, Sell }

/// Pure function: computes position size after trade.
fn compute_position_size(trade: &Trade) -> f64 {
    trade.size_usd * trade.price / 1_000_000.0  // in millions
}

/// Pure function: computes notional exposure.
fn exposure(trade: &Trade) -> f64 {
    trade.size_usd * trade.price
}

/// The policy pipeline.
/// 
/// Expands to a struct `MyRiskPolicy` implementing the `Pipeline` trait.
pipeline! {
    name: MyRiskPolicy,
    input: Trade,
    steps: [
        step("pos_size") { compute_position_size(input) },
        step("exposure") { exposure(input) }
    ],
    constraints: [
        /// Position size cannot exceed $10M
        metric("max_position").le(10.0),
        /// Exposure must be positive (no short here for simplicity)
        metric("exposure").gt(0.0)
    ]
}
```text

**What happened?** The `pipeline!` macro parsed the DSL at **compile time** and generated:

- A struct `MyRiskPolicy`
- Implementation of `Pipeline` trait
- Type-safe step wiring (checked by rustc)

---

## Step 4: Write an Executor

Create `src/main.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use my_policy::MyRiskPolicy;
    use tupa_engine::Executor;

    // Build the pipeline (constraints already proven at compile time)
    let policy = MyRiskPolicy::new();

    // Engine runs steps sequentially (or async if you use async steps)
    let mut engine = tupa_engine::Executor::new();

    // Example trade
    let trade = my_policy::Trade {
        symbol: "AAPL".into(),
        side: my_policy::Side::Buy,
        size_usd: 1_000_000.0,  # $1M trade
        price: 170.0,
    };

    // Execute
    let result = engine.run(policy, &trade)?;

    // Inspect
    println!("Pipeline result:");
    println!("  position_size = {:.2}M USD", result.values["pos_size"]);
    println!("  exposure = ${:.2}", result.values["exposure"]);
    println!("  all constraints passed: {}", result.all_constraints_passed);

    Ok(())
}
```text

Note: `src/main.rs` depends on the lib (`my-policy`). That's standard Rust layout.

---

## Step 5: Run It

```bash
cargo run --release
```text

Expected output:

```text
Pipeline result:
  position_size = 1.00M USD
  exposure = $170000000.00
  all constraints passed: true
```text

If you change the trade size to exceed $10M:

```rust
size_usd: 20_000_000.0,  # $20M
```text

You'll get a constraint violation at **runtime** (because size is dynamic):

```text
Pipeline result:
  position_size = 20.00M USD
  exposure = $3400000000.00
  constraint failed: max_position ≤ 10.0 (actual 20.0)
```text

But if you try to pass a string where a number is expected, **the code won't compile**:

```rust
// This fails at compile time — type error
let trade = Trade { size_usd: "big", ... };
//               ^^^^^^^^^^^^^ expected f64, found &str
```text

That's the core value: **type safety from Rust**.

---

## Next Steps

### 1. Try a constraint that *can* be proven at compile time

```rust
pipeline! {
    name: ConstantCheck,
    input: (),
    steps: [
        step("const") { 42 }  // constant known at compile time
    ],
    constraints: [
        metric("result").eq(42)   // ✅ proven at compile time
    ]
}
```text

**Result:** If you change `42` to `43`, compilation fails with:

```text
error[E3002]: cannot prove constraint at compile time: result == 43
```text

That's the `!nan`/`!inf` style constraint checking, but for any constant expression.

### 2. Explore the `Safe<T>` type

```rust
use tupa_core::Safe;

/// A number that cannot be NaN at compile time if we can prove it.
let x: Safe<f64, !nan> = Safe::new(3.14);  // OK

// let y: Safe<f64, !nan> = Safe::new(f64::NAN);  // ❌ Compile error
```text

### 3. Add Tensor support (requires `tupa-ml` feature, coming soon)

```rust
// Future: tensors with shape info
use tupa_core::Tensor;

let image: Tensor<f32, { shape: [28, 28], density: 1.0 }> = Tensor::zeros();
```text

### 4. Check out real example in ViperTrade

[ViperTrade strategies](https://github.com/marciopaiva/vipertrade) — production pipelines using Tupã.

---

## IDE Support

Because this is just Rust, **rust-analyzer works out of the box**:

- Hover over `metric("max_position")` to see its type
- Go to definition on `compute_position_size` to jump to function
- Inline type errors show up in editor
- Refactoring (rename, extract) works perfectly

No LSP installation needed.

---

## Testing Your Pipeline

Add unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::Executor;

    #[tokio::test]  // if using async steps
    async fn pipeline_accepts_valid_trade() {
        let policy = MyRiskPolicy::new();
        let engine = Executor::new();
        let trade = Trade { size_usd: 1_000_000.0, .. };

        let res = engine.run(policy, &trade).unwrap();
        assert!(res.all_constraints_passed);
    }

    #[test]
    fn pipeline_rejects_invalid_constraint() {
        let policy = MyRiskPolicy::new();
        let engine = Executor::new();
        let trade = Trade { size_usd: 20_000_000.0, .. };  # exceeds limit

        let res = engine.run(policy, &trade).unwrap();
        assert!(!res.all_constraints_passed);
    }
}
```text

Run:

```bash
cargo test
```text

---

## Engine Configuration

### Timeouts

Set a per-step timeout to prevent hanging steps:

```bash
TUPA_STEP_TIMEOUT=30s cargo tupa run
```

Or programmatically:

```rust
use tupa_engine::{Executor, ExecutorConfig};

let config = ExecutorConfig::default()
    .with_step_timeout(std::time::Duration::from_secs(30));
let executor = Executor::with_config(config);
```

### Metrics Export

Export step timings for profiling and observability:

```bash
cargo tupa run --metrics-output metrics.json
```

The generated `metrics.json` contains per-step start/end timestamps and execution state.

### Cancellation

Graceful shutdown is automatic when using `cargo tupa run` (Ctrl+C handled). Programmatic cancellation:

```rust
let executor = Executor::from_env()?;
let handle = executor.handle();
// In another thread or on signal:
handle.cancel();
```

---

## Formatting & Linting

```bash
# Format your code (and Tupã macro expansions are ignored safely)
cargo fmt

# Lint for common mistakes
cargo tupa lint
```

---

## Troubleshooting

| Symptom | Fix |
|---|---|
| `cannot find macro 'pipeline'` | Ensure `tupa-core = "0.9"` in `Cargo.toml` and `use tupa_core::pipeline;` |
| "constraint cannot be proven" | Adjust metric value or expression to be constant-foldable |
| Engine hangs | Check for infinite loops in step functions or missing `await` on async steps |

---

## What's Next?

- Read the [Pipeline Guide](pipeline_guide.md) for advanced patterns (async steps, multi-pipeline)
- Browse [API Reference](https://docs.rs/tupa-core) for full trait docs
- See [TRANSITION.md](TRANSITION.md) if coming from `.tp` files
- Join [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)

**You're ready to build deterministic, type-safe policy systems in Rust.**
