# Pipeline Examples

- fraud_complete.tp: fraud pipeline with constraints and validation.
- credit_decision.tp: credit decision with 3 constraints.
- loan_underwriting.tp: underwriting with risk metrics.
- customer_churn.tp: churn and retention metrics.

## Run

```
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

Generate plan and run from plan:

```
tupa codegen --plan-only examples/pipeline/fraud_complete.tp
tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json
```
