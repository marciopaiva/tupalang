# tupa-audit

Audit helpers for deterministic hashing and reproducibility.

## Purpose

Generates reproducible hashes of pipeline ASTs and execution inputs for audit trails and compliance verification.

**Compatibility:** Works with legacy `.tp` AST. Migration to `tupa-core` DSL planned.

## Usage (Legacy)

```rust
use serde_json::json;
use tupa_audit::{compiler_version, hash_execution};
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
let hash = hash_execution(&program, &[json!({"x": 1})]);
println!("{} {}", compiler_version(), hash);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Future (crate-first)

Will provide `AuditLogger` integration with `tupa-engine` executor.

## Crate

- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
