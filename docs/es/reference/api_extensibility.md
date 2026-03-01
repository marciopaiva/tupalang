# API del Compilador y Extensibilidad

## Propósito

Explicar cómo usar la API interna del compilador Tupã, extender funcionalidad y agregar nuevos backends.

## Uso como biblioteca

Cada crate puede usarse como biblioteca Rust independiente:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Puntos de extensión

- **Nuevos tipos**: implementar y registrar en `tupa-typecheck`.
- **Nuevos diagnósticos**: añadirlos en cada módulo de errores de las crates.
- **Nuevo backend**: crear una crate (por ejemplo, `tupa-backend-wasm`) e implementar el trait `CodegenBackend`.
- **CLI personalizado**: usar `tupa-cli` como base y agregar comandos.

## Ejemplo: agregando un backend WASM

1. Crea una nueva crate `tupa-backend-wasm`.

2. Implementa el trait `CodegenBackend`:

```rust
pub trait CodegenBackend {
    fn emit(&self, ir: &IrModule) -> Result<String, Error>;
}
```

1. Intégralo en el CLI:

```rust
// ...existing code...
let wasm = tupa_backend_wasm::emit(&ir)?;
```

## Enlaces útiles

- [Arquitectura](../overview/architecture.md)
- [Codegen](codegen.md)
- [Contribución](../../CONTRIBUTING.md)
