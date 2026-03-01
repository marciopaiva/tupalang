
# Erros Comuns

## Propósito

Descrever erros frequentes e soluções rápidas.

## 1) E1002 — Variável indefinida

**Causa**: variável usada antes de ser declarada.
**Solução**: declare com `let` antes de usar.

## 2) E2001 — Tipo incompatível

**Causa**: tipo esperado difere do tipo encontrado.
**Solução**: ajuste a anotação de tipo ou a expressão.

**Exemplo:**

```tupa
fn foo(x: int): bool {
  x + true
}
```

Mensagem típica:

```text
error: type mismatch: expected int, found bool
  --> foo.tupa:2:9
  |
2 |     x + true
  |         ^^^^
```

## 3) E2002 — Aridade incorreta

**Causa**: número de argumentos não corresponde à assinatura.
**Solução**: verifique a definição da função.

**Exemplo:**

```tupa
fn bar(x: int, y: int): int {
  x + y
}
bar(1)
```

Mensagem típica:

```text
error: argument count mismatch: expected 2, found 1
  --> main.tupa:6:1
  |
6 | bar(1)
  | ^^^^^
```

## 4) E2007 — Retorno ausente

**Causa**: a função deveria retornar um valor, mas não retorna.
**Solução**: adicione `return` em todos os caminhos.

**Exemplo:**

```tupa
fn f(): int {
  // sem return
}
```

Mensagem típica:

```text
error: function does not return a value for type int
  --> main.tupa:1:1
  |
1 | fn f(): int {
  | ^^^^^^^^^^^
```

## 5) E2101 — Tipo incompatível em lambda

**Causa**: o corpo da lambda retorna um tipo diferente do esperado.
**Solução**: ajuste o corpo ou a anotação.

error: type mismatch: expected int, found string
**Exemplo:**

```tupa
let f: fn(int) -> int = |x| x + "a"
```

Mensagem típica:

```text
error: type mismatch: expected int, found string
  --> main.tupa:1:29
  |
1 | let f: fn(int) -> int = |x| x + "a"
  |                             ^^^^^^
```

## 6) E2102 — Uso incorreto de print

**Causa**: número de argumentos inválido para print.
**Solução**: use apenas um argumento.

**Exemplo:**

```tupa
print(1, 2)
```

Mensagem típica:

```text
error: argument count mismatch: expected 1, found 2
  --> main.tupa:1:1
  |
1 | print(1, 2)
  | ^^^^^^^^^
```

## 7) E2103 — Concatenação incompatível

**Causa**: tentativa de concatenar string com outro tipo.
**Solução**: converta para string antes de concatenar.

**Exemplo:**

```tupa
let s = "abc" + 123
```

Mensagem típica:

```text
  --> main.tupa:1:15
  |
1 | let s = "abc" + 123
  |               ^^^
```

## 8) E3002 — Restrição não comprovada

**Causa**: o compilador não consegue comprovar `Safe<T, ...>`.
**Solução**: use literais/expressões constantes comprováveis para `f64` ou passe um valor `Safe<...>` já comprovado.

**Exemplo:**

```tupa
let x: Safe<string, !hate_speech> = "ok"
```

Mensagem típica:

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> main.tupa:1:33
```

**Exemplo positivo (propagação):**

```tupa
fn pass(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}
```

## 9) E3001 — Restrição inválida

**Causa**: restrição não suportada ou tipo base incompatível.
**Solução**: use `!nan`/`!inf` com `f64` e `!hate_speech`/`!misinformation` com `string`.

**Exemplo:**

```tupa
let x: Safe<f64, !hate_speech> = 1.0
```

Mensagem típica:

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> main.tupa:1:32
```

**Exemplo (misinformation):**

```tupa
let x: Safe<f64, !misinformation> = 1.0
```

Mensagem típica:

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> main.tupa:1:35
```

## Referências

- [Glossário de Diagnósticos](diagnostics_glossary.md)
- [Guia de Exemplos](../guides/examples_guide.md)
