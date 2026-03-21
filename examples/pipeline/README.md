# Pipeline Examples

- fraud_complete.tp: fraud pipeline with constraints and validation.
- credit_decision.tp: credit decision with 3 constraints.
- loan_underwriting.tp: underwriting with risk metrics.
- customer_churn.tp: churn and retention metrics.
- config_driven_strategy.tp: typed nested input pattern for host-provided strategy config.

## Run

```bash
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

Generate plan and run from plan:

```bash
tupa codegen --plan-only examples/pipeline/fraud_complete.tp
tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json
```

Config-driven strategy example:

```bash
tupa check examples/pipeline/config_driven_strategy.tp
tupa run \
  --pipeline=ConfigDrivenStrategy \
  --input examples/pipeline/config_driven_strategy.json \
  examples/pipeline/config_driven_strategy.tp
```
