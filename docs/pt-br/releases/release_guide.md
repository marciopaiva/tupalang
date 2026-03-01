# Guia de Release

## Propósito

Descrever o processo de release com passos claros e repetíveis.

## Passo a passo

1. Atualize [CHANGELOG](changelog.md) e index.md.
2. Rode a suíte de testes:

```bash
cargo test
```

1. Verifique exemplos principais em `examples/` e docs bilíngues.
2. Confirme que o CI está verde.
3. Crie a tag e publique:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

1. Crie o release no GitHub com as notas do CHANGELOG.

## Dicas

- Use versões semânticas (SemVer).
- Evite releases sem atualizar CHANGELOG e index.md.
- Registre mudanças que impactam SPEC, API ou docs principais.
