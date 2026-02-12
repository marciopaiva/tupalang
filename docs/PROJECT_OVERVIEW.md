# Project Overview

## Purpose

Summarize the mission, principles, and project status.

## Mission

Build a Brazilian language for AI governance in critical systems with formal safety, determinism, and predictable performance.

## Principles

- Safety and alignment via types.
- Determinism and auditability by design.
- Native differentiability.
- Declarative sparsity.
- Predictable performance via LLVM.

## Current status

- Specification v0.1 complete.
- Basic lexer, parser, typechecker, and CLI.
- JSON output in the CLI.
- Functional codegen (textual IR).

## Pipeline orchestration example (draft)

```tupa
pipeline FraudTraining {
  data = load_dataset("fraud.csv")
  model = python.train("torch_script.py", data)

  validate(model) {
    constraint accuracy >= 0.95
    constraint no_nan(model)
  }

  audit(hash_for_all: true)
  export("fraud_model_v1.tupamodel")
}
```

## Where to contribute

- Issues for bugs and improvements.
- RFCs with the `[RFC]` prefix.
