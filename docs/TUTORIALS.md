# Tutoriais Passo a Passo

## Objetivo

Guiar usuários de diferentes níveis em tarefas comuns e projetos exemplo com Tupã.

---

## 1. Olá, Mundo!

```tupa
print("Olá, Tupã!")
```

Execute:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

---

## 2. Funções e Lambdas

```tupa
let inc: fn(int) -> int = |x| x + 1
print(inc(41)) // saída: 42
```

---

## 3. Manipulando Strings

```tupa
let nome = "Tupã"
print("Bem-vindo, " + nome)
```

---

## 4. Funções com Tipos Seguros

```tupa
fn seguro(x: f64): Safe<f64, !nan> {
  return x
}
```

---

## 5. Projeto Exemplo: Soma de Vetor

Arquivo: `examples/soma_vetor.tp`

```tupa
fn soma(v: [int]) -> int {
  let mut total = 0
  for x in v {
    total = total + x
  }
  return total
}
print(soma([1,2,3,4])) // saída: 10
```

---

## 6. Debug e Diagnóstico

- Consulte [docs/COMMON_ERRORS.md](COMMON_ERRORS.md) para exemplos de erros.
- Use `cargo test` para rodar todos os testes.

---

## 7. Contribuindo com exemplos

- Adicione novos tutoriais em `docs/TUTORIALS.md`.
- Veja [CONTRIBUTING.md](../CONTRIBUTING.md) para diretrizes.
