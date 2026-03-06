# API del Compilador y Extensibilidad

## Propósito

Explicar cómo usar la API del compilador de Tupã, extender funcionalidad y hacer embedding de Tupã en sistemas Rust.

## Superficie estable de embedding (`v0.8.0-rc`)

La superficie estable de embedding para este ciclo RC es:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

Para ejemplos mínimos, ver [Embedding](embedding.md).

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
- **Nuevos diagnósticos**: añadir en cada módulo de errores de las crates.
- **Nuevo backend**: crear una crate (por ejemplo, `tupa-backend-wasm`) e implementar el trait `CodegenBackend`.
- **CLI personalizado**: usar `tupa-cli` como base y agregar comandos.

## Ejemplo: backend WASM

1. Crear una nueva crate `tupa-backend-wasm`.
2. Implementar el trait `CodegenBackend`.
3. Integrarlo en el CLI.

## Enlaces útiles

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribución](../../CONTRIBUTING.md)
