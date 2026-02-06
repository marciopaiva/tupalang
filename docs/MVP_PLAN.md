# Plano de MVP

## Objetivo

Entregar um compilador mínimo que parseia, verifica tipos simples e gera um binário nativo para `hello.tp`.

## Escopo do MVP

### Concluído
1. Lexer
   - Tokens básicos, comentários `//`/`/* */`, strings e números.
2. Parser
   - AST para funções, let, if, match, loops, arrays, chamadas e postfix.
3. Type Checker
   - Tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferência básica.
   - Tipos de função, checagem de chamadas e retorno em todos os caminhos.
4. CLI
   - `tupa-cli` com `lex`, `parse`, `check` e stdin.
5. Diagnósticos
   - Span/linha/coluna em erros do lexer/parser/typechecker.
6. Type Checker (v1)
   - Retornos, `match`, loops e tipos de função.

### Próximos marcos
7. Codegen (LLVM)
   - Funções, aritmética básica e `print`.

## Critérios de Aceite (quando houver implementação)

- Compilar e executar `examples/hello.tp`.
- Erros de tipo claros e localizados.
- Nenhuma dependência de runtime externo para o binário gerado.
