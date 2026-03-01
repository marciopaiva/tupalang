# Minimum Technical Adoption Plan

## Purpose

This document defines an incremental path to make the language usable and reliable, without committing to dates.

## Index

- [Phase 0: Minimal core](#phase-0-minimal-core)
- [Phase 1: Basic toolchain](#phase-1-basic-toolchain)
- [Phase 2: Developer experience](#phase-2-developer-experience)
- [Phase 3: Interoperability](#phase-3-interoperability)
- [Phase 4: Quality and trust](#phase-4-quality-and-trust)
- [Minimum deliverables](#minimum-deliverables)

## Phase 0: Minimal core

- Define the core subset (syntax and basic types).
- Minimal formal specification (EBNF + type semantics).
- Conformance test suite (parser + typechecker).
- Diagnostics output consumable by tools (JSON).

## Phase 1: Basic toolchain

- Official formatter.
- Linter with minimal rules.
- Language server (autocomplete, diagnostics, go-to-definition).

## Phase 2: Developer experience

- Project templates (CLI and service).
- Stable CLI with `build`, `run`, `fmt`, `check`.
- Didactic and consistent error messages.

## Phase 3: Interoperability

- FFI with C/Rust.
- Documented ABI.
- Minimal bindings for essential libraries.

## Phase 4: Quality and trust

- Public, reproducible benchmarks.
- Regression tests for performance.
- Versioning and compatibility policy.

## Minimum deliverables

- SPEC with EBNF and type rules.
- Automated parser/typechecker tests.
- Functional CLI with simple examples and `--format pretty|json`.
