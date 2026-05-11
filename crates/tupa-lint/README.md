# tupa-lint

Linter for Tupã policy code (legacy `.tp` and future DSL).

## Purpose

Detects style and quality issues: unused variables, naming conventions, purity violations, and best practice checks.

**Current mode:** Lints legacy `.tp` AST. Future: will lint Rust DSL via attribute macros.

## Usage (Legacy)

```rust
use tupa_lint::lint_program;
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
let warnings = lint_program(&program);
for w in warnings {
    println!("{}", w.message());
}
```

## Future

For Rust DSL, use `cargo tupa lint` (Phase 1) which will analyze `pipeline!` expansions.

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
