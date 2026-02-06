# Issues iniciais sugeridas

1. Diagnósticos com span/linha/coluna (concluído)
   - Padronizar mensagens de erro no lexer/parser/typechecker.
2. Type checker v1 (concluído)
   - Tipos de função, checagem de `return`, `match` e loops.
3. CLI: formatos de saída (concluído)
   - `--format pretty|json` para tokens/AST/diagnósticos.
4. Codegen MVP (LLVM)
   - Funções, aritmética básica e `print`.
5. Mais exemplos
   - Casos reais + edge cases em `examples/`.
6. Alignment Types: validação de constraints (parcial)
   - Checagem inicial para `!nan`/`!inf` em literais `f64`.
   - Pendência: solver/propagação para expressões gerais.
