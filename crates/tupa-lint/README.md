# tupa-lint

Linter for TupaLang AST programs.

## Usage

```rust
use tupa_lint::lint_program;
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
let warnings = lint_program(&program);
println!("{} warnings", warnings.len());
# Ok::<(), Box<dyn std::error::Error>>(())
```

