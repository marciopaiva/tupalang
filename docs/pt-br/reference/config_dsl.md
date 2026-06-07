# Config DSL (descontinuado)

> **Removido na 0.9.0.** A sintaxe `config { ... }` era uma funcionalidade da
> linguagem `.tp` standalone, removida na 0.9.0. Não existe palavra-chave
> `config` no Rust DSL.

## Equivalente atual

Modele a configuração como uma **struct de entrada tipada e aninhada** que o host
preenche (por exemplo, a partir de JSON) e passa ao pipeline. Os passos acessam
os valores com acesso a campos do Rust comum:

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct ConfigTrading { capital_inicial: f64, riesgo_maximo: f64, posicion_maxima: i64 }
#[derive(Debug, Clone, Serialize)]
struct StrategyInput { capital: f64, config: ConfigTrading }
```

O host fornece o objeto JSON correspondente e o type checker do `rustc` garante
que cada campo seja usado com o tipo correto. Veja
[features/trading_support.md](../features/trading_support.md) para um exemplo
completo de `pipeline!` com configuração tipada e restrições de risco.
