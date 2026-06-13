# Guía de Pipeline

## Objetivo

Ejecutar un pipeline Tupã de extremo a extremo: generar un ExecutionPlan y ejecutar con entrada JSON.

## Pasos

- Escribe el pipeline con la macro `pipeline!` (ver [getting_started.md](getting_started.md)).
- Verifica los tipos: `cargo tupa check`.
- Ejecuta con entrada JSON: `cargo tupa run --input data.json`.
- Ejecución paralela: `cargo tupa run --parallel --input data.json`.
- Persistir métricas por paso: `cargo tupa run --input data.json --metrics-output metrics.json`.

## Estructura del ExecutionPlan

- name, version, seed (opcional), input_schema
- steps: name, function_ref, effects
- constraints: metric, comparator, threshold
- metrics: valores literales capturados del bloque de validación
- metric_plans: { name, function_ref, args } para calcular métricas en runtime

## Notas

- Formato de function_ref: `<file>::step_<name>`.
- Los efectos (random/time) son identificados por el typechecker.
- El runtime evalúa restricciones y emite un reporte JSON con métricas/restricciones.

---

## StepContext — Lectura de Salidas de Pasos Anteriores

Cada cuerpo de paso tiene acceso a `ctx: &StepContext`, que contiene las salidas de los pasos anteriores. Esto permite que los pasos posteriores lean resultados previos sin volver a ejecutarlos.

```rust
pipeline! {
    name: ScoringPipeline,
    input: MarketData,
    steps: [
        step("base_score") { compute_base(input) },
        step("adjusted_score") {
            // leer la salida de "base_score" desde el contexto
            let base = ctx.get_f64("base_score").unwrap_or(0.0);
            base * input.volatility_multiplier
        } requires ["base_score"],
    ],
    constraints: []
}
```

### API de StepContext

| Método | Retorno | Descripción |
|--------|---------|-------------|
| `ctx.get("name")` | `Option<&Value>` | Valor JSON crudo |
| `ctx.get_f64("name")` | `Option<f64>` | Parsear como f64 |
| `ctx.get_bool("name")` | `Option<bool>` | Parsear como bool |
| `ctx.get_str("name")` | `Option<&str>` | Parsear como &str |
| `ctx.get_as::<T>("name")` | `Option<T>` | Deserializar en T |

### Declarar Dependencias

Use `requires ["step_name"]` para que el ejecutor paralelo sepa propagar la salida:

```rust
step("decision") {
    let score = ctx.get_f64("score").unwrap_or(0.0);
    let valid = ctx.get_bool("validate").unwrap_or(false);
    if valid && score > threshold { "ENTER" } else { "HOLD" }
} requires ["score", "validate"]
```

Si se omite `requires`, `ctx` puede estar vacío (el paso se ejecuta concurrentemente con otros). El parámetro `ctx` siempre está presente — use `_ctx` si no lo necesita.

---

## Umbrales de Constraints Calculados

Los umbrales de constraints ahora aceptan cualquier expresión Rust — la variable `input` está en ámbito:

```rust
pipeline! {
    name: DynamicThreshold,
    input: RiskParams,
    steps: [ step("equity_floor") { input.account_equity_usdt } ],
    constraints: [
        metric("equity_floor").ge(input.min_equity_threshold)
    ]
}
```

---

## Constraints Fail-Fast

Agregue `.fail_fast()` para abortar inmediatamente ante una violación — útil para invariantes estrictas:

```rust
constraints: [
    metric("equity_floor").ge(0.0).fail_fast(),  // abortar si es negativo
    metric("score").le(input.max_score),          // solo se verifica si lo anterior pasa
]
```

Sin `.fail_fast()`, todos los constraints se evalúan y se recopilan todos los fallos. Con él, el pipeline se detiene en la primera violación.

---

## Accesores Tipados de PipelineResult

```rust
let result = executor.run_parallel(&pipeline, &input).await?;
let score: Option<f64>         = result.get_f64("score");
let ok: Option<bool>           = result.get_bool("validate");
let label: Option<&str>        = result.get_str("label");
let decision: Option<MyDecision> = result.get_as::<MyDecision>("decision");

// El acceso directo al mapa sigue funcionando
let raw: &Value = &result.values["score"];
```
