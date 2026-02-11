
# Erros comuns

## Objetivo

Descrever erros frequentes e soluções rápidas.

## 1) E1002 — Variável não definida

**Causa**: variável usada antes de ser declarada.
**Solução**: declare com `let` antes do uso.

## 2) E2001 — Type mismatch

**Causa**: tipo esperado difere do encontrado.
**Solução**: ajuste a anotação de tipo ou a expressão.

**Exemplo:**

```tupa
fn foo(x: int): bool {
  x + true
}
```

Mensagem típica:

```text
erro: tipo incompatível: esperado int, encontrado bool
  --> foo.tupa:2:9
  |
2 |     x + true
  |         ^^^^
```

## 3) E2002 — Aridade incorreta

**Causa**: número de argumentos não bate com a assinatura.
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
erro: número de argumentos incompatível: esperado 2, encontrado 1
  --> main.tupa:6:1
  |
6 | bar(1)
  | ^^^^^
```

## 4) E2007 — Retorno ausente

**Causa**: função deveria retornar valor, mas não retorna.
**Solução**: adicione `return` em todos os caminhos.

**Exemplo:**

```tupa
fn f(): int {
  // sem return
}
```

Mensagem típica:

```text
erro: função não retorna valor para tipo int
  --> main.tupa:1:1
  |
1 | fn f(): int {
  | ^^^^^^^^^^^
```

## 5) E2101 — Lambda com tipo incompatível

**Causa**: corpo da lambda retorna tipo diferente do esperado.
**Solução**: ajuste o corpo ou a anotação.

erro: tipo incompatível: esperado int, encontrado string
**Exemplo:**

```tupa
let f: fn(int) -> int = |x| x + "a"
```

Mensagem típica:

```text
erro: tipo incompatível: esperado int, encontrado string
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
erro: número de argumentos incompatível: esperado 1, encontrado 2
  --> main.tupa:1:1
  |
1 | print(1, 2)
  | ^^^^^^^^^
```

## 7) E2103 — Concatenação de tipos incompatíveis

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

## 8) E3002 — Constraint não provada

**Causa**: o compilador não consegue provar `Safe<T, ...>`.
**Solução**: use literais `f64` e expressões constantes simples, ou evite `Safe<...>` nesse ponto.

## 9) E3001 — Constraint inválida

**Causa**: constraint não suportada ou tipo base incompatível.
**Solução**: use apenas `!nan`/`!inf` com base `f64`.

## Referências

- [Glossário de Diagnósticos](DIAGNOSTICS_GLOSSARY.md)
- [FAQ](FAQ.md)
