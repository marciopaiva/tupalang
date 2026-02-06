# Guia de Release

## Objetivo

Descrever o processo de release com passos claros e repetíveis.

## Passo a passo

1) Atualize o [CHANGELOG](CHANGELOG.md).
1) Rode a suite de testes:

```bash
cargo test
```

1) Verifique exemplos principais em `examples/`.
1) Confirme que o CI está verde.
1) Crie tag e publique:

```bash
git tag -a v0.1.0 -m "v0.1.0"
git push origin v0.1.0
```

1) Crie a release no GitHub com notas do CHANGELOG.

## Dicas

- Use versões semânticas (SemVer).
- Evite releases sem atualização do CHANGELOG.
- Registre mudanças que impactam a SPEC.
