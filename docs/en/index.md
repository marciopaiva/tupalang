# Tupã Documentation

> **New:** Tupã is now a set of **Rust crates** (`tupa-core`, `tupa-engine`). Learn more in the [Proposal](PROPOSAL.md).

## Quick Links

- [📦 Getting Started](guides/getting_started.md) — install and write your first pipeline in Rust
- [📘 SPEC (normative)](reference/spec.md) — formal definition of the language/DSL
- [🚀 Crate Documentation](https://crates.io/crates/tupa-core) — API reference on crates.io
- [🔗 Transition Guide](TRANSITION.md) — migrating from legacy `.tp` files
- [📋 Adoption Plan](governance/adoption_plan.md) — delivery milestones to v1.0.0
- [🗺️ Roadmap](releases/roadmap.md) — timeline and priorities
- [💡 Why Rust Crates?](PROPOSAL.md) — strategic rationale

---

## For Rust Developers

Start here:

1. `cargo add tupa-core tupa-engine`
2. Write a `pipeline! { ... }` block in your Rust code
3. `cargo build` — type-checked and ready

Detailed:

- [Installation & Setup](guides/installation.md) — `cargo add` and minimal example
- [Pipeline Guide](guides/pipeline_guide.md) — building complete pipelines
- [Safe Types](../reference/type_semantics.md) — `Safe<T, !constraint>` semantics
- [Constraints](reference/effect_system.md) — compile-time proofs
- [Error Diagnostics](reference/diagnostics_checklist.md) — understanding error codes

---

## For ViperTrade Users

- [ViperTrade Integration Guide](guides/pipeline_guide.md) — using Tupã inside ViperTrade
- [Audit & Hashing](guides/testing.md) — reproducibility with `tupa-audit`
- [Examples](https://github.com/marciopaiva/vipertrade) — real-world pipelines

ViperTrade uses Tupã as its strategy policy layer. New strategies should use Rust DSL; legacy `.tp` strategies are supported until 2027.

---

## Reference Materials

### Normative Specifications (must-read for implementers)

- [Language SPEC (v0.1)](reference/spec.md) — full normative spec
- [EBNF Grammar](../reference/grammar.ebnf) — machine-readable syntax (legacy `.tp` format)
- [Type Semantics](../reference/type_semantics.md) — formal type rules, inference, subtyping
- [Conformance Suite](https://github.com/marciopaiva/tupalang/tree/main/crates/tupa-conformance) — test oracle

### Crate API Reference

- [`tupa-core`](https://crates.io/crates/tupa-core) — DSL macros, types, constraints
- [`tupa-engine`](https://crates.io/crates/tupa-engine) — executor runtime
- [`tupa-fmt`](https://crates.io/crates/tupa-fmt) — formatter for legacy `.tp`
- [`tupa-lint`](https://crates.io/crates/tupa-lint) — linter for policy code
- [`tupa-audit`](https://crates.io/crates/tupa-audit) — audit hashing
- [`tupa-plugin`](https://crates.io/crates/tupa-plugin) — dynamic plugin loading

---

## Legacy `.tp` Documentation (Deprecated)

During the transition period, some legacy docs are still accessible:

- [Installation (legacy CLI)](guides/installation.md#legacy-cli) — standalone binary for existing `.tp` pipelines

All other `.tp`--centric guides have been removed. New code should use Rust DSL.

---

## Community & Support

- GitHub Issues: [marciopaiva/tupalang/issues](https://github.com/marciopaiva/tupalang/issues)
- Discussions: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- Applied reference: [ViperTrade](https://github.com/marciopaiva/vipertrade)

---

## Status

**Active development.** Targeting v1.0.0 release in 2026-Q4 (crate-first architecture). See [roadmap](releases/roadmap.md) for details.

---

*"Força ancestral, código moderno" — Brazilian craft meeting Rust safety.*
