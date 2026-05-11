# Examples

⚠️ **These examples use the legacy `.tp` file format.** New development should use the **Rust DSL** (`tupa-core` crate). See [Getting Started](../docs/en/guides/getting_started.md) for the modern approach.

## Purpose

Curated examples reflecting the current state of the parser, typechecker, and codegen (legacy path). These examples are **deprecated** and will be removed after 2027.

For up-to-date examples, see the `tupa-core` crate documentation and ViperTrade repository.

## Curation and playground

- Use this folder for curated and stable examples.
- Use [examples/playground](playground/README.md) for quick tests and experiments.

## Files

### General

... (keep existing list)

### Pipeline Examples

The `pipeline/` subdirectory contains realistic trading strategy pipelines demonstrating various constraints and governance features:

- `pipeline/minimal.tp`: simplest valid pipeline.
- `pipeline/fraud_complete.tp`: comprehensive fraud detection pipeline (used for valid golden).
- `pipeline/credit_decision.tp`: credit decision pipeline with state.
- `pipeline/deterministic_violation.tp`: intentionally non-deterministic (fails `@deterministic`).
- `pipeline/now_violation.tp`: uses `now` in a prohibited context (fails temporal policy).
- `pipeline/time_violation.tp`: violates time-based constraints.
- `pipeline/undefined_metric.tp`: references an undefined metric.
- `pipeline/config_driven_strategy.tp`: demonstrates **Config DSL** — declarative configuration blocks passed to pipeline steps.
- ... and others.

### Config DSL

The Config DSL allows declaring typed configuration blocks that become first-class AST nodes. Example from `pipeline/config_driven_strategy.tp`:

```tupa
config StrategyConfig {
    type threshold: f64
    type window: i64
}

step compute_signal {
    input: StrategyConfig
    // use config fields: threshold, window
}
```text

This provides strongly-typed pre-conditions for pipelines, improving safety and documentation.

### Plugin System

Tupã supports dynamic loading of step functions via the **Plugin System** (`tupa-plugin` crate). Plugins are shared libraries written in Rust that export C entry points. See the `tupa-plugin` crate README for a full example: `crates/tupa-plugin/README.md`.

Applied reference: [ViperTrade](https://github.com/marciopaiva/vipertrade) uses plugins to extend step functions.

### Negative cases (should fail)

... (keep rest)
