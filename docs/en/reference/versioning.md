# Versioning Guide

## Purpose

This document defines versioning and compatibility policy for language, crates, and binary distribution.

## SemVer

We follow [SemVer](https://semver.org/):

- **MAJOR**: incompatible changes.
- **MINOR**: compatible new features.
- **PATCH**: compatible fixes.

## Pre-1.0

Before 1.0, changes can happen more frequently. We still follow SemVer and document breaking changes in CHANGELOG.

## Release Candidates

Release candidates use the `vX.Y.Z-rc.N` format and must be treated as pre-GA builds.

- RC tags publish release artifacts for validation.
- API guarantees are limited to documented stable surfaces.

## Distribution Model (v0.8.0-rc)

Tupa uses a hybrid model:

- Standalone binary artifacts for end-user adoption.
- Public Rust crates for embedding in Rust systems.

See [Hybrid Distribution Decision](../governance/hybrid_distribution_decision.md).

## Documentation Sync

Any relevant change in docs, examples, or API must be reflected in CHANGELOG and, if applicable, in the documentation index and README.
