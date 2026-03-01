# Examples Guide

## Purpose

This document defines curation criteria and standards for examples.

## Placement

- Curated examples: `examples/`
- Experiments: `examples/playground/`

## Curation Criteria

- Be small and focused.
- Cover a specific concept.
- Prefer code that passes `check`.
- Avoid external dependencies.

## Standards

- Name files by theme (`match.tp`, `types.tp`).
- Include brief comments only when needed.
- Update `examples/README.md` when adding/removing examples.
- Prefer `Safe<string, ...>` when illustrating ethical constraints.
- Mention new `safe_*` examples in `examples/README.md`.
- Use `safe_misinformation_hate_speech.tp` as the combined-constraints reference.

## Checklist

- [ ] File added under `examples/`
- [ ] Referenced in `examples/README.md`
- [ ] Runs with `tupa-cli -- parse|check`

## Updating goldens

If example outputs change intentionally (for example, formatting improvements), update the golden files in `examples/expected/` using the provided script:

```bash
# Updates all goldens by running the local CLI
bash scripts/update-goldens.sh

# Review changes before committing
git add examples/expected
```

In CI, golden tests will fail if the real output differs from the files in `examples/expected/`.
