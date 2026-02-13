
# Architecture

## Purpose

Explain the repository organization and the compiler main flow.

## Overview

The project is a Rust workspace with multiple crates implementing compiler stages.

## Folder structure

- `crates/tupa-lexer`: source code tokenization.
- `crates/tupa-parser`: AST construction.
- `crates/tupa-typecheck`: type checking and constraints, including anonymous functions (lambdas) and function values.
- `crates/tupa-codegen`: functional IR generation (LLVM-like), supporting functions, lambdas, print, string concatenation, arrays, control flow, and more.
- `crates/tupa-audit`: deterministic audit hashing for AST + inputs.
- `crates/tupa-cli`: command-line interface, integration of all stages, and golden test execution.
- `docs/`: product documentation and specification.
- `examples/`: runnable examples and golden tests.

## Main flow

1) **Lexer**: converts text into tokens.
2) **Parser**: turns tokens into AST.
3) **Typechecker**: validates types, constraints, functions/lambdas, and function values.
4) **Codegen**: generates functional IR (LLVM-like) covering MVP features.
5) **Audit**: generates deterministic hashes from AST and inputs.
6) **CLI**: integrates stages, exposes commands (`lex`, `parse`, `check`, `codegen`, `audit`), and runs automated golden tests.

## Crate dependencies

- `tupa-parser` depends on `tupa-lexer`.
- `tupa-typecheck` depends on `tupa-parser`.
- `tupa-codegen` depends on `tupa-parser` and `tupa-typecheck`.
- `tupa-audit` depends on `tupa-parser`.
- `tupa-cli` depends on all.

## Notes

- Diagnostics follow spans and normalized errors per stage.
- CLI JSON output enables tool integration.
- Golden tests ensure pipeline stability.
- See typechecker details in [TYPECHECKER_DETAILS.md](TYPECHECKER_DETAILS.md).
