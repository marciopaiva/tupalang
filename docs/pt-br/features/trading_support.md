# Suporte a Trading no Tupã

Este documento descreve como a arquitetura atual baseada em crates do Tupã
(0.9.x) suporta casos de uso de políticas de trading algorítmico como o
[ViperTrade](https://github.com/marciopaiva/vipertrade).

> **Histórico:** As versões 0.8.x incluíam um runtime `.tp` standalone com motor
> de backtesting embutido, circuit breaker, hot reload e registro de schemas (os
> crates `tupa-runtime` / `tupa-codegen`). Esse toolchain standalone foi removido
> na 0.9.0. Os padrões abaixo refletem o que os crates Rust-DSL atuais
> (`tupa-core`, `tupa-engine`, `tupa-pyffi`, `tupa-plugin`) de fato fornecem; as
> funcionalidades removidas estão indicadas explicitamente em
> [Funcionalidades do runtime 0.8.2 descontinuadas](#funcionalidades-do-runtime-082-descontinuadas).

## Modelando uma estratégia como um pipeline tipado

Expresse uma política de trading como um `pipeline!` sobre uma entrada tipada. Os
dados de mercado e a configuração viajam juntos em uma única struct de entrada
aninhada, e os passos de política usam acesso a campos do Rust comum.

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

Esse padrão de configuração tipada cobre muitos casos de estratégia — limiares
por símbolo, overlays de modo/perfil, parâmetros de trailing, limiares de
confirmação — todos como campos da entrada.

## Restrições de risco

Limites de risco são expressos como `constraints` sobre métricas produzidas pelos
passos. O executor as avalia e reporta o resultado em `PipelineResult::passed`.
Por exemplo, limitar uma métrica calculada de tamanho de posição ou drawdown:

```rust
constraints: [
    metric("position_size").le(1_000_000.0),
    metric("max_drawdown_bps").le(1500.0)
]
```

## Política temporal via estado provido pelo host

O Tupã mantém o estado do host (contadores, timers) fora da linguagem: o host os
mantém e passa o estado temporal atual como parte da entrada tipada. Os passos
então expressam decisões de confirmação / cooldown com lógica Rust comum.

```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Confirmation { observed: bool, consecutive_hits: i64, required_hits: i64 }
#[derive(Debug, Clone, Serialize)]
struct Cooldown { active: bool, remaining_seconds: i64 }
#[derive(Debug, Clone, Serialize)]
struct Temporal { signal_confirmation: Confirmation, cooldown_guard: Cooldown }
```

Cobrem confirmação de sinal após *N* observações consecutivas, gates de cooldown
de stop-loss e limiares de persistência de tese guiados por contadores mantidos
pelo host. (Os built-ins 0.8.x `confirm(...)` / `cooldown(...)` faziam parte do
runtime `.tp` removido; expresse a mesma lógica diretamente em funções de passo
Rust.)

## Integração com Python / modelos de IA (`tupa-pyffi`)

Chame modelos Python (PyTorch / TensorFlow / NumPy) a partir de um passo via
`tupa-pyffi`:

```rust
use tupa_pyffi::call_python_function;
use serde_json::json;

fn predict(input: &StrategyInput) -> Result<serde_json::Value, String> {
    call_python_function("viper_model", "predict", json!({ "symbol": input.symbol }))
}
```

> A antiga sintaxe de passo `py:module.func` era uma funcionalidade do `.tp`; no
> Rust DSL você chama funções do `tupa-pyffi` diretamente do corpo de um passo.

## Extensão dinâmica via plugins (`tupa-plugin`)

Funções de passo podem ser carregadas de bibliotecas compartilhadas em tempo de
execução usando `tupa-plugin` (`PluginManager`), permitindo extensão sem
recompilar o host. Veja o README do crate `tupa-plugin` para o ABI de plugins e
um exemplo completo.

## Funcionalidades do runtime 0.8.2 descontinuadas

As seguintes faziam parte do runtime `.tp` standalone removido na 0.9.0 e **não**
estão disponíveis nos crates atuais:

- Motor de backtesting embutido (`run_backtest`)
- Circuit breaker (`configure_circuit_breaker`)
- Hot reload (`Runtime::watch_and_reload`)
- Registro de schemas / migrações (`tupa-codegen`)
- Built-ins `tupa::*` e a sintaxe de passo `py:`

Para observabilidade de execução hoje, use as métricas por passo do executor
(`PipelineResult::metrics`) mais o logging próprio da sua aplicação.
