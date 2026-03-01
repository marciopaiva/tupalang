# Guia de Pipeline

## Objetivo

Executar um pipeline Tupã de ponta a ponta: gerar um ExecutionPlan e executar com entrada JSON.

## Passos

- Gerar o plano:
  - `tupa codegen --format=json examples/pipeline/fraud_complete.tp`
- Executar o pipeline:
  - `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp`

### Somente plano

- `tupa codegen --plan-only examples/pipeline/fraud_complete.tp`
- `tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json`

### Persistir saída

- `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json --output out.json examples/pipeline/fraud_complete.tp`

## Estrutura do ExecutionPlan

- name, version, seed (opcional), input_schema
- steps: name, function_ref, effects
- constraints: metric, comparator, threshold
- metrics: valores literais capturados do bloco de validação
- metric_plans: { name, function_ref, args } para calcular métricas em runtime

## Notas

- Formato de function_ref: `<file>::step_<name>`.
- Efeitos (random/time) são identificados pelo typechecker.
- O runtime avalia restrições e emite um relatório JSON com métricas/restrições.
