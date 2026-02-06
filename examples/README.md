# Exemplos

Exemplos executáveis para o estado atual do parser e typechecker.

## Arquivos

- `hello.tp`: sintaxe básica (`fn`, `let`, chamadas).
- `functions.tp`: tipos de função e chamadas via variável.
- `if_else.tp`: fluxo de controle com `if/else if/else`.
- `guards.tp`: `match` com guardas.
- `match.tp`: `match` com guardas e wildcard.
- `arrays.tp`: literais de array, indexação e tipos `[T; N]`.
- `loops.tp`: `while` e `for in` com range (`..`).
- `types.tp`: anotações de tipo e retornos explícitos.
- `hello.codegen.txt`: saída esperada (stub) do `codegen` para `hello.tp`.

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
