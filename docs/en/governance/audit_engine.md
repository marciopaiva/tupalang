# Audit Engine

## Purpose

This document describes the deterministic audit hash used to fingerprint executions.

## Inputs

The audit hash combines:

- Normalized AST (stable field order, no spans)
- Canonical JSON inputs (sorted object keys)
- Compiler version string

## Output

The hash output is a SHA3-256 hex string. The CLI returns:

- `hash`: execution hash (AST + inputs + version)
- `ast_fingerprint`: AST-only hash (AST + version)
- `compiler_version`: compiler version used for hashing

## Example

Source:

```tupa
fn main() {
  let x = 1;
  print(x);
}
```

Inputs:

```json
[
  1,
  "ok",
  {
    "b": 2,
    "a": 1
  }
]
```

CLI:

```bash
cargo run -p tupa-cli -- audit examples/audit_hello.tp --input examples/audit_inputs.json
cargo run -p tupa-cli -- audit --format json examples/audit_hello.tp --input examples/audit_inputs.json
```

## Acceptance Criteria

```bash
tupa audit examples/pipeline.tp --input=data.json
```

## Library API

```rust
use serde_json::Value;
use tupa_audit::hash_execution;
use tupa_parser::parse_program;

let program = parse_program("fn main() { let x = 1; }").unwrap();
let inputs = vec![Value::from(1)];
let hash = hash_execution(&program, &inputs);
println!("{hash}");
```

## Determinism

Given the same source, compiler version, and inputs, the hash is stable across machines.
