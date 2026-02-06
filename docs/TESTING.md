# Guia de Testes

## Objetivo

Descrever comandos padrão de teste e dicas de triagem de falhas.

## Comandos principais

```bash
# suite completa
cargo test

# por crate
cargo test -p tupa-lexer
cargo test -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli
```

## Testes do CLI

```bash
# golden outputs
cargo test -p tupa-cli -- tests::cli_golden
```

## Dicas de triagem

- Rode o teste isolado antes do suite completo.
- Verifique se o erro é de parsing ou typecheck.
- Compare spans e mensagens com o esperado.
- Reproduza via `tupa-cli -- parse|check`.
