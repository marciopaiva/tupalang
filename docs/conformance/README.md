# Conformance Test Suite

## Purpose

The Tupã Conformance Test Suite verifies that the implementation adheres to the language specification (Phase 0: Minimal core). It is the primary validation tool for compiler correctness.

## Scope

The suite tests:

- **Parser**: syntactic validity of source files (valid programs must parse; invalid programs must fail)
- **Type checker**: semantic validation (well-typed programs must typecheck; ill-typed programs must fail)

## Running the Suite

### As a binary

```bash
cd tupalang
cargo run -p tupa-conformance
```text

The runner produces a JSON report on stdout:

```json
{
  "total": 29,
  "passed": 29,
  "failed": 0,
  "results": [
    {
      "id": "parser_valid_hello",
      "passed": true,
      "error": null
    },
    ...
  ]
}
```text

Exit code is `0` if all tests pass, `1` otherwise.

### As a library

You can embed the runner in other tools by calling `tupa_conformance::runner::run()`, which returns `Result<()>`.

## Test Manifest

Tests are defined in `crates/tupa-conformance/data/manifest.json`. Each entry contains:

| Field | Meaning |
|-------|---------|
| `id` | Unique identifier |
| `file` | Path to the `.tp` source file relative to the workspace root |
| `stage` | `"parse"` or `"typecheck"` |
| `expect` | `"ok"` (should succeed) or `"err"` (should fail) |
| `description` | Human-readable summary |

Adding a new test:

1. Place the `.tp` file under `examples/` (or another location within the workspace).
2. Add an entry to the manifest.
3. Run `cargo run -p tupa-conformance` to verify.

## Current Coverage

- **Parser**: basic programs, functions, types, control flow, patterns, enums, arrays, pipelines, syntax errors
- **Typechecker**: primitive types, composite types, generics, pattern matching exhaustiveness, Safe constraints, binary/unary operators

## Artifacts

- **Machine-readable grammar**: `docs/reference/grammar.ebnf` (EBNF)
- **Type semantics**: `docs/reference/type_semantics.md`
- **Specification**: `docs/reference/spec.md` (normative)

## Integration with CI

The conformance suite runs as part of CI validation. Add to your CI pipeline:

```yaml
- name: Conformance tests
  run: cargo run -p tupa-conformance
```text
