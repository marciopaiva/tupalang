# Gu?a de Release

## Prop?sito

Describir el proceso de release con pasos claros y repetibles.

## Fuente de verdad del changelog

- El Release Draft de GitHub es la fuente de verdad para notas de release.
- Antes de etiquetar, reconcilia el draft con [CHANGELOG](changelog.md).

## Paso a paso

1. Revisa el Release Draft actual.
2. Revisa labels de PR mergeadas y corrige inconsistencias.
3. Actualiza [CHANGELOG](changelog.md) a partir del draft.
4. Ejecuta validaci?n local:

```bash
./scripts/ci-local.sh
```

5. Ejecuta el gate de verificaci?n de release:

```bash
./scripts/release-verify.sh X.Y.Z
```

6. Confirma que los checks requeridos est?n en verde en `main`.
7. Verifica ejemplos principales y docs multilenguaje.
8. Crea la tag y publica:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

9. Crea el release en GitHub desde el draft.

## Go/No-Go (RC y Final)

Go solo si se cumple todo:

- `./scripts/release-verify.sh X.Y.Z` pasa.
- Smoke ViperTrade-like pasa (`./scripts/vipertrade-smoke.sh`).
- Checks requeridos en verde.
- Changelog `X.Y.Z` alineado en EN/ES/PT-BR.
- Sin issues bloqueantes abiertos para el milestone.

No-Go si ocurre cualquiera:

- Falla en checks requeridos.
- Desalineaci?n de docs/changelog entre idiomas.
- Smoke test con estado distinto de `pass`.

## Checklist r?pida

Ver [Release Cut Checklist](release_cut_checklist.md).
