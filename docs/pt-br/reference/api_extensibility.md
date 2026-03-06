# API do Compilador e Extensibilidade

## Propósito

Explicar como usar a API do compilador do Tupã, estender funcionalidades e fazer embedding de Tupã em sistemas Rust.

## Superfície estável de embedding (`v0.8.0-rc`)

A superfície estável de embedding para este ciclo RC é:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

Para exemplos mínimos, veja [Embedding](embedding.md).

## Uso como biblioteca

Cada crate pode ser usada como biblioteca Rust independente:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Pontos de extensão

- **Novos tipos**: implementar e registrar em `tupa-typecheck`.
- **Novos diagnósticos**: adicionar em cada módulo de erro das crates.
- **Novo backend**: criar uma crate (por exemplo, `tupa-backend-wasm`) e implementar o trait `CodegenBackend`.
- **CLI customizado**: usar `tupa-cli` como base e adicionar comandos.

## Exemplo: backend WASM

1. Criar uma nova crate `tupa-backend-wasm`.
2. Implementar o trait `CodegenBackend`.
3. Integrar no CLI.

## Links úteis

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribuição](../../CONTRIBUTING.md)
