# Guia de Estilo da Linguagem

## Objetivo

Definir convenções de nomenclatura, formatação e padrões de exemplo para a linguagem.

## Nomes

- **Funções**: `snake_case`.
- **Variáveis**: `snake_case`.
- **Tipos**: `PascalCase`.
- **Constantes**: `SCREAMING_SNAKE_CASE`.

## Formatação

- Indentação com 2 espaços em exemplos.
- Um espaço após `,` e `:`.
- Chaves em linha separada apenas para blocos multilinha.

## Comentários

- Prefira comentários curtos e objetivos.
- Evite comentários que repetem o óbvio.

## Boas práticas

- Prefira `let` com tipo explícito em exemplos didáticos.
- Use `Safe<T, !constraint>` em exemplos sensíveis a segurança.
- Mantenha exemplos pequenos e focados.

## Exemplos

```tupa
fn soma(a: i64, b: i64) -> i64 {
  return a + b
}

let score: Safe<f64, !nan> = risk_score(tx)
```
