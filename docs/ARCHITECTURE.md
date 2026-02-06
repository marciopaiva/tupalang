# Arquitetura

## Objetivo

Explicar a organização do repositório e o fluxo principal do compilador.

## Visão geral

O projeto é um workspace Rust com múltiplos crates que implementam etapas do compilador.

## Estrutura de pastas

- `crates/tupa-lexer`: tokenização do código-fonte.
- `crates/tupa-parser`: construção do AST.
- `crates/tupa-typecheck`: verificação de tipos e constraints.
- `crates/tupa-codegen`: geração de código (stub).
- `crates/tupa-cli`: interface de linha de comando.
- `docs/`: documentação de produto e especificação.
- `examples/`: exemplos executáveis.

## Fluxo principal

1) **Lexer**: converte texto em tokens.
2) **Parser**: transforma tokens em AST.
3) **Typechecker**: valida tipos e constraints.
4) **Codegen**: gera IR/código (stub no momento).
5) **CLI**: integra as etapas e expõe comandos (`lex`, `parse`, `check`, `codegen`).

## Dependências entre crates

- `tupa-parser` depende de `tupa-lexer`.
- `tupa-typecheck` depende de `tupa-parser`.
- `tupa-codegen` depende de `tupa-parser` e `tupa-typecheck`.
- `tupa-cli` depende de todos.

## Observações

- Diagnósticos seguem spans e erros normalizados por etapa.
- Saída JSON do CLI facilita integração com ferramentas.
