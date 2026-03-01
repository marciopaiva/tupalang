# Guía de Estilo del Lenguaje

## Propósito

Definir convenciones de nombres, formato y ejemplos para el lenguaje.

## Nombres

- **Funciones**: `snake_case`.
- **Variables**: `snake_case`.
- **Tipos**: `PascalCase`.
- **Constantes**: `SCREAMING_SNAKE_CASE`.

## Formato

- Indenta con 2 espacios en los ejemplos.
- Un espacio después de `,` y `:`.
- Llaves en línea separada solo para bloques multilínea.

## Comentarios

- Prefiere comentarios cortos y objetivos.
- Evita comentarios que repitan lo obvio.

## Buenas prácticas

- Prefiere `let` con tipo explícito en ejemplos didácticos.
- Usa `Safe<T, !constraint>` en ejemplos sensibles a seguridad.
- Mantén ejemplos pequeños y enfocados.

## Ejemplos

```tupa
fn sum(a: i64, b: i64) -> i64 {
  return a + b
}

let score: Safe<f64, !nan> = risk_score(tx)
let summary: Safe<string, !misinformation, !hate_speech> = review(text)
```
