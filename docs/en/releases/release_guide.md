# Release Guide

## Purpose

This document describes the release process with clear, repeatable steps.

## Release cadence

- Weekly cadence: every Thursday.
- A release draft is automatically refreshed by GitHub Actions every Thursday at 13:00 UTC.
- The draft is also refreshed on every push to `main`.

## Step-by-Step

1. Review the current GitHub Release Draft.
2. Update [CHANGELOG](changelog.md) and the documentation set.
3. Run local validation:

```bash
./scripts/ci-local.sh
```

4. Confirm all required CI checks are green on `main`.
5. Verify main examples in `examples/` and bilingual docs.
6. Create the tag and publish:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

7. Create the GitHub release using the generated draft notes.

## Tips

- Use semantic versions (SemVer).
- Avoid releases without updating CHANGELOG and the documentation set.
- Record changes that impact SPEC, API, or core docs.
