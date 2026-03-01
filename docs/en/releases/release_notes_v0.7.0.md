# Tupã v0.7.0 — Hybrid Engine

## Highlights

- `pipeline { ... }` — blocks with formal determinism guarantees
- Effect System — compile-time tracking of IO/Random/Time
- Hybrid backend — LLVM for APIs + JSON for pipelines
- Pipeline runtime with metric and constraint reports
- Integrated audit: hash and AST fingerprint

## How to Use

```bash
tupa new my-audit-pipeline
cd my-audit-pipeline
tupa run --pipeline=FraudDetection --input=tx.json
```

## Links

- Pipeline guide: [Pipeline Guide](../guides/pipeline_guide.md)
- ExecutionPlan schema: [ExecutionPlan Schema](../reference/execution_plan_schema.md)
- Hybrid backend and codegen: [Codegen](../reference/codegen.md)
- Effect system: [Effect System](../reference/effect_system.md)

## Success Metrics (Targets)

- Valid pipelines compile: 100%
- Non-deterministic pipelines rejected: 100%
- General functions keep working: 100%
- Compile time (medium example): < 200ms
- Docs with executable example: 1 complete guide
- GitHub stars post-release: +15

## Risks and Mitigations

- Slow effect system — cache analysis per AST node
- Pipelines confused with fn — clear docs + educational CLI warning
- Hybrid backend complexity — fallback: JSON in v0.7.0, LLVM in v0.8.0
- Low adoption — real case study (BR fintech partnership)

## Technical Notes

- ExecutionPlan JSON v1 with `steps`, `constraints`, `metrics`, `metric_plans`.
- Optional plan seed propagated to deterministic PRNG in runtime.
- Input JSON validated against TypeSchema before execution.
