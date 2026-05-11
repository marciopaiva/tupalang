# tupa-conformance

Conformance test runner — validates Tupã implementation against SPEC.

## Purpose

Runs the SPEC-mandated test suite. All Tupã implementations (legacy compiler and new crate-based) must pass this suite.

## Usage

```bash
cargo run -p tupa-conformance
```text

JSON report is printed to stdout. Exit code: 0 if all pass, 1 otherwise.

## Scope

This suite tests **SPEC compliance for the legacy `.tp` DSL** — parser and typechecker behavior.
The new Rust-first architecture (`tupa-core`, `tupa-engine`, `pipeline!` macro) is tested separately via unit and integration tests in those crates, and through the example programs in `tupa-engine/examples/`.

**This crate is frozen at 0.8.x and will be deprecated in a future release** once the Rust DSL achieves feature parity with the legacy SPEC. Until then, it serves as an oracle for legacy behavior validation during the transition period.

## Crate

- Binary name: `tupa-conformance`
- Source: [tupalang](https://github.com/marciopaiva/tupalang)
- License: Apache-2.0
