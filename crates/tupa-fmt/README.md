# tupa-fmt

Formatter for TupaLang source code.

## Usage

```rust
use tupa_fmt::format_source;

let out = format_source("fn main(){return;}");
println!("{}", out);
```