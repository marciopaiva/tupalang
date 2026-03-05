# Release Guide

## Purpose

This document describes the release process with clear, repeatable steps.

## Release cadence

- Weekly cadence: every Thursday.
- A release draft is automatically refreshed by GitHub Actions every Thursday at 13:00 UTC.
- The draft is also refreshed on every push to `main`.

## Changelog source of truth

- GitHub Release Draft is the source of truth for release notes composition.
- PR labels (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`, `breaking`) drive section grouping.
- Before tagging, reconcile draft notes with [CHANGELOG](changelog.md).

## Step-by-Step

1. Review the current GitHub Release Draft.
2. Review merged PR labels and fix mislabeled items (if any).
3. Update [CHANGELOG](changelog.md) from the draft notes.
4. Run local validation:

```bash
./scripts/ci-local.sh
```

5. Run release verification gate:

```bash
./scripts/release-verify.sh X.Y.Z
```

6. Confirm all required CI checks are green on `main`.
7. Verify main examples in `examples/` and multilingual docs.
8. Create the tag and publish:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

9. Create the GitHub release using the generated draft notes.

## Go/No-Go (RC and Final)

Go only when all conditions below are true:

- `./scripts/release-verify.sh X.Y.Z` passes.
- ViperTrade-like smoke passes (`./scripts/vipertrade-smoke.sh`).
- Required checks are green on `main`.
- Changelog for `X.Y.Z` is aligned in EN/ES/PT-BR.
- No open blocker issue for the release milestone.

No-Go conditions:

- Any CI failure in required checks.
- Docs parity mismatch or missing required changelog sections.
- Smoke test reports status different from `pass`.

## Quick checklist

See [Release Cut Checklist](release_cut_checklist.md).

## Tips

- Use semantic versions (SemVer).
- Avoid releases without updating CHANGELOG and the documentation set.
- Record changes that impact SPEC, API, or core docs.
