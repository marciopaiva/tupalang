# Soporte para Bots de Trading en Tupã

Este documento detalla las funcionalidades implementadas en Tupã Runtime (v0.8.1-dev) para soportar aplicaciones de trading algorítmico como `ViperTrade`.

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

## Ejemplo de Uso

```rust
// Configurando el runtime para un bot de trading
let runtime = Runtime::new();
runtime.configure_circuit_breaker(3, Duration::from_secs(10));

// Ejecutando un backtest
let result = runtime.run_backtest(&plan, historical_data).await?;
println!("PnL final: {}", result["final_pnl"]);
```
