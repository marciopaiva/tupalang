
# Errores Comunes

## Propósito

Describir errores frecuentes y soluciones rápidas.

## 1) E1002 — Variable indefinida

**Causa**: variable usada antes de ser declarada.
**Solución**: declárala con `let` antes de usar.

## 2) E2001 — Tipo incompatible

**Causa**: el tipo esperado difiere del tipo encontrado.
**Solución**: ajusta la anotación de tipo o la expresión.

**Ejemplo:**

```tupa
fn foo(x: int): bool {
  x + true
}
```

Mensaje típico:

```text
error: type mismatch: expected int, found bool
  --> foo.tupa:2:9
  |
2 |     x + true
  |         ^^^^
```

## 3) E2002 — Aridad incorrecta

**Causa**: el número de argumentos no coincide con la firma.
**Solución**: revisa la definición de la función.

**Ejemplo:**

```tupa
fn bar(x: int, y: int): int {
  x + y
}
bar(1)
```

Mensaje típico:

```text
error: argument count mismatch: expected 2, found 1
  --> main.tupa:6:1
  |
6 | bar(1)
  | ^^^^^
```

## 4) E2007 — Retorno ausente

**Causa**: la función debería retornar un valor, pero no lo hace.
**Solución**: agrega `return` en todos los caminos.

**Ejemplo:**

```tupa
fn f(): int {
  // sem return
}
```

Mensaje típico:

```text
error: function does not return a value for type int
  --> main.tupa:1:1
  |
1 | fn f(): int {
  | ^^^^^^^^^^^
```

## 5) E2101 — Tipo incompatible en lambda

**Causa**: el cuerpo de la lambda devuelve un tipo distinto al esperado.
**Solución**: ajusta el cuerpo o la anotación.

error: type mismatch: expected int, found string
**Ejemplo:**

```tupa
let f: fn(int) -> int = |x| x + "a"
```

Mensaje típico:

```text
error: type mismatch: expected int, found string
  --> main.tupa:1:29
  |
1 | let f: fn(int) -> int = |x| x + "a"
  |                             ^^^^^^
```

## 6) E2102 — Uso incorrecto de print

**Causa**: número de argumentos inválido para print.
**Solución**: usa solo un argumento.

**Ejemplo:**

```tupa
print(1, 2)
```

Mensaje típico:

```text
error: argument count mismatch: expected 1, found 2
  --> main.tupa:1:1
  |
1 | print(1, 2)
  | ^^^^^^^^^
```

## 7) E2103 — Concatenación incompatible

**Causa**: intento de concatenar string con otro tipo.
**Solución**: convierte a string antes de concatenar.

**Ejemplo:**

```tupa
let s = "abc" + 123
```

Mensaje típico:

```text
  --> main.tupa:1:15
  |
1 | let s = "abc" + 123
  |               ^^^
```

## 8) E3002 — Restricción no comprobada

**Causa**: el compilador no puede comprobar `Safe<T, ...>`.
**Solución**: usa literales/expresiones constantes comprobables para `f64` o pasa un valor `Safe<...>` ya comprobado.

**Ejemplo:**

```tupa
let x: Safe<string, !hate_speech> = "ok"
```

Mensaje típico:

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> main.tupa:1:33
```

**Ejemplo positivo (propagación):**

```tupa
fn pass(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}
```

## 9) E3001 — Restricción inválida

**Causa**: restricción no soportada o tipo base incompatible.
**Solución**: usa `!nan`/`!inf` con `f64` y `!hate_speech`/`!misinformation` con `string`.

**Ejemplo:**

```tupa
let x: Safe<f64, !hate_speech> = 1.0
```

Mensaje típico:

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> main.tupa:1:32
```

**Ejemplo (misinformation):**

```tupa
let x: Safe<f64, !misinformation> = 1.0
```

Mensaje típico:

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> main.tupa:1:35
```

## Referencias

- [Glosario de Diagnósticos](diagnostics_glossary.md)
- [Guía de Ejemplos](../guides/examples_guide.md)
