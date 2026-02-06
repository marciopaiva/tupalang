# Arquitetura

## Objetivo

Explicar a organização do repositório e o fluxo principal do compilador.

## Visão geral

O projeto é um workspace Rust com múltiplos crates que implementam etapas do compilador.


## Estrutura de pastas

- `crates/tupa-lexer`: tokenização do código-fonte.
- `crates/tupa-parser`: construção do AST.
- `crates/tupa-typecheck`: verificação de tipos e constraints, incluindo funções anônimas (lambdas) e valores de função.
- `crates/tupa-codegen`: geração de código IR funcional (LLVM-like), com suporte a funções, lambdas, print, concatenação de strings, arrays, controle de fluxo, etc.
- `crates/tupa-cli`: interface de linha de comando, integração de todas as etapas e execução de testes golden.
- `docs/`: documentação de produto e especificação.
- `examples/`: exemplos executáveis e testes golden.


## Fluxo principal

1) **Lexer**: converte texto em tokens.
2) **Parser**: transforma tokens em AST.
3) **Typechecker**: valida tipos, constraints, funções/lambdas e valores de função.
4) **Codegen**: gera IR funcional (LLVM-like) com suporte a todos os recursos do MVP.
5) **CLI**: integra as etapas, expõe comandos (`lex`, `parse`, `check`, `codegen`) e executa testes golden automatizados.

## Dependências entre crates

- `tupa-parser` depende de `tupa-lexer`.
- `tupa-typecheck` depende de `tupa-parser`.
- `tupa-codegen` depende de `tupa-parser` e `tupa-typecheck`.
- `tupa-cli` depende de todos.


## Observações

- Diagnósticos seguem spans e erros normalizados por etapa.
- Saída JSON do CLI facilita integração com ferramentas.
- Testes golden garantem a estabilidade do pipeline.
- Veja detalhes do typechecker em [TYPECHECKER_DETAILS.md](TYPECHECKER_DETAILS.md).
