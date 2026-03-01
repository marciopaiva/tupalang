# Release Guide

## Purpose

This document describes the release process with clear, repeatable steps.

## Step-by-Step

1. Update [CHANGELOG](changelog.md) and the documentation set.
2. Run the test suite:

```bash
cargo test
```

1. Verify main examples in `examples/` and bilingual docs.
2. Confirm CI is green.
3. Create the tag and publish:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

1. Create the GitHub release with CHANGELOG notes.

## Tips

- Use semantic versions (SemVer).
- Avoid releases without updating CHANGELOG and the documentation set.
- Record changes that impact SPEC, API, or core docs.
