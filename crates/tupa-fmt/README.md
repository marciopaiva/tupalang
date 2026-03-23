# tupa-fmt

Formatter for TupaLang source code.

## Usage

```rust
use tupa_fmt::format_source;

let out = format_source("fn main(){return;}");
println!("{}", out);
```

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade is the main applied integration reference for TupaLang strategy authoring; `tupa-fmt` supports that workflow when formatting `.tp` sources in local development and editor tooling.
