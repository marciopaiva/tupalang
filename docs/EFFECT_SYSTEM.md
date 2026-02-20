# Effect System

## Ideia
Identificar efeitos em expressões para validação (ex.: determinismo).

## Efeitos suportados
- IO (ex.: `print`)
- Random (ex.: `random`)
- Time (ex.: `time`, `now`)

## Uso em Pipelines
- `@deterministic` rejeita `Random` e `Time` em steps.
- Diagnóstico: E2005 (impure in deterministic).
