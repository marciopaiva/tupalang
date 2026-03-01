# Release Plan (0.4.x → 1.0)

## Purpose

This document defines the release milestones from v0.4.x up to v1.0, aligned with the roadmap and adoption phases.

## References

- [Roadmap](roadmap.md)
- [Adoption Plan](../governance/adoption_plan.md)
- [Versioning Guide](../reference/versioning.md)
- [Changelog](changelog.md)

## Baseline (current)

- v0.6.0 released with enum generics, match destructuring/guards, and audit prototype.
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

#### 0.6.0 — Strategic plan

**Core theme**: State machines with formal guarantees.

##### Technical priorities

1. Enums with ethical constraints (parser/typechecker)
   - EBNF syntax for enums with generics.
   - Variant type inference.
   - Constraint propagation through variants (`Safe<T>` inside `Enum<Safe<T>>`).
   - Clear errors when constraints are violated inside `match`.
   - Status: done
2. Pattern matching with full destructuring
   - Tuple destructuring inside patterns.
   - Guards with access to bindings.
   - Exhaustiveness checking.
   - Precise span for uncovered pattern.
   - Status: done
3. Audit engine v0.1 (deterministic prototype)
   - CLI `tupa audit` with JSON output (hash + AST fingerprint).
   - Reproducibility: same input → same hash across machines.
   - Documentation in `docs/en/governance/audit_engine.md`.
   - Status: done
4. Diagnostics with actionable suggestions
   - Specific error code for unproven constraints.
   - Contextual suggestions with safety attributes.
   - Links to constraint documentation.

##### Acceptance criteria

- Enum generics parse and typecheck with correct inference. (done)
- Safe constraints preserved through enum variants and `match` arms. (done)
- Non-exhaustive matches are rejected with actionable spans. (done)
- `tupa audit` JSON output includes SHA3-256 hash and AST fingerprint.
- Audit output is stable across two independent runs.
- Diagnostics include a help hint when a safety proof is missing.
- `examples/audit/fraud_pipeline.tp` compiles only with valid `@safety`.

##### Weekly roadmap

- Week 1: Enums + generics in parser/typechecker.
- Week 1: Enums + generics in parser/typechecker, EBNF updated, parsing tests.
- Week 2: Constraint propagation in enums, 15+ tests with `Safe<T>` in variants.
- Week 3: Exhaustiveness + destructuring in match, negative tests.
- Week 4: Audit engine prototype + CLI, `tupa audit` command, initial docs.
- Week 5: Diagnostics refinement with suggestions, golden tests.
- Week 6: RC + docs, CHANGELOG, real examples in `examples/audit/`.

##### Out of scope

- Full LLVM backend.
- Python FFI.
- `∇` operator.
- Async/await.

##### Success metric

- A credit-decision pipeline with approve/review/reject states compiles with a formal safety proof in under 50 lines.

### 0.7.x — Tooling and orchestration foundation

- Official formatter (`fmt`) with minimal ruleset.
- Minimal linter (`lint`) for style and safety checks.
- CLI stabilization with `build`, `run`, `fmt`, `check`.

### 0.8.x — Controlled Python integration and auditability

- PyTorch/TensorFlow execution via controlled adapters.
- Traceable Python calls with validation hooks.
- Audit log schema for external execution (Python integrations).

### 0.9.x — Interoperability

- FFI with C/Rust and documented ABI.
- Minimal bindings for essential libraries and examples.
- Language Server with autocomplete, diagnostics, and go-to-definition.

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
