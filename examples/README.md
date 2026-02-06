# Exemplos

## Objetivo

Reunir exemplos curados que refletem o estado atual do parser, typechecker e codegen.

## Curadoria e playground

- Use esta pasta para exemplos curados e estáveis.
- Use [examples/playground](playground/README.md) para testes rápidos e experimentos.

## Arquivos

- `hello.tp`: sintaxe básica (`fn`, `let`, chamadas).
- `functions.tp`: tipos de função e chamadas via variável.
- `if_else.tp`: fluxo de controle com `if/else if/else`.
- `if_expr.tp`: `if` como expressão.
- `return_if_expr.tp`: `return` com `if` como expressão.
- `guards.tp`: `match` com guardas.
- `match.tp`: `match` com guardas e wildcard.
- `match_guard_if_expr.tp`: `match` com guarda usando `if` como expressão.
- `arrays.tp`: literais de array, indexação e tipos `[T; N]`.
- `float_array_ops.tp`: arrays de `f64` e indexação.
- `loops.tp`: `while` e `for in` com range (`..`).
- `types.tp`: anotações de tipo e retornos explícitos.
- `arith.tp`: aritmética e concatenação básica.
- `if_match.tp`: `if` e `match` com strings.
- `while.tp`: laço `while`.
- `for_range.tp`: `for` com range.
- `break_continue.tp`: controle de loop.
- `array_ops.tp`: arrays, indexação e atribuição.
- `bool_print.tp`: impressão de booleanos.
- `bool_ops.tp`: `&&`, `||`, `==`, `!=`.
- `unary_ops.tp`: `-` e `!`.
- `pow_ops.tp`: potência `**` em `i64`.
- `match_guard.tp`: `match` com guardas.
- `match_bind.tp`: bind de identificadores em `match`.
- `match_expr.tp`: `match` como expressão.
- `string_eq.tp`: igualdade/inegualdade de strings.
- `string_concat.tp`: concatenação de strings em runtime.
- `function_call.tp`: chamada de função definida pelo usuário.
- `float_ops.tp`: operações e comparação com `f64`.

### Casos negativos (devem falhar)

- `invalid_type.tp`: erro de tipo em `let`.
- `invalid_return.tp`: falta de retorno em função não-`unit`.
- `invalid_call.tp`: aridade incorreta de chamada.

## Como testar

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- check examples/match.tp

# saída JSON
cargo run -p tupa-cli -- lex --format json examples/hello.tp
cargo run -p tupa-cli -- check --format json examples/hello.tp

# golden outputs (CLI)
cargo test -p tupa-cli -- tests::cli_golden

# exemplos com erro (espera falha)
cargo run -p tupa-cli -- check examples/invalid_type.tp
cargo run -p tupa-cli -- check examples/invalid_return.tp
cargo run -p tupa-cli -- check examples/invalid_call.tp
```
