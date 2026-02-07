
# Changelog

## Objetivo

Registrar mudanças relevantes por versão.

## 0.3.0 (2026-02-07)

- Suporte a closures com captura real de variáveis (environment structures, heap allocation).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipos Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correção de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos entre docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correção de TODOs residuais no codegen para melhor robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (sem captura de ambiente ainda).
- Correção dos testes golden para casos de erro (removidas mensagens do cargo).

## 0.2.0 (2026-02-06)

- Suporte a closures com captura real de variáveis (environment structures, heap allocation).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipos Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correção de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos entre docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correção de TODOs residuais no codegen para melhor robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (sem captura de ambiente ainda).
- Correção dos testes golden para casos de erro (removidas mensagens do cargo).

## 0.1.0

- Especificação v0.1 publicada.
- Lexer, parser, typechecker e CLI básicos.
