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
