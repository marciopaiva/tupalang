# Plano técnico mínimo de adoção

## Objetivo

Definir um caminho incremental para tornar a linguagem utilizável e confiável, sem assumir datas.

## Fase 0: Núcleo mínimo

- Definir o subset core (sintaxe e tipos básicos).
- Especificação formal mínima (EBNF + semântica de tipos).
- Suite de testes de conformidade (parser + type checker).

## Fase 1: Toolchain básica

- Formatter oficial.
- Linter com regras mínimas.
- Language Server (autocomplete, diagnostics, go-to-definition).

## Fase 2: Experiência do dev

- Templates de projeto (CLI e service).
- CLI estável com `build`, `run`, `fmt`, `check`.
- Mensagens de erro didáticas e consistentes.

## Fase 3: Interoperabilidade

- FFI com C/Rust.
- ABI documentada.
- Bindings mínimos para libs essenciais.

## Fase 4: Qualidade e confiança

- Benchmarks públicos e reproduzíveis.
- Testes de regressão para performance.
- Política de versões e compatibilidade.

## Entregáveis mínimos

- SPEC com EBNF e regras de tipos.
- Testes automatizados de parser/type checker.
- CLI funcional com exemplos simples.
