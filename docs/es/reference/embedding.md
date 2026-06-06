# Embedding de Tupã en Rust (Crate-First)

**Estado:** Forma canónica de usar Tupã (0.9.x).

Tupã se integra como biblioteca Rust. No hay runtime separado que instalar: agrega
los crates a tu `Cargo.toml` y escribe pipelines tipados con la macro `pipeline!`.

## Crates públicos (0.9.x)

| Crate | Propósito |
|---|---|
| `tupa-core` | Macro DSL, tipos (`Safe`, `Tensor`), trait `Pipeline` |
| `tupa-engine` | Executor, evaluación de constraints, runner |
| `tupa-plugin` | Carga dinámica de step functions |
| `tupa-pyffi` | Bindings de Python (PyO3) — alpha |

## Ejemplo

```toml
[dependencies]
tupa-core = "0.9"
tupa-engine = "0.9"
```

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Order { user_id: u64, amount_usd: f64, risk_score: f64 }

fn risk(o: &Order) -> f64 { o.risk_score }

pipeline! {
    name: OrderPolicy,
    input: Order,
    steps: [
        step("risk") { risk(input) }
    ],
    constraints: [
        metric("risk").le(0.9)
    ]
}

fn main() {
    let policy = OrderPolicy::new();
    let engine = Executor::new();
    let order = Order { user_id: 42, amount_usd: 5_000.0, risk_score: 0.3 };
    let result = engine.run(&policy, &order).expect("run failed");
    println!("constraints OK: {}", result.passed);
}
```

Para extender step functions en tiempo de ejecución, usa el sistema de plugins
(`tupa-plugin`). Para Python, usa `tupa-pyffi`. Las crates `.tp` de embedding
(`tupa-parser`, `tupa-typecheck`, `tupa-runtime`, `tupa-codegen`) fueron
eliminadas en 0.9.0.
