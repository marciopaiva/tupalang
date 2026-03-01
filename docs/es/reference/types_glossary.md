# Glosario de Tipos

## Propósito

Listar tipos básicos y compuestos con ejemplos mínimos.

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

## Tipos compuestos

- Tuplas

  ```tupa
  let pair: (i64, string) = (1, "a")
  ```

- Arrays

  ```tupa
  let xs: [i64; 3] = [1, 2, 3]
  ```

- Funciones

  ```tupa
  let add: fn(i64, i64) -> i64 = sum
  ```

## Tipos Safe

- `Safe<T, !constraint>`

  ```tupa
  let score: Safe<f64, !nan> = 0.9
  let summary: Safe<string, !misinformation> = review(text)
  ```
