# Soporte para Trading en Tupã

Este documento describe cómo la arquitectura actual basada en crates de Tupã
(0.9.x) soporta casos de uso de políticas de trading algorítmico como
[ViperTrade](https://github.com/marciopaiva/vipertrade).

> **Historia:** Las versiones 0.8.x incluían un runtime `.tp` independiente con
> un motor de backtesting integrado, circuit breaker, hot reload y registro de
> esquemas (los crates `tupa-runtime` / `tupa-codegen`). Ese toolchain
> independiente fue eliminado en 0.9.0. Los patrones siguientes reflejan lo que
> los crates Rust-DSL actuales (`tupa-core`, `tupa-engine`, `tupa-pyffi`,
> `tupa-plugin`) realmente proveen; las funciones eliminadas se indican
> explícitamente en
> [Funciones del runtime 0.8.2 descontinuadas](#funciones-del-runtime-082-descontinuadas).

## Modelar una estrategia como un pipeline tipado

Expresa una política de trading como un `pipeline!` sobre una entrada tipada. Los
datos de mercado y la configuración viajan juntos en una sola estructura de
entrada anidada, y los pasos de política usan acceso a campos de Rust normal.

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Entry { max_spread_pct: f64, min_trend_score_long: f64 }
#[derive(Debug, Clone, Serialize)]
struct Config { entry: Entry }
#[derive(Debug, Clone, Serialize)]
struct Signal { spread_pct: f64, trend_score: f64 }
#[derive(Debug, Clone, Serialize)]
struct StrategyInput { symbol: String, signal: Signal, config: Config }

fn long_ok(i: &StrategyInput) -> f64 {
    let within_spread = i.signal.spread_pct <= i.config.entry.max_spread_pct;
    let trend_ok = i.signal.trend_score >= i.config.entry.min_trend_score_long;
    if within_spread && trend_ok { 1.0 } else { 0.0 }
}

pipeline! {
    name: EntryPolicy,
    input: StrategyInput,
    steps: [
        step("long_ok") { long_ok(input) }
    ],
    constraints: [
        metric("long_ok").ge(0.0)
    ]
}

fn main() {
    let plan = EntryPolicy::new();
    let engine = Executor::new();
    let input = StrategyInput {
        symbol: "BTCUSDT".into(),
        signal: Signal { spread_pct: 0.04, trend_score: 0.8 },
        config: Config { entry: Entry { max_spread_pct: 0.10, min_trend_score_long: 0.5 } },
    };
    let result = engine.run(&plan, &input).expect("run failed");
    println!("passed={} long_ok={}", result.passed, result.values["long_ok"]);
}
```

Este patrón de configuración tipada cubre muchos casos de estrategia —umbrales
por símbolo, overlays de modo/perfil, parámetros de trailing, umbrales de
confirmación— todos como campos de la entrada.

## Restricciones de riesgo

Los límites de riesgo se expresan como `constraints` sobre métricas producidas
por los pasos. El executor las evalúa e informa el resultado en
`PipelineResult::passed`. Por ejemplo, limitar una métrica calculada de tamaño
de posición o drawdown:

```rust
constraints: [
    metric("position_size").le(1_000_000.0),
    metric("max_drawdown_bps").le(1500.0)
]
```

## Política temporal vía estado provisto por el host

Tupã mantiene el estado del host (contadores, temporizadores) fuera del
lenguaje: el host los mantiene y pasa el estado temporal actual como parte de la
entrada tipada. Los pasos expresan decisiones de confirmación / cooldown con
lógica Rust normal.

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Confirmation { observed: bool, consecutive_hits: i64, required_hits: i64 }
#[derive(Debug, Clone, Serialize)]
struct Cooldown { active: bool, remaining_seconds: i64 }
#[derive(Debug, Clone, Serialize)]
struct Temporal { signal_confirmation: Confirmation, cooldown_guard: Cooldown }
```

Cubren la confirmación de señal tras *N* observaciones consecutivas, gates de
cooldown de stop-loss y umbrales de persistencia de tesis impulsados por
contadores mantenidos por el host. (Los built-ins 0.8.x `confirm(...)` /
`cooldown(...)` eran parte del runtime `.tp` eliminado; exprese la misma lógica
directamente en funciones de paso Rust.)

## Integración con Python / modelos de IA (`tupa-pyffi`)

Llama a modelos de Python (PyTorch / TensorFlow / NumPy) desde un paso vía
`tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

fn predict(input: &StrategyInput) -> Result<serde_json::Value, String> {
    call_python_function("viper_model", "predict", json!({ "symbol": input.symbol }))
}
```

> La antigua sintaxis de paso `py:module.func` era una función de `.tp`; en el
> Rust DSL se llama a funciones de `tupa-pyffi` directamente desde el cuerpo de un
> paso.

## Extensión dinámica vía plugins (`tupa-plugin`)

Las funciones de paso se pueden cargar desde bibliotecas compartidas en tiempo de
ejecución usando `tupa-plugin` (`PluginManager`), permitiendo extensión sin
recompilar el host. Vea el README del crate `tupa-plugin` para el ABI de plugins
y un ejemplo completo.

## Funciones del runtime 0.8.2 descontinuadas

Las siguientes eran parte del runtime `.tp` independiente eliminado en 0.9.0 y
**no** están disponibles en los crates actuales:

- Motor de backtesting integrado (`run_backtest`)
- Circuit breaker (`configure_circuit_breaker`)
- Hot reload (`Runtime::watch_and_reload`)
- Registro de esquemas / migraciones (`tupa-codegen`)
- Built-ins `tupa::*` y la sintaxis de paso `py:`

Para observabilidad de ejecución hoy, use las métricas por paso del executor
(`PipelineResult::metrics`) más el logging propio de su aplicación.
