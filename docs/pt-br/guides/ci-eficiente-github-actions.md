# Proposta de CI Eficiente (GitHub Actions)

## Objetivos

- Confiabilidade: garantir disparo previsivel em `push`/`pull_request`.
- Velocidade: redunir tempo total de feedback para mudancas de codigo.
- Custo: evitar execucoes desnecessarias para mudancas so de documentacao.
- Seguranca: minimizar permissoes e separar jobs de leitura/escrita.

## Mudanca imediata aplicada

No workflow `CI` (`.github/workflows/ci.yml`):

- adiciona `workflow_dispatch` para execucao manual;
- adiciona `concurrency` com `cancel-in-progress: true`;
- aplica `paths-ignore` para docs (`**/*.md`, `docs/**`, markdownlint config);
- ativa cache de Rust (`Swatinem/rust-cache@v2`);
- usa `cargo clippy --workspace --all-targets -- -D warnings`;
- usa `cargo test --workspace --locked`;
- define `permissions: contents: read`.

## Ajustes recomendados na sequencia

1. `examples-golden.yml`

- Rodar apenas quando houver mudancas em `examples/**`, `crates/tupa-cli/**`, `scripts/update-goldens.sh`.
- Adicionar `workflow_dispatch` e `concurrency`.
- Nao abortar antes de publicar artifact/diff.
- Separar job de verificacao (`read`) do job de criacao de PR (`write`, so em `push` para `main`).

2. `docs-lint.yml`

- Adicionar `concurrency` para cancelar execucoes antigas no mesmo branch.
- Opcional: quebrar `link-check` em job independente com timeout menor.

3. Branch protection

- Exigir checks obrigatorios: `CI / lint` e `CI / test`.
- Manter `Docs Lint` e `Examples Golden Tests` como obrigatorios apenas se o time quiser bloqueio nessas areas.

## Meta de desempenho (SLO)

- PR comum (codigo Rust): feedback inicial em ate 5 min.
- PR completo: conclusao em ate 12 min.
- Reexecucoes redundantes no mesmo branch: canceladas automaticamente.
