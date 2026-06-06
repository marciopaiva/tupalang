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
