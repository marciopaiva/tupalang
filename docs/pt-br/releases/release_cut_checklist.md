# Checklist de Corte de Release

Use este checklist ao criar uma nova tag de release.

## Antes da tag

- [ ] O Release Draft está atualizado e agrupado por labels.
- [ ] O CHANGELOG foi atualizado a partir do draft.
- [ ] Os checks obrigatórios estão verdes na `main`.
- [ ] A validação local passou: `./scripts/ci-local.sh`.
- [ ] Não há bloqueadores abertos em issues de alta prioridade.

## Tag e publicação

- [ ] Tag criada: `vX.Y.Z`.
- [ ] Tag enviada para origin.
- [ ] GitHub Release criado a partir do draft.

## Após o release

- [ ] Anunciar o release nos canais do time.
- [ ] Abrir issues de acompanhamento para itens adiados.
- [ ] Confirmar o escopo do próximo milestone.
