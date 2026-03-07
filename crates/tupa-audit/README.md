# tupa-audit

Audit helpers for compiler/runtime versioning and deterministic hashing.

## Usage

```rust
use serde_json::json;
use tupa_audit::{compiler_version, hash_execution};
use tupa_parser::parse_program;

let program = parse_program("fn main() {}")?;
let hash = hash_execution(&program, &[json!({"x": 1})]);
println!("{} {}", compiler_version(), hash);
# Ok::<(), Box<dyn std::error::Error>>(())
```
