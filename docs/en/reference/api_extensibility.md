# Compiler API and Extensibility

## Purpose

This document explains how to use Tupã's compiler API, extend functionality, and embed Tupã in Rust systems.

## Stable Embedding Surface (`v0.8.0-rc`)

The stable embedding surface for this RC cycle is:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`

For minimal embedding examples, see [Embedding](embedding.md).

## Library Usage

Each crate can be used as an independent Rust library:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Extension Points

- **New types**: implement and register in `tupa-typecheck`.
- **New diagnostics**: add them in each crate error module.
- **New backend**: create a crate (for example, `tupa-backend-wasm`) and implement the `CodegenBackend` trait.
- **Custom CLI**: use `tupa-cli` as a base and add commands.

## Example: Adding a WASM Backend

1. Create a new crate `tupa-backend-wasm`.
2. Implement the `CodegenBackend` trait.
3. Integrate it into the CLI.

## Useful Links

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribution](../../CONTRIBUTING.md)
