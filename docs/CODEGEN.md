# Codegen (stub)

## Objetivo

Descrever o estado atual do `tupa-codegen` e o fluxo `parse -> typecheck -> codegen`.

O `tupa-codegen` ainda é um *stub*. O objetivo atual é provar o fluxo `parse -> typecheck -> codegen`.

## Uso via CLI

```bash
cargo run -p tupa-cli -- codegen examples/hello.tp

# saída JSON
cargo run -p tupa-cli -- codegen --format json examples/hello.tp
```

## Saída atual

A saída é um placeholder:

```text
// TODO: LLVM codegen stub
```

Em JSON, a saída vem encapsulada em um objeto:

```json
{
	"codegen": "// TODO: LLVM codegen stub"
}
```

## Próximos passos

- Gerar IR LLVM mínimo para `fn main()`
- Suportar `let`, `return` e chamadas simples
- Emitir binário nativo via `llvm`/`inkwell`
