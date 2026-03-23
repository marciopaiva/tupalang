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
- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-typecheck` to validate strategy and diagnostics pipelines before code generation and embedded runtime execution.
