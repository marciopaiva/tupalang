# tupa-lexer

Tokenizes TupaLang source code.

## Usage

```rust
use tupa_lexer::lex_with_spans;

let tokens = lex_with_spans("fn main() {}")?;
println!("{} tokens", tokens.len());
# Ok::<(), tupa_lexer::LexerError>(())
```

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade depends on the parser/typecheck/runtime stack built on top of `tupa-lexer` for embedded strategy and diagnostics pipelines.
