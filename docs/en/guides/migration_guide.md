# Migration Guide: From `.tp` to Rust-DSL

**Status:** Legacy `.tp` toolchain **removed** in v0.9.0. Active development is Rust-DSL only.

The standalone `.tp` language and its compiler (`tupa-cli`, `tupa-parser`, `tupa-typecheck`, etc.) were permanently removed from the workspace. All new development uses the `pipeline!` macro in Rust source files.

This guide helps you migrate existing `.tp` pipelines to the Rust DSL.

---

## Why Migrate?

- ✅ No separate toolchain — use `cargo` and `rustc` directly
- ✅ Full IDE support (rust-analyzer works out of the box)
- ✅ Better error messages (rustc diagnostics with spans)
- ✅ Access to Rust ecosystem (crates, macros, traits)
- ✅ Faster iteration — no language boundary debugging
- ✅ Production-ready — ViperTrade uses Rust DSL exclusively

---

## Migration Checklist

For each `.tp` file in your project:

- [ ] Create a `.rs` module with `pipeline!` macro
- [ ] Convert step functions to Rust functions
- [ ] Convert pipeline definition to Rust DSL syntax
- [ ] Update `Cargo.toml` with `tupa-core` and `tupa-engine`
- [ ] Remove the `.tp` file
- [ ] Run `cargo tupa check` to validate
- [ ] Run `cargo tupa run` to test execution

---

## Step-by-Step Migration

### 1. Identify Legacy Files

```bash
find . -name "*.tp"
```

Example output:

```text
strategies/risk_limits.tp
strategies/position_sizing.tp
```

### 2. Create Rust Module Skeleton

For `strategies/risk_limits.tp`, create `strategies/risk_limits.rs`:

```rust
use tupa_core::{pipeline, step, constraint, metric};

pipeline! {
    name: RiskLimits,           // same as .tp pipeline name
    input: Trade,               // replace with your input type
    steps: [
        // TODO: convert each step body
    ],
    constraints: [
        // TODO: convert each constraint
    ]
}
```

### 3. Convert Step Functions

Copy the body of each step function from `.tp` to Rust. Adjust syntax:

| `.tp` Syntax | Rust DSL Equivalent |
|--------------|---------------------|
| `fn score(s: Signal): i64` | `fn score(s: &Signal) -> i64` |
| `fn validate(t: Trade): bool` | `fn validate(t: &Trade) -> bool` |
| `let x = 42` | same |
| `match s { Buy => 100 }` | same (Rust pattern syntax) |
| `if cond { a } else { b }` | same |

**Important changes:**

- Return type annotation: `: T` → `-> T`
- Parameters are passed by reference (`&T`) to avoid clones (input is `&Input`)
- No implicit returns — last expression is returned (same as Rust)

**Example conversion:**

```tupa
// .tp
fn compute_risk(trade: Trade): f64 {
    trade.size * trade.price / 1_000_000.0
}
```

↓

```rust
// .rs
fn compute_risk(trade: &Trade) -> f64 {
    trade.size * trade.price / 1_000_000.0
}
```

### 4. Convert Pipeline Definition

The `pipeline!` macro uses almost identical syntax to `.tp`:

| Construct | `.tp` | Rust DSL |
|-----------|-------|----------|
| Pipeline start | `pipeline MyPolicy {` | `pipeline! { name: MyPolicy,` |
| Input type | `input: Trade` | `input: Trade,` (identical) |
| Step block | `step("name") { expr }` | `step("name") { expr }` (identical) |
| Constraint | `metric("x").ge(10)` | `metric("x").ge(10)` (identical) |
| Pipeline end | `}` | `}` (comma after last element optional) |

**Full example:**

`.tp`:

```tupa
pipeline PreTradeCheck {
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) },
        step("limit") { input.size <= 1_000_000.0 }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0),
        metric("max_leverage").le(2.0)
    ]
}
```

Rust DSL:

```rust
pipeline! {
    name: PreTradeCheck,
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) },
        step("limit") { input.size <= 1_000_000.0 }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0),
        metric("max_leverage").le(2.0)
    ]
}
```

### 5. Update Cargo.toml

Add dependencies:

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
```

If you used plugins in `.tp`, add:

```toml
tupa-plugin = "0.10"
```

### 6. Create Main Binary (if not already)

Your package needs a binary that runs the pipeline:

```rust
// src/main.rs
use your_crate::{YourPipeline, Trade};  // adjust imports
use tupa_engine::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = YourPipeline::new();
    let engine = Executor::new();

    // Build input (from JSON, config, etc.)
    let input = Trade {
        symbol: "AAPL".into(),
        size: 1_000_000.0,
        price: 170.0,
    };

    let result = engine.run(pipeline, &input)?;
    println!("All constraints passed: {}", result.passed);
    Ok(())
}
```

### 7. Validate

```bash
# Type-check the pipeline macro
cargo tupa check

# Run the pipeline
cargo tupa run

