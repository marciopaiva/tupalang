# Codegen

## Objetivo

Descrever o estado atual do `tupa-codegen` e o fluxo `parse -> typecheck -> codegen`.

O `tupa-codegen` gera um IR textual estilo LLVM (não é LLVM completo) para demonstrar o pipeline.

## Uso via CLI

```bash
cargo run -p tupa-cli -- codegen examples/hello.tp
cargo run -p tupa-cli -- codegen examples/arith.tp
cargo run -p tupa-cli -- codegen examples/array_ops.tp

# saída JSON
cargo run -p tupa-cli -- codegen --format json examples/hello.tp
```

## Saída atual

Saída em IR textual (exemplo simplificado):

```text
declare i32 @printf(i8*, ...)
@.fmt_int = private unnamed_addr constant [5 x i8] c"%ld\0A\00"

define void @main() {
entry:
  %t0 = alloca i64
  store i64 42, i64* %t0
  ret void
}
```

Em JSON, a saída vem encapsulada em um objeto:

```json
{
  "codegen": "declare i32 @printf(i8*, ...)\n..."
}
```

## Recursos suportados

- Literais `i64`, `f64`, `bool` e `string` (strings são constantes globais)
- `let`, `return`, `print`
- Operadores aritméticos e comparações em `i64` e `f64`
- `if`/`match` (inclui guardas, bind de identificadores e `match` em `string` via `strcmp`)
- `while`, `for` em `range`, `break`/`continue`
- Arrays de `i64`, `f64` e `string`, indexação e atribuição
- Chamadas a funções definidas pelo usuário
- Concatenação de strings em runtime
- `+=` para strings (via concatenação)

## Próximos passos

- Reduzir `TODO` residuais no codegen
- Suportar mais tipos e chamadas
- Emitir binário nativo via `llvm`/`inkwell`
