
# Common Errors

## Purpose

Describe frequent errors and quick solutions.

## 1) E1002 — Undefined variable

**Cause**: variable used before being declared.
**Solution**: declare it with `let` before use.

## 2) E2001 — Type mismatch

**Cause**: expected type differs from the found type.
**Solution**: adjust the type annotation or the expression.

**Example:**

```tupa
fn foo(x: int): bool {
  x + true
}
```

Typical message:

```text
error: type mismatch: expected int, found bool
  --> foo.tupa:2:9
  |
2 |     x + true
  |         ^^^^
```

## 3) E2002 — Incorrect arity

**Cause**: argument count does not match the signature.
**Solution**: check the function definition.

**Example:**

```tupa
fn bar(x: int, y: int): int {
  x + y
}
bar(1)
```

Typical message:

```text
error: argument count mismatch: expected 2, found 1
  --> main.tupa:6:1
  |
6 | bar(1)
  | ^^^^^
```

## 4) E2007 — Missing return

**Cause**: the function should return a value, but does not.
**Solution**: add `return` on all paths.

**Example:**

```tupa
fn f(): int {
  // sem return
}
```

Typical message:

```text
error: function does not return a value for type int
  --> main.tupa:1:1
  |
1 | fn f(): int {
  | ^^^^^^^^^^^
```

## 5) E2101 — Lambda type mismatch

**Cause**: lambda body returns a different type than expected.
**Solution**: adjust the body or the annotation.

error: type mismatch: expected int, found string
**Example:**

```tupa
let f: fn(int) -> int = |x| x + "a"
```

Typical message:

```text
error: type mismatch: expected int, found string
  --> main.tupa:1:29
  |
1 | let f: fn(int) -> int = |x| x + "a"
  |                             ^^^^^^
```

## 6) E2102 — Incorrect print usage

**Cause**: invalid argument count for print.
**Solution**: use only one argument.

**Example:**

```tupa
print(1, 2)
```

Typical message:

```text
error: argument count mismatch: expected 1, found 2
  --> main.tupa:1:1
  |
1 | print(1, 2)
  | ^^^^^^^^^
```

## 7) E2103 — Incompatible concatenation

**Cause**: attempt to concatenate string with another type.
**Solution**: convert to string before concatenating.

**Example:**

```tupa
let s = "abc" + 123
```

Typical message:

```text
  --> main.tupa:1:15
  |
1 | let s = "abc" + 123
  |               ^^^
```

## 8) E3002 — Unproven constraint

**Cause**: the compiler cannot prove `Safe<T, ...>`.
**Solution**: use provable literals/constant expressions for `f64`, or pass a `Safe<...>` value already proven.

**Example:**

```tupa
let x: Safe<string, !hate_speech> = "ok"
```

Typical message:

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> main.tupa:1:33
```

**Positive example (propagation):**

```tupa
fn pass(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}
```

## 9) E3001 — Invalid constraint

**Cause**: unsupported constraint or incompatible base type.
**Solution**: use `!nan`/`!inf` with `f64`, and `!hate_speech`/`!misinformation` with `string`.

**Example:**

```tupa
let x: Safe<f64, !hate_speech> = 1.0
```

Typical message:

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> main.tupa:1:32
```

**Example (misinformation):**

```tupa
let x: Safe<f64, !misinformation> = 1.0
```

Typical message:

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> main.tupa:1:35
```

## References

- [Diagnostics Glossary](DIAGNOSTICS_GLOSSARY.md)
- [FAQ](FAQ.md)
