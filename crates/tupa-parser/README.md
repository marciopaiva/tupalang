# tupa-parser

Parses TupaLang tokens into an AST.

## Usage

```rust
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
println!("{} top-level items", program.items.len());
# Ok::<(), tupa_parser::ParserError>(())
```

## Crate

- Depends on `tupa-lexer`
- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-parser` as part of its embedded pipeline compilation path for strategies and analyst diagnostics.
