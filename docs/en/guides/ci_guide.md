# CI Guide

## Purpose

This document describes CI workflows and their triggers.

## Workflows

### CI (tests)

File: `.github/workflows/ci.yml`

- Runs on `push` and `pull_request`.
- Runs tests per crate:
  - `tupa-lexer`
  - `tupa-parser`
  - `tupa-typecheck`
  - `tupa-cli`

### Docs Lint

File: `.github/workflows/docs-lint.yml`

- Runs on `push` and `pull_request` when `.md` files change.
- Runs two jobs:
  - `markdownlint` (formatting)
  - `link-check` (link verification)

### Wiki Sync

File: `.github/workflows/wiki-sync.yml`

- Runs on `push` to `main` when docs/README/examples/README change.
- Syncs the wiki with repository contents.

## Tips

- If a test fails, run the command locally.
- For local docs lint: `markdownlint "**/*.md"`.
