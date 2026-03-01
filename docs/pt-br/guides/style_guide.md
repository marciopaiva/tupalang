# Guia de Estilo da Linguagem

## Propósito

Definir convenções de nomeação, formatação e exemplos para a linguagem.

## Nomes

- **Funções**: `snake_case`.
- **Variáveis**: `snake_case`.
- **Tipos**: `PascalCase`.
- **Constantes**: `SCREAMING_SNAKE_CASE`.

## Formatação

- Recuo com 2 espaços nos exemplos.
- Um espaço após `,` e `:`.
- Chaves em linha separada apenas para blocos multilinha.

## Comentários

- Prefira comentários curtos e objetivos.
- Evite comentários que repitam o óbvio.

## Boas práticas

- Prefira `let` com tipo explícito em exemplos didáticos.
- Use `Safe<T, !constraint>` em exemplos sensíveis à segurança.
- Mantenha exemplos pequenos e focados.

## Exemplos

```tupa
fn sum(a: i64, b: i64) -> i64 {
  return a + b
}

let score: Safe<f64, !nan> = risk_score(tx)
let summary: Safe<string, !misinformation, !hate_speech> = review(text)
```
