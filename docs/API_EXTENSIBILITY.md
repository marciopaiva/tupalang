# Compiler API and Extensibility

## Purpose

Explain how to use Tupã's internal compiler API, extend functionality, and add new backends.

## Library usage

Each crate can be used as an independent Rust library:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Extension points

- **New types**: implement and register in `tupa-typecheck`.
- **New diagnostics**: add them in each crate error module.
- **New backend**: create a crate (for example, `tupa-backend-wasm`) and implement the `CodegenBackend` trait.
- **Custom CLI**: use `tupa-cli` as a base and add commands.

## Example: adding a WASM backend

1. Create a new crate `tupa-backend-wasm`.

2. Implement the `CodegenBackend` trait:

```rust
pub trait CodegenBackend {
    fn emit(&self, ir: &IrModule) -> Result<String, Error>;
}
```

1. Integrate it into the CLI:

```rust
// ...existing code...
let wasm = tupa_backend_wasm::emit(&ir)?;
```

# Useful links

- [Architecture](ARCHITECTURE.md)
- [Codegen](CODEGEN.md)
- [Contribution](../CONTRIBUTING.md)
