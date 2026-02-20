# Tupã v0.7.0 — Hybrid Engine

## ✨ Novidades
- pipeline { ... } — blocos com garantias formais de determinismo
- Effect System — rastreamento de IO/Random/Time em tempo de compilação
- Backend híbrido — LLVM para APIs + JSON para pipelines
- Runtime de pipelines com relatório de métricas e constraints
- Audit integrado: hash e fingerprint do AST

## 🛠️ Como usar
```bash
tupa new my-audit-pipeline
cd my-audit-pipeline
tupa run --pipeline=FraudDetection --input=tx.json
```

## 📚 Links
- Guia de pipelines: docs/PIPELINE_GUIDE.md
- Esquema do ExecutionPlan: docs/EXECUTION_PLAN_SCHEMA.md
- Backend híbrido e codegen: docs/CODEGEN.md
- Sistema de efeitos: docs/EFFECT_SYSTEM.md

## 📊 Métricas de Sucesso (alvo)
- Pipelines válidos compilam: 100%
- Pipelines não-determinísticos rejeitados: 100%
- Funções gerais continuam funcionando: 100%
- Tempo de compilação (exemplo médio): < 200ms
- Documentação com exemplo executável: 1 guia completo
- Stars no GitHub pós-release: +15

## ⚠️ Riscos & Mitigações
- Effect system lento — Cache de análise por nó AST
- pipeline confunde devs com fn — Docs claras + warning educacional no CLI
- Backend híbrido complexo — Fallback: JSON na v0.7.0, LLVM v0.8.0
- Adoção baixa — Case study real (parceria fintech BR)

## Notas Técnicas
- ExecutionPlan JSON versão 1 com `steps`, `constraints`, `metrics`, `metric_plans`.
- Seed opcional no plano, propagada para PRNG determinístico no runtime.
- Validação de input JSON contra TypeSchema antes de executar.
