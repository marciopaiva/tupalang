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

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade is the applied reference integration for TupaLang pipelines; `tupa-lint` fits into that workflow for pre-runtime quality checks on `.tp` sources.
