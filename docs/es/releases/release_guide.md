# Guía de Release

## Propósito

Describir el proceso de release con pasos claros y repetibles.

## Paso a paso

1. Actualiza [CHANGELOG](changelog.md) y index.md.
2. Ejecuta la suite de tests:

```bash
cargo test
```

1. Verifica los ejemplos principales en `examples/` y docs bilingües.
2. Confirma que CI esté en verde.
3. Crea el tag y publica:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

1. Crea el release en GitHub con las notas del CHANGELOG.

## Consejos

- Usa versiones semánticas (SemVer).
- Evita releases sin actualizar CHANGELOG e index.md.
- Registra cambios que impacten SPEC, API o docs principales.
