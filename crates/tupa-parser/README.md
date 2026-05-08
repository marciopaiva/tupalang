# tupa-parser

Parses TupaLang tokens into an AST.

## Features

- Full grammar including functions, expressions, statements, pipelines, and annotations
- Config DSL support: `config` blocks become `ConfigDecl` AST nodes
- Source span tracking for accurate error reporting

## Usage

```rust
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
println!("{} top-level items", program.items.len());
# Ok::<(), tupa_parser::ParserError>(())
```

## Config DSL Example

```tupa
config TradingConfig {
    max_position_usdt: f64,
    max_daily_loss_pct: f64,
}
```

## Crate

- Depends on `tupa-lexer`
- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-parser` as part of its embedded pipeline compilation path for strategies and analyst diagnostics.
