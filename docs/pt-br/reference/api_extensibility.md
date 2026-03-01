# API do Compilador e Extensibilidade

## Propósito

Explicar como usar a API interna do compilador Tupã, estender funcionalidades e adicionar novos backends.

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

## Exemplo: adicionando um backend WASM

1. Crie uma nova crate `tupa-backend-wasm`.

2. Implemente o trait `CodegenBackend`:

```rust
pub trait CodegenBackend {
    fn emit(&self, ir: &IrModule) -> Result<String, Error>;
}
```

1. Integre no CLI:

```rust
// ...existing code...
let wasm = tupa_backend_wasm::emit(&ir)?;
```

## Links úteis

- [Arquitetura](architecture.md)
- [Codegen](codegen.md)
- [Contribuição](../CONTRIBUTING.md)
