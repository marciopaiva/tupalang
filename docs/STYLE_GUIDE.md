# Language Style Guide

## Purpose

Define naming, formatting, and example conventions for the language.

## Names

- **Functions**: `snake_case`.
- **Variables**: `snake_case`.
- **Types**: `PascalCase`.
- **Constants**: `SCREAMING_SNAKE_CASE`.

## Formatting

- Indent with 2 spaces in examples.
- One space after `,` and `:`.
- Braces on a separate line only for multiline blocks.

## Comments

- Prefer short, objective comments.
- Avoid comments that repeat the obvious.

## Best practices

- Prefer `let` with explicit type in didactic examples.
- Use `Safe<T, !constraint>` in safety-sensitive examples.
- Keep examples small and focused.

## Examples

```tupa
fn sum(a: i64, b: i64) -> i64 {
  return a + b
}

let score: Safe<f64, !nan> = risk_score(tx)
```
