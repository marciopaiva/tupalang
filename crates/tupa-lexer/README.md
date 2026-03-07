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

- Source: https://github.com/marciopaiva/tupalang
- License: Apache-2.0