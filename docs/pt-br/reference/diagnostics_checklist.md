# Checklist de Diagnósticos

## Propósito

Manter uma lista verificável de requisitos de diagnóstico por fase do compilador.

## Lexer

- [x] Reporta erro com posição absoluta (offset em bytes)
- [x] Converte offset para linha/coluna (base 1)
- [x] Trecho de código com caret apontando para o token
- [x] Mensagem curta e objetiva

## Parser

- [x] Erro de token inesperado com span válido
- [x] EOF aponta para o fim do arquivo
- [x] Mostra token esperado (quando aplicável)

## Verificador de tipos

- [x] Erros incluem tipos esperado/encontrado
- [x] Mensagens para aridade incorreta
- [x] `return` ausente em funções não-`unit`
- [x] Spans (linha/coluna) quando disponíveis
- [x] Diagnósticos para funções anônimas (lambdas), valores de função e print

## CLI

- [x] Formato padrão consistente com a SPEC
- [x] Inclui arquivo/linha/coluna
- [x] Suporta saída limpa para pipes (sem ruído extra)

## Futuro

- [ ] Mensagens de erro ainda mais detalhadas e sugestões automáticas
