# Transition Guide: From Standalone Tupã to Rust Crate DSL

**Target audience:** Existing Tupã users (`.tp` files + CLI) migrating to the new crate-based Rust DSL.

**Timeline:** O compilador standalone `.tp` foi **removido** do workspace Tupã na versão 0.9.0. Pipelines `.tp` legacy devem ser migrados para Rust DSL. Não haverá mais atualizações da toolchain `.tp`.

---

## Why This Change?

The standalone `.tp` language required:

- Installing a custom toolchain (`tupa` binary)
- Learning a new syntax (similar to Rust but different)
- Debugging across language boundaries
- Limited IDE support (waiting for LSP)

**The Rust crate approach gives you:**

- `cargo add tupa-core` — no new toolchain
- Rust syntax + macros — Rust IDE support immediately
- Full type checking by rustc before macro expansion
- Simpler mental model (just Rust traits and types)

---

## Quick Side-by-Side

### Legacy (.tp file)

```tupa
// strategy.tp
enum Signal { Buy, Sell, Hold }

fn score(s: Signal): i64 {
  match s {
    Buy => 100,
    Sell => -100,
    Hold => 0
  }
}

pipeline MyStrategy {
  input: Signal,
  steps: [
    step("score") { score(input) }
  ],
  constraints: [
    metric("sharpe").ge(1.5)
  ]
}
```text

```bash
# Old workflow
tupa check strategy.tp
tupa codegen --format json strategy.tp
tupa run --pipeline MyStrategy --input signal.json strategy.tp
```text

---

### New (Rust DSL with `tupa-core`)

```rust
// src/strategy.rs
use tupa_core::{pipeline, step, constraint, metric, Safe};

#[derive(Debug, Clone, PartialEq)]
enum Signal { Buy, Sell, Hold }

fn score(s: &Signal) -> i64 {
    match s {
        Signal::Buy => 100,
        Signal::Sell => -100,
        Signal::Hold => 0,
    }
}

// Declarative macro — expands to a struct implementing Pipeline trait
pipeline! {
    name: MyStrategy,
    input: Signal,
    steps: [
        step("score") { score(input) }
    ],
    constraints: [
        metric("sharpe").ge(1.5)
    ]
}
```text

```rust
// src/main.rs
use tupa_engine::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plan = my_strategy::MyStrategy::new();
    let engine = Executor::new();
    let signal = Signal::Buy;
    let result = engine.run(plan, &signal)?;
    println!("Result: {:?}", result);
    Ok(())
}
```text

```bash
# New workflow
cargo tupa check          # validates pipeline DSL
cargo tupa fmt            # formats Rust-DSL pipeline code
cargo tupa lint           # lints Rust code for Tupã patterns
cargo build --release
cargo run --release
```text

---

## Mapping Concepts

| Concept | Legacy (.tp) | New (Rust) |
|---|---|---|
| Pipeline definition | `pipeline Name { ... }` | `pipeline! { name: Name, ... }` |
| Step function | `step("id") { expr }` | `step("id") { expr }` — same inside macro |
| Constraints | `metric("x").le(1)` | `metric("x").le(1)` — identical inside macro |
| Input type | `input: Type` | `input: Type` — identical |
| Safe type | `Safe<T, !nan>` | `Safe<T, !nan>` — now a Rust type alias |
| Tensor type | `Tensor<f32, shape=[28,28]>` | `Tensor<f32, shape=[28,28]>` — Rust const generics |
| ∇ (gradient) | `∇f(x)` | `grad!(f)(x)` — macro generates backward pass |

**Key insight:** The DSL syntax inside `pipeline!{}` is nearly identical to `.tp`. The difference is that it's now **verified by rustc** before macro expansion.

---

## Migration Checklist

### Step 1: Add Dependencies

```toml
# Cargo.toml
[dependencies]
tupa-core = "0.9"
tupa-engine = "0.9"
```

### Step 2: Convert enum/type definitions

`.tp`:

```tupa
enum MarketEvent {
  Tick(symbol: string, price: f64),
  Bar(symbol: string, open: f64, close: f64)
}
```text

Rust:

```rust
#[derive(Debug, Clone)]
enum MarketEvent {
    Tick { symbol: String, price: f64 },
    Bar { symbol: String, open: f64, close: f64 },
}
```text

### Step 3: Port pure functions

`.tp`:

```tupa
fn sma(prices: [f64], window: i64): f64 {
  let sum = prices.slice(-window).sum();
  return sum / window as f64;
}
```text

Rust:

```rust
fn sma(prices: &[f64], window: usize) -> f64 {
    let sum: f64 = prices.iter().rev().take(window).sum();
    sum / window as f64
}
```text

