# Embedding de Tupã en Rust (Crate-First)

**Status:** Forma canônica de usar o Tupã (0.9.x).

O Tupã se integra como biblioteca Rust. Não há runtime separado para instalar:
adicione os crates ao seu `Cargo.toml` e escreva pipelines tipados com a macro
`pipeline!`.

## Crates públicos (0.9.x)

| Crate | Propósito |
|---|---|
| `tupa-core` | Macro DSL, tipos (`Safe`, `Tensor`), trait `Pipeline` |
| `tupa-engine` | Executor, avaliação de constraints, runner |
| `tupa-plugin` | Carga dinâmica de step functions |
| `tupa-pyffi` | Bindings de Python (PyO3) — alpha |

## Exemplo

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
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

Para estender step functions em tempo de execução, use o sistema de plugins
(`tupa-plugin`). Para Python, use `tupa-pyffi`. Os crates `.tp` de embedding
(`tupa-parser`, `tupa-typecheck`, `tupa-runtime`, `tupa-codegen`) foram removidos
na 0.9.0.
