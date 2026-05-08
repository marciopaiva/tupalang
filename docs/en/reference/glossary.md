# Glossary

## Purpose

This document defines key terms used in the language and documentation.

## Terms

- **Alignment**: set of ethical constraints checked at compile time.
- **Config DSL**: declarative configuration blocks (`config Name { type field, ... }`) as first-class AST nodes for pipeline pre-conditions.
- **Constraint**: condition that must be proven for a `Safe<T, ...>` type (for example `!nan`, `!hate_speech`).
- **Extension**: custom step functions registered via the `TupaExtension` trait or plugin system.
- **Hot Reload**: automatic pipeline reload on file change via `notify`-based file watching.
- **Built-in Function**: predefined helpers in the `tupa::` namespace (`weighted`, `warn`, `pass`, `confirm`, `cooldown`).
- **Density**: type parameter that controls tensor sparsity.
- **EBNF**: formal notation for language grammar.
- **Nabla (`∇`)**: native differentiability operator.
- **Plugin**: dynamically loaded shared library (`.so`/`.dll`) that exports step functions via C entry points.
- **Safe Type**: type annotated with constraints, for example `Safe<f64, !nan>` or `Safe<string, !misinformation>`.
- **Schema Registry**: versioned schema storage with migration support for evolving pipeline inputs/outputs.
- **Span**: text interval used to point to errors (line/column).
- **Typechecker**: compiler static type checker.
