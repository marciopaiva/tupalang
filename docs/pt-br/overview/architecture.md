
# Arquitetura

## Propósito

Explicar a organização do repositório e o fluxo principal do compilador.

## Visão geral

O projeto é um workspace Rust com múltiplas crates implementando fases do compilador.

## Estrutura de pastas

- `crates/tupa-lexer`: tokenização do código-fonte.
- `crates/tupa-parser`: construção de AST.
- `crates/tupa-typecheck`: verificação de tipos e restrições, incluindo funções anônimas (lambdas) e valores de função.
- `crates/tupa-codegen`: geração de IR funcional (LLVM-like), com suporte a funções, lambdas, print, concatenação de strings, arrays, fluxo de controle e mais.
- `crates/tupa-audit`: hash de auditoria determinístico para AST + entradas.
- `crates/tupa-cli`: interface de linha de comando, integração de todas as fases e execução de testes goldens.
- `docs/`: documentação do produto e especificação.
- `examples/`: exemplos executáveis e testes goldens.

## Fluxo principal

1) **Lexer**: converte texto em tokens.
2) **Parser**: transforma tokens em AST.
3) **Verificador de tipos**: valida tipos, restrições, funções/lambdas e valores de função.
4) **Codegen**: gera IR funcional (LLVM-like) cobrindo features do MVP.
5) **Audit**: gera hashes determinísticos a partir de AST e entradas.
6) **CLI**: integra fases, expõe comandos (`lex`, `parse`, `check`, `codegen`, `audit`) e roda testes goldens automatizados.

## Dependências entre crates

- `tupa-parser` depende de `tupa-lexer`.
- `tupa-typecheck` depende de `tupa-parser`.
- `tupa-codegen` depende de `tupa-parser` e `tupa-typecheck`.
- `tupa-audit` depende de `tupa-parser`.
- `tupa-cli` depende de todas.

## Notas

- Diagnósticos seguem spans e erros normalizados por fase.
- Saída JSON do CLI permite integração de ferramentas.
- Testes goldens garantem estabilidade do pipeline.
- Veja detalhes do verificador de tipos em [typechecker_details.md](../reference/typechecker_details.md).
