# Soporte para Bots de Trading en Tupã

Este documento detalla las funcionalidades implementadas en Tupã Runtime para la línea `0.8.1`, orientadas a soportar aplicaciones de trading algorítmico como `ViperTrade`.

## Resumen

El lenguaje y runtime de Tupã fueron ampliados para soportar operaciones financieras críticas, garantizando seguridad, resiliencia y auditabilidad.

## Funcionalidades Clave

### 1. Motor de Backtesting

La función `run_backtest` ofrece un entorno nativo de simulación para estrategias de trading.

- **Propósito**: Validar la lógica de estrategia contra datos históricos antes del despliegue en vivo.
- **Mecanismo**:
  - Itera sobre un dataset de velas/ticks históricos.
  - Ejecuta el pipeline para cada paso temporal.
  - Evalúa restricciones de riesgo (ej. `MaxDrawdown`, `PositionSize`).
  - Rastrea PnL de portafolio (Profit and Loss) con base en señales `BUY`/`SELL` y precios `close`.
- **Auditoría**: Cada operación y acción bloqueada queda registrada con una traza de auditoría estructurada.

### 2. Circuit Breaker

Patrón de resiliencia para evitar fallas en cascada durante volatilidad de mercado o caídas de API.

- **Configuración**:
  - `failure_threshold`: Número de errores consecutivos permitidos (ej. 3).
  - `reset_timeout`: Tiempo de espera antes de volver a probar la conexión (ej. 30s).
- **Comportamiento**:
  - **Closed**: Operación normal.
  - **Open**: Bloquea la ejecución inmediatamente al alcanzar el umbral.
  - **Half-Open**: Permite una única solicitud de prueba para verificar recuperación.

### 3. Integración con IA en Python (`tupa-pyffi`)

Integración transparente con modelos ML en Python (PyTorch/TensorFlow) para generación de señales.

- **Sintaxis**: Pasos definidos como `py:module.func` (ej. `py:viper_model.predict`).
- **Seguridad**: Inputs y outputs se validan contra esquemas estrictos (ej. shapes de tensores `[1, 60, 4]`).
- **Rendimiento**: Transferencia de datos zero-copy (cuando es posible) vía FFI.

### 4. Logging de Auditoría Estructurado

Logging listo para cumplimiento utilizando `tracing`.

- **Formato**: Logs estructurados en JSON.
- **Eventos**:
  - `pipeline_start` / `pipeline_complete`
  - `trade_executed` (con precio, tipo e índice)
  - `trade_blocked_by_risk` (cuando fallan restricciones)
  - `circuit_breaker_tripped`

### 5. Config tipada provista por el host vía input estructurado

Tupã ya soporta un patrón práctico de config binding para sistemas de estrategia en producción:

- declarar el input del pipeline como un record anidado
- pasar datos de mercado y config en el mismo objeto tipado
- usar field access común dentro de funciones de policy

Esto ya cubre muchos casos de estrategia, como:

- thresholds por símbolo
- overlays por modo/perfil
- parámetros de trailing
- thresholds de confirmación

Shape de ejemplo:

```text
input: {
  symbol: string,
  signal: { spread_pct: f64, trend_score: f64 },
  config: {
    entry: {
      max_spread_pct: f64,
      min_trend_score_long: f64
    }
  }
}
```

Ver:

- `examples/pipeline/config_driven_strategy.tp`
- `examples/pipeline/config_driven_strategy.json`

### 6. Política temporal declarativa vía estado provisto por el host

Tupã ya puede modelar un primer slice de política temporal sin mover el estado del host al runtime
del lenguaje:

- el host mantiene contadores y timers
- el pipeline recibe ese estado temporal como input estructurado
- built-ins expresan el shape del resultado para confirmación y cooldown

Built-ins actuales:

- `confirm(observed, consecutive_hits, required_hits, reason)`
- `cooldown(active, remaining_ticks, reason)`

Esto es útil para casos como:

- confirmación de señal tras `N` observaciones consecutivas
- cooldown después de stop loss
- persistencia de tesis dirigida por contadores mantenidos en el host

Shape de ejemplo:

```text
input: {
  temporal: {
    signal_confirmation: {
      observed: bool,
      consecutive_hits: i64,
      required_hits: i64
    },
    cooldown_guard: {
      active: bool,
      remaining_seconds: i64
    },
    thesis_confirmation: {
      observed: bool,
      consecutive_hits: i64,
      required_hits: i64
    }
  }
}
```

Ver:

- `examples/pipeline/temporal_policy.tp`
- `examples/pipeline/temporal_policy.json`

## Ejemplo de Uso

```rust
// Configurando el runtime para un bot de trading
let runtime = Runtime::new();
runtime.configure_circuit_breaker(3, Duration::from_secs(10));

// Ejecutando un backtest
let result = runtime.run_backtest(&plan, historical_data).await?;
println!("PnL final: {}", result["final_pnl"]);
```
