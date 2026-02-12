# Documentation Contribution Guide

## Purpose

Standardize documentation changes to keep quality and consistency.

## Scope

- README, docs, and examples.
- Wiki pages mirrored from docs.

## Language policy

English is the canonical language in `docs/`. PT-BR alternatives live under `docs/pt-br`. Keep translations aligned with the English version and note when a PT-BR page is pending.

## Single source

The canonical content lives in `docs/`. The wiki is updated automatically via CI.

## Writing standards

- Clear, direct English.
- Short sentences.
- Avoid jargon without an explanation.
- Use objective, predictable headings.

## Recommended structure

- **Purpose** right after the title.
- Short sections with subtitles.
- Lists for steps and requirements.

## PR checklist (docs)

- [ ] Is the purpose clear?
- [ ] Do internal links work?
- [ ] Are examples small and runnable?
- [ ] Is the content consistent with SPEC?
- [ ] Does the wiki need sync?
- [ ] If PT-BR exists, is it aligned or flagged as pending?

## Wiki sync

The wiki is synced automatically via workflow. If you need to force it, run:

```bash
bash scripts/sync-wiki.sh
```
