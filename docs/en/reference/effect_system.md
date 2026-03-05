# Effect System

## Purpose

This document explains how the compiler classifies effects and how they are enforced in pipelines.

## Supported Effects

- IO (for example, `print`)
- Random (for example, `random`)
- Time (for example, `time`, `now`)
- Pure utility (for example, `hash`)

## Pipeline Rules

- `@deterministic` rejects Random and Time effects in steps.
- `hash(...)` is treated as pure and is allowed in deterministic pipelines.
- `now()`/`time()` are treated as Time effects and are rejected under `@deterministic`.
- Diagnostic: E2005 (impure in deterministic pipeline).
