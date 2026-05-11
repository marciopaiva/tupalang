# Embedding Tupã in Rust (Crate-First)

**Status:** This is the canonical way to use Tupã in 2026+.

Tupã is designed to be embedded as a Rust library. There is **no separate runtime** to install. Add crates to your `Cargo.toml` and write typed pipelines using the `pipeline!` macro.

---

## Core Crates

| Crate | Purpose | Public API |
|---|---|---|
| `tupa-core` | DSL macros, types (`Safe`, `Tensor`), traits (`Pipeline`) | ✅ Stable |
| `tupa-engine` | Executor, constraint solver, runner | ✅ Beta |
| `tupa-runtime` | Low-level runtime primitives (internal) | ❌ Internal |
| `tupa-audit` | AST hashing, reproducibility | ✅ Stable |
| `tupa-plugin` | Dynamic step function loading | ✅ Stable |
| `tupa-fmt` | Formatter (legacy `.tp` files) | ✅ Stable |
| `tupa-lint` | Linter for policy code | ✅ Beta |

**Stable crates** can be used in production. **Beta** crates have minor breaking changes possible before 1.0.

---

## Quick Embedding Example

```rust
// Cargo.toml
[dependencies]
tupa-core = "0.2"
tupa-engine = "0.2"
```text

```rust
// src/lib.rs
use tupa_core::{pipeline, step, metric};

/// Your domain type
#[derive(Debug, Clone)]
struct Order {
    user_id: u64,
    amount_usd: f64,
    risk_score: f64,
}

/// A pure step function (no side effects except maybe logging)
fn risk_level(order: &Order) -> &str {
    if order.risk_score > 0.8 { "high" } else { "normal" }
}

/// The policy pipeline
pipeline! {
    name: OrderPolicy,
    input: Order,
    steps: [
        step("risk") { risk_level(input) },
        step("amount_ok") { input.amount_usd <= 10_000.0 }
    ],
    constraints: [
        metric("max_amount").le(10_000.0),
        metric("risk_score").le(0.9)
    ]
}
```text

```rust
// src/main.rs or inside your trading service
use tupa_engine::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let policy = my_crate::OrderPolicy::new();
    let mut executor = Executor::new();

    let order = Order { user_id: 42, amount_usd: 5_000.0, risk_score: 0.3 };
    let result = executor.run(policy, &order)?;

    println!("All constraints passed: {}", result.all_constraints_passed);
    Ok(())
}
```text

---

## Advanced: Custom Step Registration

For dynamically loaded step functions, use `tupa-plugin`:

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/custom_strategies.so")?;

// Later, inside a pipeline step:
// pm.call("strategy_name", json!(input))?
```text

See [tupa-plugin README](../../crates/tupa-plugin/README.md) for plugin development.

---

## Safety & Guarantees

### Compile-Time Guarantees

The `pipeline!` macro checks:

1. All step functions exist and have compatible signatures
2. Constraint expressions are well-typed and pure
3. Input type matches pipeline input
4. Output type compatibility with constraint metrics

Any violation produces a **Rust compiler error** with span info.

### Runtime Guarantees

- **Channels are ownership-based** — no data races
- **Deterministic step ordering** — steps execute in declared order (unless `@async` attribute used)
- **Constraint guards** — even compile-time-proven constraints are checked at startup in debug mode

---

## FFI (C & Python)

Phase 3 will deliver stable FFI bindings.

### C ABI (planned)

```c
// Exported function signature
typedef struct tupa_pipeline tupa_pipeline_t;

extern "C" {
    tupa_pipeline_t* tupa_pipeline_new(const char* name);
    void tupa_pipeline_add_step(tupa_pipeline_t*, const char* step_name, void (*fn)(void*));
    int tupa_pipeline_run(tupa_pipeline_t*, const void* input, void* output);
    void tupa_pipeline_free(tupa_pipeline_t*);
}
```text

### Python bindings (planned)

```python
import tupa

pipeline = tupa.Pipeline.from_rust_module("my_strategy")
result = pipeline.run(input_data)
```text

Track progress: [tupa-pyffi crate](../../crates/tupa-pyffi/).

---

## API Stability

- `tupa-core` 0.x → 1.0 (est. 2026-Q4) will have minor breaking changes in macro syntax. Once 1.0 released, API freeze.
- `tuba-engine` runtime behavior (scheduling, channel semantics) will be stable after 0.2.0.
- FFI ABI will be locked at 1.0.0.

See [versioning](../reference/versioning.md) for details.

---

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tupa_engine::Executor;

    #[test]
    fn policy_accepts_valid_input() {
        let policy = OrderPolicy::new();
        let engine = Executor::new();
        let order = Order { amount_usd: 1_000.0, .. };
        let res = engine.run(policy, &order).unwrap();
        assert!(res.all_constraints_passed);
    }
}
```text

CI integration: run `cargo test` as usual.

---

## Debugging

- Step outputs logged at `trace` level — enable with `RUST_LOG=tupa_engine=debug`
- Constraint failures include metric name, expected vs actual
- Audit trail: enable `tupa_audit::enable()` to get per-step hashes

---

## Performance Tips

1. **Reuse `Executor`** — it holds channel pools and scheduler state
2. **Mark pure steps as `#[tupa::pure]`** — enables compile-time optimizations
3. **Use tensors** for numeric arrays (SIMD-accelerated soon)
4. **Profile with `cargo flamegraph`** — engine is the hot path

Benchmarks coming in Phase 4.

---

## Next

- [Pipeline Guide](pipeline_guide.md) — full syntax, async steps, plugins
- [Type Semantics](type_semantics.md) — formal guarantees
- [Error Reference](common_errors.md) — diagnostic codes and fixes
