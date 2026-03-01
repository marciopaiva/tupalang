# ExecutionPlan Schema

## Purpose

This document defines the JSON schema emitted by the pipeline codegen and consumed by the runtime.

## Version

- `plan_version`: 1 (implicit in version 0.7.0)

## Fields

- `name`: string — pipeline name
- `version`: string — compiler version
- `seed`: number|null — optional deterministic seed
- `input_schema`: object
  - `kind`: "i64" | "f64" | "bool" | "string" | "array" | "slice" | "ident" | "unknown"
  - `elem`: TypeSchema|null — element type for array/slice
  - `len`: number|null — fixed length for array
  - `name`: string|null — domain type name for `ident`
- `steps`: array<{ name, function_ref, effects[] }>
- `constraints`: array<{ metric, comparator, threshold }>
- `metrics`: object — literal values computed in validation
- `metric_plans`: array<{ name, function_ref, args }>

## Example

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

## Compatibility

- New fields are optional by default for forward compatibility.
- `name` and `steps` should keep the same `function_ref` for traceability.
