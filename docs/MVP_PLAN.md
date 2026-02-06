# Plano de MVP

## Objetivo

Entregar um compilador mínimo que parseia, verifica tipos simples e gera um binário nativo para `hello.tp`.

## Índice

- [Escopo do MVP](#escopo-do-mvp)
- [Concluído](#concluído)
- [Próximos marcos](#próximos-marcos)
- [Critérios de Aceite](#critérios-de-aceite-quando-houver-implementação)

## Escopo do MVP

### Concluído

1. Lexer
   - Tokens básicos, comentários `//`/`/* */`, strings e números.
2. Parser
   - AST para funções, let, if, match, loops, arrays, chamadas, postfix e funções anônimas (lambdas).
3. Type Checker
   - Tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferência básica.
   - Tipos de função, checagem de chamadas, valores de função, lambdas e retorno em todos os caminhos.
4. Codegen
   - Geração de IR funcional (LLVM-like) para funções, lambdas, print, concatenação de strings, arrays, controle de fluxo, etc.
5. CLI
   - `tupa-cli` com `lex`, `parse`, `check`, `codegen`, stdin e execução de testes golden.
6. Diagnósticos
   - Span/linha/coluna em erros do lexer/parser/typechecker.
   - Mensagens para aridade, tipos, print, lambdas, etc.

## MVP

### Próximos marcos

1. Otimizações no codegen (eliminação de código morto, melhor uso de registradores)
2. Suporte a closures com captura de variáveis

3. Arrays/slices genéricos e mais tipos
4. Cobertura de testes e benchmarks

## Critérios de Aceite (quando houver implementação)

## Roadmap

- Erros de tipo claros e localizados.
- Nenhuma dependência de runtime externo para o binário gerado.
