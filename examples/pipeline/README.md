# Exemplos de Pipeline

- fraud_complete.tp: pipeline de fraude com constraints e validation.
- credit_decision.tp: decisão de crédito com 3 constraints.
- loan_underwriting.tp: underwriting com métricas de risco.
- customer_churn.tp: métricas de churn e retenção.

## Executar

```
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

Gerar apenas o plano e executar via plano:

```
tupa codegen --plan-only examples/pipeline/fraud_complete.tp
tupa run --plan fraud_complete.plan.json --pipeline=FraudDetection --input examples/pipeline/tx.json
```
