# Glossário de tipos

## Objetivo

Listar tipos básicos e compostos com exemplos mínimos.

## Primitivos

- `i64`
  ```tupa
  let n: i64 = 42
  ```
- `f64`
  ```tupa
  let x: f64 = 3.14
  ```
- `bool`
  ```tupa
  let ok: bool = true
  ```
- `string`
  ```tupa
  let name: string = "Tupã"
  ```

## Compostos

- Tuplas
  ```tupa
  let pair: (i64, string) = (1, "a")
  ```
- Arrays
  ```tupa
  let xs: [i64; 3] = [1, 2, 3]
  ```
- Funções
  ```tupa
  let add: fn(i64, i64) -> i64 = soma
  ```

## Tipos seguros

- `Safe<T, !constraint>`
  ```tupa
  let score: Safe<f64, !nan> = 0.9
  ```
