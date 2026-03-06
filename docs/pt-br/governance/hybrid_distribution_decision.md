# Decisão de Distribuição Híbrida (Binário Standalone + Crates Rust Públicas)

## Status

Aceita para `v0.8.0-rc`.

## Contexto

Tupa precisa atender dois públicos distintos:

- Usuários finais (times de ML/compliance/plataforma) que precisam de distribuição simples por executável.
- Integradores Rust que precisam de embeddability via crates estáveis.

Uma estratégia apenas binário prejudica embeddability. Uma estratégia apenas crates prejudica adoção e operação.

## Decisão

Tupa adota um modelo de distribuição híbrido:

1. Distribuição principal: binário CLI standalone (`tupa-cli`) publicado como artefato de release.
2. Distribuição secundária: crates Rust públicas selecionadas para embedding (`tupa-parser`, `tupa-typecheck`, `tupa-runtime`).
3. Núcleo compartilhado: uma base de código e uma política de CI para as duas superfícies.

## Escopo para `v0.8.0-rc`

- Adicionar workflow de release para gerar/publicar binários multiplataforma e checksums.
- Documentar instalação por binário e fluxo de embedding em Rust.
- Manter compromisso de estabilidade de API apenas para as crates selecionadas.

## Fora de Escopo deste RC

- Automação de Homebrew tap.
- Publicação de imagem Docker oficial.
- Endpoint externo de instalador (por exemplo, `tupa.dev/install.sh`).

## Consequências

- Melhor caminho de adoção para usuários não Rust.
- Embeddability preservada para ecossistemas Rust.
- Responsabilidades de release mais claras (artefatos + checksums + docs).
