# Plano MVP

## Propósito

Entregar um compilador mínimo que faz parsing, checa tipos simples e gera um binário nativo para `hello.tp`.

## Índice

- [Escopo do MVP](#escopo-do-mvp)
- [Concluído](#concluído)
- [Próximos marcos](#próximos-marcos)
- [Critérios de aceitação](#critérios-de-aceitação-quando-implementado)

## Escopo do MVP

### Concluído

1. Lexer
   - Tokens básicos, comentários `//`/`/* */`, strings e números.
2. Parser
   - AST para funções, let, if, match, loops, arrays, chamadas, postfix e funções anônimas (lambdas).
3. Verificador de tipos
   - Tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferência básica.
   - Tipos de função, checagem de chamadas, valores de função, lambdas e retorno em todos os caminhos.
4. Geração de código
   - Geração de IR funcional (LLVM-like) para funções, lambdas, print, concatenação de strings, arrays, fluxo de controle e mais.
5. CLI
   - `tupa-cli` com `lex`, `parse`, `check`, `codegen`, stdin e testes goldens.
6. Diagnósticos
   - Span/linha/coluna em erros de lexer/parser/typechecker.
   - Mensagens para aridade, tipos, print, lambdas e mais.
7. Closures
   - Suporte a closures com captura real de variáveis (estruturas de ambiente, alocação em heap).

## MVP

### Próximos marcos

1. Otimizações de geração de código (eliminação de código morto, melhor uso de registradores)
2. Arrays/slices genéricos e mais tipos
3. Cobertura de testes e benchmarks

## Critérios de aceitação (quando implementado)

## Roadmap

- Erros de tipo claros e localizados.
- Sem dependências externas em tempo de execução para binários gerados.
