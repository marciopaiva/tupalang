# Sistema de Efectos

## Idea

Identificar efectos en expresiones para validación (p. ej., determinismo).

## Efectos soportados

- IO (p. ej., `print`)
- Random (p. ej., `random`)
- Time (p. ej., `time`, `now`)

## Uso en Pipelines

- `@deterministic` rechaza `Random` y `Time` en pasos.
- Diagnóstico: E2005 (impuro en determinístico).
