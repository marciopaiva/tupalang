# Issues Guide

## Purpose

This document standardizes issue creation with useful triage information.

## When to Open an Issue

- Bugs and unexpected errors.
- Improvement proposals (use `[RFC]`).
- Questions about SPEC or behavior.

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
