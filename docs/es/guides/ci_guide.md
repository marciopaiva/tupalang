# Guía de CI

## Propósito

Describir los workflows de CI y cuándo se ejecutan.

## Workflows

### CI (tests)

Archivo: `.github/workflows/ci.yml`

- Se ejecuta en `push` y `pull_request`.
- Ejecuta pruebas por crate:
  - `tupa-lexer`
  - `tupa-parser`
  - `tupa-typecheck`
  - `tupa-cli`

### Lint de docs

Archivo: `.github/workflows/docs-lint.yml`

- Se ejecuta en `push` y `pull_request` cuando cambian archivos `.md`.
- Ejecuta dos jobs:
  - `markdownlint` (formato)
  - `link-check` (verificación de enlaces)

### Sincronización del wiki

Archivo: `.github/workflows/wiki-sync.yml`

- Se ejecuta en `push` a `main` cuando cambian docs/README/examples/README.
- Sincroniza el wiki con el contenido del repositorio.

## Consejos

- Si una prueba falla, ejecuta el comando localmente.
- Para lint local de docs: `markdownlint "**/*.md"`.
- Para sincronización manual del wiki: `bash scripts/sync-wiki.sh`.
