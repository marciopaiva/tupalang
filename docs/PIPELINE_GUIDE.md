# Pipeline Guide

## Objetivo
Executar um pipeline Tupã do zero: gerar ExecutionPlan e rodar com JSON.

## Passos
- Gere o plano:
  - `tupa codegen --format=json examples/pipeline/fraud_complete.tp`
- Rode o pipeline:
  - `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp`

### Somente plano
- `tupa codegen --plan-only examples/pipeline/fraud_complete.tp`
- `tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json`

### Persistir saída
- `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json --output out.json examples/pipeline/fraud_complete.tp`

## Estrutura de ExecutionPlan
- name, version, seed (opcional), input_schema
- steps: name, function_ref, effects
- constraints: metric, comparator, threshold
 - metrics: valores literais da validation
 - metric_plans: { name, function_ref, args } para calcular métricas no runtime

## Observações
- function_ref segue `<arquivo>::step_<nome>`.
- Efeitos (random/time) são identificados pelo typechecker.
- O runtime avalia constraints e gera relatório JSON com metrics/constraints.