# Run tests (if any)
cargo test
```

If `cargo tupa check` passes, the pipeline is syntactically and type-ally correct.

### 8. Remove `.tp` File

Once validated, delete the legacy file:

```bash
rm strategies/risk_limits.tp
```

---

## Syntax Differences Summary

| Feature | `.tp` | Rust DSL | Notes |
|---------|-------|----------|-------|
| File extension | `.tp` | `.rs` | Standard Rust source |
| Function signature | `fn name(args): ReturnType` | `fn name(args) -> ReturnType` | Rust uses `->` |
| Step expression | `{ expr }` | `{ expr }` | identical |
| Constraints block | `constraints: [ ... ]` | `constraints: [ ... ]` | identical |
| Metric access | `metric("name")` | `metric("name")` | identical |
| Type names | built-in (`i64`, `f64`) | Rust standard types (`i64`, `f64`) | identical |
| Tuples | `(i32, i64)` | `(i32, i64)` | identical |
| Enums | `enum Side { Buy, Sell }` | `enum Side { Buy, Sell }` | identical |
| Structs | `struct Trade { ... }` | `struct Trade { ... }` | identical |
| Pattern matching | `match x { ... }` | `match x { ... }` | identical |
| Comments | `//` or `/* */` | `//` or `/* */` | identical |

**No semantic differences** — the DSL inside `pipeline!` is designed to be nearly identical to `.tp`. The main change is that it's embedded in Rust and type-checked by `rustc` instead of a separate type checker.

---

## Common Migration Issues

### Issue: "cannot find macro `pipeline`"

**Cause:** Missing `use tupa_core::pipeline;` or `tupa-core` not in `Cargo.toml`.

**Fix:**

```toml
[dependencies]
tupa-core = "0.10"
```

```rust
use tupa_core::pipeline;
```

---

### Issue: Step function not found in scope

**Cause:** Step function defined after `pipeline!` macro or not `pub` if in another module.

**Fix:** Ensure step functions are declared `pub` if cross-module, or reorder definitions (functions must be in scope before macro invocation).

---

### Issue: Constraint cannot be proven at compile time

**Cause:** In `.tp` some constraints were proven by the old type checker; Rust DSL proving is more limited.

**Fix:** This is a **warning**, not an error. The constraint will be checked at runtime. If you need compile-time proof, simplify the expression to constant folding.

---

### Issue: Missing `.tp` standard library functions

**Cause:** `.tp` had built-in functions like `abs`, `max`, `min` that may not exist in Rust DSL.

**Fix:** Use Rust standard library equivalents (`f64::abs`, `f64::max`, etc.) or define your own helper functions.

---

### Issue: Plugin step functions (Python) no longer work

**Cause:** `.tp` plugin system was different.

**Fix:** Rewrite plugins as Rust plugins (`tupa-plugin` crate) or use `tupa-pyffi` for Python integration. See [Plugin Tutorial](../tutorials/plugin-rust.md).

---

## Validation Strategy

After migrating a pipeline:

1. **Compile-time check:**

   ```bash
   cargo tupa check
   ```

   Should emit no errors.

2. **Unit tests:** Test individual step functions with `#[test]`.

3. **Integration test:** Run full pipeline with representative input:

   ```rust
   #[tokio::test]
   async fn test_migrated_pipeline() {
       let pipeline = YourPipeline::new();
       let engine = Executor::new();
       let input = build_test_input();
       let result = engine.run(pipeline, &input).unwrap();
       assert!(result.passed);
   }
   ```

4. **Compare outputs:** If you have the old `.tp` pipeline output saved, compare results to ensure semantic equivalence.

---

## Upgrading from 0.10.0 to 0.11.0

### What changed

The `pipeline!` macro now passes `ctx: &StepContext` to every step body. If you implement `ParallelPipeline` **manually** (without the macro), you must update one method signature:

```rust
// 0.10.0
fn check_constraints(
    values: &HashMap<String, Value>,
) -> (bool, Vec<ConstraintFailure>);

// 0.11.0
fn check_constraints(
    values: &HashMap<String, Value>,
    input: &Self::Input,
) -> (bool, Vec<ConstraintFailure>);
```

**If you use `pipeline!`:** no changes needed. The macro regenerates this method automatically.

### New features available immediately

Update `Cargo.toml`:

```toml
tupa-core   = "0.11"
tupa-engine = "0.11"
```

Then use in step bodies:

```rust
// Read prior step output
let prev = ctx.get_f64("prev_step").unwrap_or(0.0);

// Computed constraint threshold
metric("score").le(input.config.max_score)

// Fail-fast
metric("equity").ge(0.0).fail_fast()

// Typed result accessors
let score = result.get_f64("score");
let decision = result.get_as::<MyDecision>("decision");
```

---

## Need Help?

- Open an issue: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- See [Pipeline Guide](../guides/pipeline_guide.md) for advanced patterns
- Browse examples in `crates/tupa-engine/examples/`

---

## Migration Complete

Once all legacy `.tp` files are converted:

- Remove any remaining references to `tupa-cli` or `.tp` in your build scripts
- Update documentation to reflect Rust-DSL usage
- Consider contributing your migration story to the project!
