# Plano de MVP

## Objetivo

Entregar um compilador mínimo que parseia, verifica tipos simples e gera um binário nativo para `hello.tp`.

## Escopo do MVP

1. Lexer
   - Tokens básicos, comentários `//`, strings e números.
2. Parser
   - AST para funções, let, if, match e chamadas simples.
3. Type Checker
   - Tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferência básica.
4. Codegen (LLVM)
   - Funções, aritmética básica e `print`.
5. CLI
   - `tupa-cli` com `build` e `run`.

## Critérios de Aceite

- Compilar e executar `examples/hello.tp`.
- Erros de tipo claros e localizados.
- Nenhuma dependência de runtime externo para o binário gerado.
