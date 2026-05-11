# Glosario de Sintaxis

## Propósito

Presentar ejemplos mínimos de construcciones básicas del lenguaje.

## Funciones

```tupa
fn sum(a: i64, b: i64) -> i64 {
  return a + b
}
```text

## Variables

```tupa
let x: i64 = 10
let y = 20
```text

## Condicionales

```tupa
if x > 0 {
  print("positivo")
} else {
  print("no positivo")
}
```text

## Match

```tupa
match status {
  200 => print("OK"),
  404 => print("No encontrado"),
  _ => print("Otro")
}
```text

## Arrays

```tupa
let xs = [1, 2, 3]
```text

## Bucles

```tupa
while i < 10 {
  i = i + 1
}

for i in 0..10 {
  print(i)
}
```text

## Tipos Safe

```tupa
let summary: Safe<string, !misinformation> = reviewed_summary
let score: Safe<f64, !nan> = 0.9
```text