### Step 4: Wrap in `pipeline!` macro

```rust
pipeline! {
    name: SMAPipeline,
    input: MarketEvent,
    steps: [
        step("sma") { sma(input.prices, 20) }
    ],
    constraints: [
        metric("max_leverage").le(2.0)
    ]
}
```text

### Step 5: Remove `.tp` file

Delete `strategy.tp`. All logic now lives in `.rs` files.

### Step 6: Update build/CI

```yaml
# .github/workflows/ci.yml (old)
- run: tupa check strategy.tp

# (new)
- run: cargo tupa check  # checks embedded DSL via proc-macro
- run: cargo test         # unit tests for pipeline logic
```text

---

## Automatic Migration Tool

There is no automatic conversion tool. Migrate manually by moving each `.tp`
construct into a `pipeline!` block in a `.rs` file, using the mapping table above.

---

## Differences & Gotchas

### 1. Pattern Matching

`.tp` supports tuple patterns and `|` or-patterns directly. Rust DSL uses Rust's pattern syntax, which is more featureful but slightly different.

```tupa
match x {
  (1, _) | (2, _) => print("one or two"),
  _ => print("other")
}
```text

In Rust DSL:

```rust
match x {
    (1, _) | (2, _) => println!("one or two"),
    _ => println!("other"),
}
```text

Works the same — good.

### 2. Option/Result

`.tp` has `Option<T>`/`Result<T,E>` built-in. Rust DSL uses Rust's `Option`/`Result` from std.

```tupa
fn divide(a: f64, b: f64): Result<f64, string> {
  if b == 0.0 { Err("div by zero") } else { Ok(a/b) }
}
```text

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 { Err("div by zero".into()) } else { Ok(a / b) }
}
```text

Same concept, Rust standard types.

### 3. Tensors

`.tp` special syntax: `Tensor<f32, shape=[28,28], density=0.1>`

Rust DSL:

```rust
use tupa_types::Tensor;

type Image = Tensor<f32, { shape: [28, 28], density: 0.1 }>;
// or via const generics (exact syntax TBD in 0.1)
```text

Still being designed — expect 0.1 design iteration.

### 4. `∇` Gradient

`.tp`: `let g = ∇square(3.0)` automatic differentiation.

Rust DSL:

```rust
use tupa_core::grad;

grad!(|x| x * x)(3.0);  // returns (6.0,)
// or
let squared = |x: f64| x * x;
let grad_fn = grad!(squared);
grad_fn(3.0);
```text

Gradient operator becomes a procedural macro `grad!` that generates backward pass at compile time.

---

## Testing

Old:

```bash
tupa check strategy.tp
```text

New:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::Executor;

    #[test]
    fn pipeline_runs() {
        let plan = MyStrategy::new();
        let engine = Executor::new();
        let input = MarketEvent::Tick { ... };
        let out = engine.run(plan, &input).unwrap();
        assert!(out.score > 0);
    }
}
```text

```bash
cargo test          # runs unit tests
cargo tupa check    # validates DSL macros (compile-time)
```text

---

## IDE Support

**Standalone `.tp`:** Waiting for LSP (Phase 1). Basic syntax highlighting only.

**Rust DSL:** Full rust-analyzer:

- Autocomplete for pipeline steps, constraints
- Go to definition on `step("score")` → `fn score()`
- Inline type errors from rustc
- Refactoring works (rename function, extract variable)

**→ Immediate productivity boost.**

---

## Performance Notes

Standalone `.tp` compiled to LLVM IR → native binary (theoretically fastest).

Rust DSL currently interpreted by `tupa-engine`. Performance is **sufficient for strategy policy** (not HFT). Future optimization path:

1. Cranelift JIT for hot pipelines
2. LLVM codegen opt-in (`#[tupa(backend="llvm")]`)
3. Mature engine + specialization will close gap

Benchmark target: <1μs per step decision.

---

**Status da migração:** A toolchain `.tp` foi removida na v0.9.0. Todos usuários devem migrar para Rust DSL. Não haverá mais atualizações do `.tp`.

---

## Precisa de Ajuda?

- Open issue: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- Migration examples: `examples/migration/` directory
- Discord: `#migration` channel (coming soon)

---

## Summary

You're trading:

- ✗ One syntax to learn
- ✗ Separate toolchain install

For:
- Rust toolchain everywhere
- IDE support day one
- Safer types (Rust compiler guarantees)
- Faster path to production (v1.0 in 6–12 months)

**Bottom line:** Porting a `.tp` file to Rust DSL takes 30–60 minutes per file for experienced Rustaceans. The resulting code is more maintainable and debuggable.

Start new projects with Rust DSL today. Legacy projects can coexist during transition.
