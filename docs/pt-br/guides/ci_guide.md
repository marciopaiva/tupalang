# Guia de CI

## Propósito

Descrever os workflows de CI e quando eles rodam.

## Workflows

### CI (tests)

Arquivo: `.github/workflows/ci.yml`

- Roda em `push` e `pull_request`.
- Roda testes por crate:
  - `tupa-lexer`
  - `tupa-parser`
  - `tupa-typecheck`
  - `tupa-cli`

### Lint de docs

Arquivo: `.github/workflows/docs-lint.yml`

- Roda em `push` e `pull_request` quando arquivos `.md` mudam.
- Roda dois jobs:
  - `markdownlint` (formatação)
  - `link-check` (verificação de links)

### Sincronização do wiki

Arquivo: `.github/workflows/wiki-sync.yml`

- Roda em `push` para `main` quando docs/README/examples/README mudam.
- Sincroniza o wiki com o conteúdo do repositório.

## Dicas

- Se um teste falhar, rode o comando localmente.
- Para lint local de docs: `markdownlint "**/*.md"`.
- Para sincronização manual do wiki: `bash scripts/sync-wiki.sh`.
