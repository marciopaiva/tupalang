# Sistema de Efectos

## Idea

Identificar efectos en expresiones para validación (p. ej., determinismo).

## Efectos soportados

- IO (p. ej., `print`)
- Random (p. ej., `random`)
- Time (p. ej., `time`, `now`)
- Utilidad pura (p. ej., `hash`)

## Uso en Pipelines

- `@deterministic` rechana `Random` y `Time` en pasos.
- `hash(...)` se trata como puro y se permite en pipelines determinísticos.
- `now()`/`time()` se tratan como `Time` y se rechazan bajo `@deterministic`.
- Diagnóstico: E2005 (impuro en determinístico).