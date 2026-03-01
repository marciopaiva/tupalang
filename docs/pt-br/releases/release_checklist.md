# Checklist de Release

## Propósito

Garantir consistência e qualidade antes de publicar um release.

## Checklist

- [ ] Atualize [CHANGELOG](changelog.md)
- [ ] Atualize [INDEX](index.md)
- [ ] Rode `cargo test`
- [ ] Rode `markdownlint "**/*.md"`
- [ ] Valide exemplos principais em `examples/`
- [ ] Valide o CLI (`lex`, `parse`, `check`)
- [ ] Revise a SPEC em busca de inconsistências
- [ ] Tagueie a versão no Git (`vX.Y.Z`)
- [ ] Publique o release no GitHub
