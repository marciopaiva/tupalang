# Config DSL (descontinuado)

> **Eliminado en 0.9.0.** La sintaxis `config { ... }` era una función del
> lenguaje `.tp` independiente, eliminado en 0.9.0. No existe una palabra clave
> `config` en el Rust DSL.

## Equivalente actual

Modele la configuración como una **struct de entrada tipada y anidada** que el
host completa (por ejemplo, desde JSON) y pasa al pipeline. Los pasos acceden a
los valores con acceso a campos de Rust normal:

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct ConfigTrading { capital_inicial: f64, riesgo_maximo: f64, posicion_maxima: i64 }
#[derive(Debug, Clone, Serialize)]
struct StrategyInput { capital: f64, config: ConfigTrading }
```

El host provee el objeto JSON correspondiente y el type checker de `rustc`
garantiza que cada campo se use con el tipo correcto. Vea
[features/trading_support.md](../features/trading_support.md) para un ejemplo
completo de `pipeline!` con configuración tipada y restricciones de riesgo.
