# Roadmap

## Purpose

This document summarizes the project evolution plan.

## Short Term

- Use [`v0.8.2 Checklist`](checklist_v0.8.2.md) as the planning anchor for the next release line.
- Consolidate the `v0.8.1` release learnings into examples and docs.
- Refine reusable predicate ergonomics on top of the new policy-modeling primitives.
- Decide whether any structured policy breakdowns should become stable public contracts.

- Consolidate SPEC v0.1 (fine-tuning and validated examples).
- Improve the typechecker (constraints and diagnostics).
- Stabilize codegen textual IR and CLI outputs.
- Expand safe examples and negative goldens.

## Mid Term

- MVP pipeline language with deterministic execution.
- Audit and hashing primitives for reproducibility.
- Controlled, auditable Python integration (PyTorch/TensorFlow).
- Official formatter and minimal linter.
- Basic language server.

## Long Term

- FFI with C/Rust.
- Documented ABI.
- Public benchmarks.
- Enterprise-grade tooling and compliance workflows.
