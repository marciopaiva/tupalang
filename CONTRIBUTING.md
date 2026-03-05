# How to contribute

## Purpose

Explain the contribution workflow and expectations for project changes.

Thank you for considering contributing to the Tupa project.

## Getting started

1. Check [README.md](README.md) for the overview.
2. Open an issue describing the proposal or problem.
3. Fork and create a branch from `main`.

## Contribution standards

- Be clear and objective in problem/solution descriptions.
- Keep changes small and focused.
- Prefer documentation and examples when possible.

## PR workflow

1. Create a branch using the `codex/` prefix for automation branches (for manual branches, use `feat/` or `fix/`).
2. Update/add relevant documentation.
3. Run local checks before opening the PR.
4. Open the PR with context, motivation, and scope.
5. Use a Conventional Commits style title in the PR.
6. Wait for review.

## Commit message convention

Use Conventional Commits format:

`<type>(<scope>): <summary>`

Allowed `type` values:

- `feat`: new functionality
- `fix`: bug fix
- `docs`: documentation-only change
- `refactor`: internal code improvement without behavior change
- `test`: test changes
- `ci`: CI/workflow changes
- `chore`: maintenance tasks

Examples:

- `feat(parser): support typed pipeline attributes`
- `fix(typecheck): reject now() in deterministic pipelines`
- `docs(guides): add local CI usage section`

Rules:

- Generic messages like `ci:` or `docs:` without summary are rejected in CI.
- PR titles must follow the same Conventional pattern.

## Local tests

Before opening a PR, run:

```bash
./scripts/ci-local.sh
```

Optional strict link check:

```bash
CI_LOCAL_STRICT_LINKS=1 ./scripts/ci-local.sh
```

## Documentation style

- Clear, direct English.
- Prefer short sentences.
- Avoid unexplained abbreviations.

## Licensing

By contributing, you agree your contribution will be licensed under the project terms.
