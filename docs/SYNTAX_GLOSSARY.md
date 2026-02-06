# Glossário de sintaxe

## Objetivo

Apresentar exemplos mínimos das construções básicas da linguagem.

## Funções

```tupa
fn soma(a: i64, b: i64) -> i64 {
	return a + b
}
```

## Variáveis

```tupa
let x: i64 = 10
let y = 20
```

## Condicionais

```tupa
if x > 0 {
	print("positivo")
} else {
	print("não-positivo")
}
```

## Match

```tupa
match status {
	200 => print("OK"),
	404 => print("Not Found"),
	_ => print("Outro")
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

## Tipos seguros

```tupa
let score: Safe<f64, !nan> = 0.9
```
