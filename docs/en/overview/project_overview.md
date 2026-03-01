# Project Overview

## Purpose

This document summarizes the mission, principles, and project status.

## Mission

Build a Brazilian-designed language for AI governance in critical systems, with formal safety, determinism, and predictable performance.

## Principles

- Safety and alignment via types.
- Determinism and auditability by design.
- Integrate without losing governance — every Python call is traced, validated, and auditable.
- Native differentiability.
- Declarative sparsity.
- Predictable performance via LLVM.

## Current Status

- Specification v0.1 complete.
- Basic lexer, parser, typechecker, and CLI.
- JSON output in the CLI.
- Functional codegen (textual IR).

## Pipeline Orchestration Example (Draft)

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

## Where to Contribute

- Issues for bugs and improvements.
- RFCs with the `[RFC]` prefix.
