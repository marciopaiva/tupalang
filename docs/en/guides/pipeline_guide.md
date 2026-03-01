# Pipeline Guide

## Purpose

Run a Tupã pipeline end-to-end: generate an ExecutionPlan and execute it with JSON input.

## Steps

- Generate the plan:
  - `tupa codegen --format=json examples/pipeline/fraud_complete.tp`
- Run the pipeline:
  - `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp`

### Plan-only

- `tupa codegen --plan-only examples/pipeline/fraud_complete.tp`
- `tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json`

### Persist Output

- `tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json --output out.json examples/pipeline/fraud_complete.tp`

## ExecutionPlan Structure

- name, version, seed (optional), input_schema
- steps: name, function_ref, effects
- constraints: metric, comparator, threshold
- metrics: literal values captured from the validation block
- metric_plans: { name, function_ref, args } to compute metrics at runtime

## Notes

- function_ref format: `<file>::step_<name>`.
- Effects (random/time) are identified by the typechecker.
- The runtime evaluates constraints and emits a JSON report with metrics/constraints.
