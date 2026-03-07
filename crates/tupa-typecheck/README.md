# tupa-typecheck

Static checks for TupaLang programs (types, determinism and constraints).

## Usage

```rust
use tupa_parser::parse_program;
use tupa_typecheck::typecheck_program;

let program = parse_program("fn main() {}")?;
typecheck_program(&program)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Crate

- Works with `tupa-parser` AST
- Source: https://github.com/marciopaiva/tupalang