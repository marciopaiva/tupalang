# Plano Mínimo de Adoção Técnica

## Propósito

Definir um caminho incremental para tornar a linguagem usável e confiável, sem se comprometer com datas.

## Índice

- [Fase 0: Núcleo mínimo](#fase-0-núcleo-mínimo)
- [Fase 1: Toolchain básica](#fase-1-toolchain-básica)
- [Fase 2: Experiência do desenvolvedor](#fase-2-experiência-do-desenvolvedor)
- [Fase 3: Interoperabilidade](#fase-3-interoperabilidade)
- [Fase 4: Qualidade e confiança](#fase-4-qualidade-e-confiança)
- [Entregáveis mínimos](#entregáveis-mínimos)

## Fase 0: Núcleo mínimo

- Definir o subconjunto central (sintaxe e tipos básicos).
- Especificação formal mínima (EBNF + semântica de tipos).
- Suite de conformidade (parser + type checker).
- Saída de diagnósticos consumível por ferramentas (JSON).

## Fase 1: Toolchain básica

- Formatador oficial.
- Linter com regras mínimas.
- Language server (autocomplete, diagnósticos, go-to-definition).

## Fase 2: Experiência do desenvolvedor

- Templates de projeto (CLI e serviço).
- CLI estável com `build`, `run`, `fmt`, `check`.
- Mensagens de erro didáticas e consistentes.

## Fase 3: Interoperabilidade

- FFI com C/Rust.
- ABI documentada.
- Bindings mínimos para bibliotecas essenciais.

## Fase 4: Qualidade e confiança

- Benchmarks públicos e reprodutíveis.
- Testes de regressão de desempenho.
- Política de versionamento e compatibilidade.

## Entregáveis mínimos

- SPEC com EBNF e regras de tipos.
- Testes automatizados de parser/type checker.
- CLI funcional com exemplos simples e `--format pretty|json`.
