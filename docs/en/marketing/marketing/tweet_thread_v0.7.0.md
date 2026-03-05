# Thread: Tupã v0.7.0 — Hybrid Engine

1/ Tupã v0.7.0 is here. The first language with native AI governance.

2/ ✨ Highlights

- `pipeline { ... }` with formal determinism guarantees
- Effect System: IO/Random/Time tracked at compile time
- Hybrid backend: LLVM for APIs + JSON for pipelines

3/ 🛡️ Who is it for?

- ML engineers with production audit needs
- Fintech/healthtech developers with strict compliance
- Anyone who wants governance without sacrificing performance

4/ 🚀 Get started

```bash
tupa new my-audit-pipeline
cd my-audit-pipeline
tupa run --pipeline=FraudDetection --input=tx.json
```

5/ 📚 Docs

- [Pipeline Guide](../../guides/pipeline_guide.md)
- [SPEC](../../reference/spec.md)
- [Glossary](../../reference/glossary.md)

6/ 🎥 Executable demo

- Gist: [tupa-v0.7.0-demo](https://github.com/marciopaiva/tupalang/blob/main/examples/pipeline/fraud_complete.tp)

7/ 🌩️ Thesis
Not “another ML language”. It’s the language for regulated AI.

tupalang #AIgovernance #Rust #compilers
