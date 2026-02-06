# Checklist de Release

## Objetivo

Garantir consistência e qualidade antes de publicar uma versão.

## Checklist

- [ ] Atualizar o [CHANGELOG](CHANGELOG.md)
- [ ] Rodar `cargo test`
- [ ] Validar exemplos principais em `examples/`
- [ ] Validar CLI (`lex`, `parse`, `check`)
- [ ] Revisar SPEC para inconsistências
- [ ] Taggear versão no Git (`vX.Y.Z`)
- [ ] Publicar release no GitHub
