# Diagnostics Checklist

## Objetivo

Manter uma lista verificável de requisitos de diagnóstico por fase do compilador.

## Lexer

- [x] Reporta erro com posição absoluta (byte offset)
- [x] Converte offset para linha/coluna (1-based)
- [x] Trecho de código com caret apontando o token
- [x] Mensagem curta e objetiva

## Parser

- [x] Erro de token inesperado com span válido
- [x] EOF aponta para fim do arquivo
- [ ] Mostra token esperado (quando aplicável)

## Typechecker

- [x] Erros incluem tipo esperado/encontrado
- [x] Mensagens para aridade incorreta
- [x] `return` ausente em funções não-`unit`
- [x] Spans (linha/coluna) quando disponíveis

## CLI

- [x] Formato padrão consistente com SPEC
- [x] Inclui arquivo/linha/coluna
- [x] Suporta saída limpa para pipes (sem ruído extra)
