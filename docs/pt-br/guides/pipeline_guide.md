# Guia de Pipeline

## Objetivo

Executar um pipeline Tupã de ponta a ponta: gerar um ExecutionPlan e executar com entrada JSON.

## Passos

- Escreva o pipeline com a macro `pipeline!` (veja [getting_started.md](getting_started.md)).
- Verifique os tipos: `cargo tupa check`.
- Execute com entrada JSON: `cargo tupa run --input data.json`.
- Execução paralela: `cargo tupa run --parallel --input data.json`.
- Persistir métricas por passo: `cargo tupa run --input data.json --metrics-output metrics.json`.

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
