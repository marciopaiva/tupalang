# Effect System

## Purpose

This document explains how the compiler classifies effects and how they are enforced in pipelines.

## Supported Effects

- IO (for example, `print`)
- Random (for example, `random`)
- Time (for example, `time`, `now`)

## Pipeline Rules

- `@deterministic` rejects Random and Time effects in steps.
- Diagnostic: E2005 (impure in deterministic pipeline).
