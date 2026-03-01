# Checklist de Release

## Propósito

Garantizar consistencia y calidad antes de publicar un release.

## Checklist

- [ ] Actualiza [CHANGELOG](changelog.md)
- [ ] Actualiza [INDEX](../index.md)
- [ ] Ejecuta `cargo test`
- [ ] Ejecuta `markdownlint "**/*.md"`
- [ ] Valida los ejemplos principales en `examples/`
- [ ] Valida el CLI (`lex`, `parse`, `check`)
- [ ] Revisa la SPEC por inconsistencias
- [ ] Etiqueta la versión en Git (`vX.Y.Z`)
- [ ] Publica el release en GitHub
