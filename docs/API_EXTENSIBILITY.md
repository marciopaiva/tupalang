# API e Extensibilidade do Compilador

## Objetivo
Explicar como usar a API interna do compilador Tupã, estender funcionalidades e adicionar novos backends.

## Uso como biblioteca
Cada crate pode ser usado como biblioteca Rust independente:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Pontos de Extensão
- **Novos tipos**: implemente e registre em `tupa-typecheck`.
- **Novos diagnósticos**: adicione no módulo de erros de cada crate.
- **Novo backend**: crie um crate (ex: `tupa-backend-wasm`) e implemente a trait `CodegenBackend`.
- **CLI customizado**: use `tupa-cli` como base e adicione comandos.

## Exemplo: Adicionando backend WASM
1. Crie um novo crate `tupa-backend-wasm`.
2. Implemente a trait `CodegenBackend`:
```rust
pub trait CodegenBackend {
    fn emit(&self, ir: &IrModule) -> Result<String, Error>;
}
```
3. Integre ao CLI:
```rust
// ...existing code...
let wasm = tupa_backend_wasm::emit(&ir)?;
```

## Links úteis
- [Arquitetura](ARCHITECTURE.md)
- [Codegen](CODEGEN.md)
- [Contribuição](../CONTRIBUTING.md)
