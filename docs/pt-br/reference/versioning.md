# Guia de Versionamento

## Propósito

Definir a política de versionamento e compatibilidade para linguagem, crates e distribuição binária.

## SemVer

Seguimos o [SemVer](https://semver.org/):

- **MAJOR**: mudanças incompatíveis.
- **MINOR**: novas features compatíveis.
- **PATCH**: correções compatíveis.

## Pré-1.0

Antes da 1.0, mudanças podem acontecer com mais frequência. Ainda seguimos SemVer e documentamos mudanças incompatíveis no CHANGELOG.

## Release Candidates

Release candidates usam o formato `vX.Y.Z-rc.N` e devem ser tratados como builds pré-GA.

- Tags RC publicam artefatos de release para validação.
- Garantias de API ficam limitadas às superfícies estáveis documentadas.

## Modelo de distribuição (v0.8.1)

Tupa usa um modelo híbrido:

- Artefatos binários standalone para adoção de usuários finais.
- Crates Rust públicas para embedding em sistemas Rust.

Veja [Decisão de distribuição híbrida](../governance/hybrid_distribution_decision.md).

## Sincronização de documentação

Qualquer mudança relevante em docs, exemplos ou API deve ser refletida no CHANGELOG e, se aplicável, em index.md e README.
