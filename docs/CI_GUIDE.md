# Guia de CI

## Objetivo

Descrever os workflows de CI e quando eles são executados.

## Workflows

### CI (tests)

Arquivo: `.github/workflows/ci.yml`

- Executa em `push` e `pull_request`.
- Roda testes por crate:
  - `tupa-lexer`
  - `tupa-parser`
  - `tupa-typecheck`
  - `tupa-cli`

### Docs Lint

Arquivo: `.github/workflows/docs-lint.yml`

- Executa em `push` e `pull_request` quando arquivos `.md` mudam.
- Rodam dois jobs:
  - `markdownlint` (formatação)
  - `link-check` (verificação de links)

### Sync Wiki

Arquivo: `.github/workflows/wiki-sync.yml`

- Executa em `push` na `main` quando docs/README/examples/README mudam.
- Sincroniza o wiki com o conteúdo do repositório.

## Dicas

- Se um teste falhar, rode o comando localmente.
- Para lint de docs local: `markdownlint "**/*.md"`.
- Para sync manual do wiki: `bash scripts/sync-wiki.sh`.
