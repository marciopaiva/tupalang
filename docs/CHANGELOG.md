
# Changelog

## Objetivo

Registrar mudanças relevantes por versão.

## 0.2.0 (2026-02-06)

- Suporte completo a funções, funções anônimas (lambdas), valores de função e chamadas como valor.
- Print como built-in, concatenação de strings, arrays, controle de fluxo, etc.
- Codegen funcional (LLVM-like) cobrindo todos os recursos do MVP.
- Testes golden automatizados e integração contínua (CI) validando todo o pipeline.
- Diagnósticos aprimorados para tipos, aridade, print, lambdas, etc.

## Unreleased

- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correção de warnings.
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos entre docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correção de TODOs residuais no codegen para melhor robustez.

## 0.1.0

- Especificação v0.1 publicada.
- Lexer, parser, typechecker e CLI básicos.
