# Esquema de ExecutionPlan

## Versión

- `plan_version`: 1 (implícito en la versión 0.7.0)

## Campos

- `name`: string — nombre del pipeline
- `version`: string — versión del compilador
- `seed`: number|null — seed determinística opcional
- `input_schema`: object
  - `kind`: "i64" | "f64" | "bool" | "string" | "array" | "slice" | "ident" | "unknown"
  - `elem`: TypeSchema|null — tipo de elemento para array/slice
  - `len`: number|null — longitud fija para array
  - `name`: string|null — nombre del tipo de dominio para `ident`
- `steps`: array<{ name, function_ref, effects[] }>
- `constraints`: array<{ metric, comparator, threshold }>
- `metrics`: object — valores literales calculados en la validación
- `metric_plans`: array<{ name, function_ref, args }>

## Ejemplo

```json
{
  "name": "FraudDetection",
  "version": "0.8.0",
  "seed": 42,
  "input_schema": { "kind": "ident", "name": "Transaction" },
  "steps": [
    { "name": "enrich", "function_ref": "fraud_complete::step_enrich", "effects": [] }
  ],
  "constraints": [
    { "metric": "false_positive_rate", "comparator": "lt", "threshold": 0.01 }
  ],
  "metrics": {},
  "metric_plans": [
    { "name": "false_positive_rate", "function_ref": "fraud_complete::compute_fpr", "args": [true, false] }
  ]
}
```

## Compatibilidad

- Los campos nuevos son opcionales por defecto para forward-compat.
- `name` y `steps` deben mantener el mismo `function_ref` para trazabilidad.
