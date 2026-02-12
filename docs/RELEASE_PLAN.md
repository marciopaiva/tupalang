# Release Plan (0.4.x → 1.0)

## Purpose

Define the release milestones from v0.4.x up to v1.0, aligned with the roadmap and adoption phases.

## References

- [Roadmap](ROADMAP.md)
- [Adoption Plan](ADOPTION_PLAN.md)
- [Versioning Guide](VERSIONING.md)
- [Changelog](CHANGELOG.md)

## Baseline (current)

- v0.4.0 released with functional compiler pipeline (lexer, parser, typechecker, codegen, CLI).
- Diagnostics with spans and JSON output.
- SPEC v0.1 and documentation consolidated.

## Milestones

### 0.5.x — Compiler reliability

- Complete remaining typechecker constraints and validations.
- Improve diagnostics consistency and error clarity.
- Expand test coverage, especially negative cases.

### 0.6.x — Codegen stability and pipeline groundwork

- Basic IR optimizations (dead code elimination, simplifications).
- Stable output for `fn main()` and core examples.
- Initial benchmarks and regression tests.
- Draft pipeline syntax (orchestration, validation, audit hooks).

### 0.7.x — Tooling and orchestration foundation

- Official formatter (`fmt`) with minimal ruleset.
- Minimal linter (`lint`) for style and safety checks.
- CLI stabilization with `build`, `run`, `fmt`, `check`.
- Controlled Python integration for pipeline execution.

### 0.8.x — Developer experience and governance

- Language Server with autocomplete, diagnostics, and go-to-definition.
- Stable JSON diagnostic schema for tooling integrations.
- Audit and hashing primitives for reproducibility.

### 0.9.x — Interoperability

- FFI with C/Rust and documented ABI.
- Minimal bindings for essential libraries and examples.

### 1.0.0 — Quality and trust

- Public, reproducible benchmarks.
- Compatibility policy audited and enforced.
- SPEC finalized with EBNF, validated examples, and normative diagnostics.
- Governance workflows validated for regulated environments.

## Release gates (all versions)

- CHANGELOG updated with user-visible changes.
- Tests and docs lint pass.
- Main examples validated.
- CI green before tagging.
