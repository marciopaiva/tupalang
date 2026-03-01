# Tutoriais passo a passo

## Propósito

Guiar usuários de diferentes níveis por tarefas comuns e projetos de exemplo com Tupã.

---

## 1. Olá, Mundo

```tupa
print("Olá, Tupã!")
```

Execute:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

---

## 2. Funções e lambdas

```tupa
let inc: fn(int) -> int = |x| x + 1
print(inc(41)) // saída: 42
```

---

## 3. Trabalhando com strings

```tupa
let name = "Tupã"
print("Bem-vindo, " + name)
```

---

## 4. Funções com tipos Safe

```tupa
fn safe(x: f64): Safe<f64, !nan> {
  return x
}

fn safe_text(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}
```

---

## 5. Projeto de exemplo: Soma de vetores

Arquivo: `examples/soma_vetor.tp`

```tupa
fn sum(v: [int]) -> int {
  let mut total = 0
  for x in v {
    total = total + x
  }
  return total
}
print(sum([1,2,3,4])) // saída: 10
```

---

## 6. Depuração e diagnósticos

- Veja [docs/common_errors.md](../reference/common_errors.md) para exemplos de erros.
- Use `cargo test` para executar todos os testes.

---

## 7. Contribuindo com exemplos

- Adicione novos tutoriais em `docs/en/guides/tutorials.md`.
- Veja [CONTRIBUTING.md](../../CONTRIBUTING.md) para orientações.
