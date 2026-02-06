# Guia de Release

## Objetivo

Descrever o processo de release com passos claros e repetíveis.

## Passo a passo

1. Atualize o [CHANGELOG](CHANGELOG.md) e SUMMARY.md.
2. Rode a suite de testes:

```bash
cargo test
```

1. Verifique exemplos principais em `examples/` e docs bilíngues.
2. Confirme que o CI está verde.
3. Crie tag e publique:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

1. Crie a release no GitHub com notas do CHANGELOG.

## Dicas

- Use versões semânticas (SemVer).
- Evite releases sem atualização do CHANGELOG e SUMMARY.
- Registre mudanças que impactam a SPEC, API ou docs principais.
