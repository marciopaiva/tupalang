# Embedding Tupã in Rust

## Purpose

Describe the supported embedding surface for `v0.8.0-rc`.

## Supported Public Crates

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

These crates are the stable embedding surface for this RC cycle.

## Minimal Example

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

## Compatibility Notes

- Follow SemVer constraints from [Versioning](versioning.md).
- Avoid depending on internal crates not listed above if you need API stability.
