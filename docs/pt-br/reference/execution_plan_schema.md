# Esquema do ExecutionPlan

## Versão

- `plan_version`: 1 (implícito na versão 0.7.0)

## Campos

- `name`: string — nome do pipeline
- `version`: string — versão do compilador
- `seed`: number|null — seed determinística opcional
- `input_schema`: object
  - `kind`: "i64" | "f64" | "bool" | "string" | "array" | "slice" | "ident" | "unknown"
  - `elem`: TypeSchema|null — tipo do elemento para array/slice
  - `len`: number|null — comprimento fixo para array
  - `name`: string|null — nome do tipo de domínio para `ident`
- `steps`: array<{ name, function_ref, effects[] }>
- `constraints`: array<{ metric, comparator, threshold }>
- `metrics`: object — valores literais computados na validação
- `metric_plans`: array<{ name, function_ref, args }>

## Exemplo

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

## Compatibilidade

- Novos campos são opcionais por padrão para forward-compat.
- `name` e `steps` devem manter o mesmo `function_ref` para rastreabilidade.
