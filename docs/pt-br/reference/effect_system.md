# Sistema de Efeitos

## Ideia

Identificar efeitos em expressões para validação (ex.: determinismo).

## Efeitos suportados

- IO (ex.: `print`)
- Random (ex.: `random`)
- Time (ex.: `time`, `now`)
- Utilitário puro (ex.: `hash`)

## Uso em Pipelines

- `@deterministic` rejeita `Random` e `Time` em etapas.
- `hash(...)` é tratado como puro e permitido em pipelines determinísticos.
- `now()`/`time()` são tratados como `Time` e rejeitados em `@deterministic`.
- Diagnóstico: E2005 (impuro em determinístico).
