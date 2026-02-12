# Issues Guide

## Purpose

Standardize issue creation with useful triage information.

## When to open an issue

- Bugs and unexpected errors.
- Improvement proposals (use `[RFC]`).
- Questions about spec or behavior.

## Checklist

- [ ] Clear and specific title.
- [ ] Steps to reproduce (if bug).
- [ ] Expected vs actual result.
- [ ] Relevant logs/prints.
- [ ] Rust and project version.

## Example (bug)

**Title**: `Parser fails with nested match`

**Description**:

- Steps: `tupa-cli -- parse examples/match.tp`
- Expected: valid AST
- Actual: `unexpected token` error

## Example (RFC)

**Title**: `[RFC] Optional types`

**Description**:

- Motivation
- Alternatives
- Impact on SPEC
