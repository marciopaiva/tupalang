# Syntax Glossary

## Purpose

Present minimal examples of basic language constructs.

## Functions

```tupa
fn sum(a: i64, b: i64) -> i64 {
  return a + b
}
```

## Variables

```tupa
let x: i64 = 10
let y = 20
```

## Conditionals

```tupa
if x > 0 {
  print("positive")
} else {
  print("non-positive")
}
```

## Match

```tupa
match status {
  200 => print("OK"),
  404 => print("Not Found"),
  _ => print("Other")
}
```

## Arrays

```tupa
let xs = [1, 2, 3]
```

## Loops

```tupa
while i < 10 {
  i = i + 1
}

for i in 0..10 {
  print(i)
}
```

## Safe types

```tupa
let summary: Safe<string, !misinformation> = reviewed_summary
let score: Safe<f64, !nan> = 0.9
```
