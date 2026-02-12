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
- `if_unit_expr.tp`: `if` sem `else` (unit).
- `return_if_expr.tp`: `return` com `if` como expressão.
- `guards.tp`: `match` com guardas.
- `closure_basic.tp`: funções anônimas (lambdas) básicas.
- `match.tp`: `match` com guardas e wildcard.
- `match_guard_if_expr.tp`: `match` com guarda usando `if` como expressão.
- `arrays.tp`: literais de array, indexação e tipos `[T; N]`.
- `float_array_ops.tp`: arrays de `f64` e indexação.
- `string_array_ops.tp`: arrays de `string` e indexação.
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
- `string_plus_eq.tp`: concatenação com `+=`.
- `function_call.tp`: chamada de função definida pelo usuário.
- `enum_basic.tp`: declaração básica de enum.
- `trait_basic.tp`: declaração básica de trait.
- `safe_hate_speech_propagation.tp`: propagação de constraint ética via parâmetro `Safe`.
- `safe_misinformation_return.tp`: propagação de constraint ética via retorno `Safe`.
- `safe_misinformation_hate_speech.tp`: propagação com múltiplas constraints éticas.

### Casos negativos (devem falhar)

- `invalid_lex_char.tp`: caractere inválido no lexer.
- `invalid_parse_missing_semicolon.tp`: `;` ausente no `let`.
- `invalid_type.tp`: erro de tipo em `let`.
- `invalid_return.tp`: falta de retorno em função não-`unit`.
- `invalid_call.tp`: aridade incorreta de chamada.
- `invalid_unknown_var.tp`: uso de variável inexistente.
- `invalid_unknown_function.tp`: chamada de função inexistente.
- `invalid_unknown_type.tp`: tipo desconhecido.
- `invalid_call_target.tp`: chamada em valor não-funcional.
- `invalid_binary_op.tp`: tipos inválidos em operação binária.
- `invalid_unary_op.tp`: tipo inválido em operação unária.
- `invalid_break.tp`: `break` fora de loop.
- `invalid_continue.tp`: `continue` fora de loop.
- `invalid_print_arity.tp`: aridade inválida em `print`.
- `invalid_match_guard.tp`: guarda de `match` com tipo inválido.
- `invalid_match_pattern.tp`: padrão de `match` com tipo inválido.
- `invalid_match_arm_type.tp`: braços de `match` com tipos divergentes.
- `invalid_index_type.tp`: índice com tipo inválido.
- `invalid_index_base.tp`: indexação em base não-array.
- `invalid_array_mixed.tp`: array com tipos mistos.
- `invalid_assign_type.tp`: atribuição com tipo inválido.
- `invalid_assign_index_value.tp`: atribuição em index com tipo inválido.
- `invalid_return_type.tp`: retorno com tipo inválido.
- `invalid_if_condition.tp`: condição de `if` com tipo inválido.
- `invalid_while_condition.tp`: condição de `while` com tipo inválido.
- `invalid_for_range_type.tp`: `for in` sobre tipo inválido.
- `invalid_range_bounds.tp`: limites de `range` inválidos.
- `invalid_safe_hate_speech.tp`: constraint ética não provada em `Safe<string, ...>`.
- `invalid_safe_hate_speech_base.tp`: constraint ética usada em base inválida.
- `invalid_safe_misinformation.tp`: constraint ética não provada em `Safe<string, ...>`.
- `invalid_safe_misinformation_base.tp`: constraint ética usada em base inválida.
- `invalid_safe_param_base.tp`: constraint inválida em parâmetro `Safe`.
- `invalid_safe_return_base.tp`: constraint inválida em retorno `Safe`.

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
cargo run -p tupa-cli -- lex examples/invalid_lex_char.tp
cargo run -p tupa-cli -- parse examples/invalid_parse_missing_semicolon.tp
cargo run -p tupa-cli -- check examples/invalid_type.tp
cargo run -p tupa-cli -- check examples/invalid_return.tp
cargo run -p tupa-cli -- check examples/invalid_call.tp
```
