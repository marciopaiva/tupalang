# Guía de Pipeline

## Objetivo

Ejecutar un pipeline Tupã de extremo a extremo: generar un ExecutionPlan y ejecutar con entrada JSON.

## Pasos

- Generar el plan:
  - `tupa codegen --format=json examples/pipeline/fraud_complete.tp`
- Ejecutar el pipeline:
  - `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp`

### Solo plan

- `tupa codegen --plan-only examples/pipeline/fraud_complete.tp`
- `tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json`

### Persistir salida

- `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json --output out.json examples/pipeline/fraud_complete.tp`

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
