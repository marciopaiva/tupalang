# Embedding de Tupã en Rust

## Propósito

Describir la superficie soportada de embedding para `v0.8.0-rc`.

## Crates públicas soportadas

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

Estas crates son la superficie estable de embedding para este ciclo RC.

## Ejemplo mínimo

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;

fn main() -> anyhow::Result<()> {
    let src = "fn main() { print(1) }";
    let ast = parse(src)?;
    let _typed = typecheck(&ast)?;
    Ok(())
}
```

## Notas de compatibilidad

- Sigue SemVer según [Versionado](versioning.md).
- Evita depender de crates internas no listadas arriba si necesitas estabilidad de API.
