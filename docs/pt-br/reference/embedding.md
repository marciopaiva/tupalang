# Embedding de Tupã em Rust

## Propósito

Descrever a superfície suportada de embedding para `v0.8.0-rc`.

## Crates públicas suportadas

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

Essas crates são a superfície estável de embedding para este ciclo RC.

## Exemplo mínimo

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

## Notas de compatibilidade

- Siga SemVer conforme [Versionamento](versioning.md).
- Evite depender de crates internas não listadas acima se você precisa de estabilidade de API.
