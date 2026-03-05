# Guia de Release

## Prop?sito

Descrever o processo de release com passos claros e repet?veis.

## Fonte de verdade do changelog

- O Release Draft do GitHub ? a fonte de verdade para notas de release.
- Antes de criar tag, reconcilie o draft com o [CHANGELOG](changelog.md).

## Passo a passo

1. Revise o Release Draft atual.
2. Revise labels das PRs mergeadas e corrija inconsist?ncias.
3. Atualize [CHANGELOG](changelog.md) a partir do draft.
4. Execute valida??o local:

```bash
./scripts/ci-local.sh
```

5. Execute o gate de verifica??o de release:

```bash
./scripts/release-verify.sh X.Y.Z
```

6. Confirme checks obrigat?rios verdes em `main`.
7. Verifique exemplos principais e docs multil?ngues.
8. Crie a tag e publique:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

9. Crie o release no GitHub a partir do draft.

## Go/No-Go (RC e Final)

Go somente se tudo abaixo for verdadeiro:

- `./scripts/release-verify.sh X.Y.Z` passa.
- Smoke ViperTrade-like passa (`./scripts/vipertrade-smoke.sh`).
- Checks obrigat?rios verdes.
- Changelog `X.Y.Z` alinhado em EN/ES/PT-BR.
- Sem issues bloqueadoras abertas para o milestone.

No-Go em qualquer caso abaixo:

- Falha em check obrigat?rio.
- Desalinhamento de docs/changelog entre idiomas.
- Smoke test com status diferente de `pass`.

## Checklist r?pida

Veja [Release Cut Checklist](release_cut_checklist.md).
