# Release Checklist

## Purpose

This document ensures consistency and quality before publishing a release.

## Checklist

- [ ] Update [CHANGELOG](changelog.md)
- [ ] Update the [Documentation Index](../index.md)
- [ ] Run `cargo test`
- [ ] Run `markdownlint "**/*.md"`
- [ ] Validate main examples in `examples/`
- [ ] Validate CLI (`lex`, `parse`, `check`)
- [ ] Review the [SPEC](../reference/spec.md) for inconsistencies
- [ ] Tag the version in Git (`vX.Y.Z`)
- [ ] Publish the release on GitHub
