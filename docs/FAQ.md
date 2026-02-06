
# FAQ

## Objetivo

Responder dúvidas comuns sobre o projeto e a linguagem.

## Perguntas frequentes

### 1) O projeto já está pronto para produção?
Ainda não. A especificação v0.1 está completa, mas o compilador está em implementação.

### 2) Qual é o foco principal da linguagem?
IA e sistemas críticos com segurança formal, alinhamento e performance previsível.

### 3) Como contribuo?
Veja [CONTRIBUTING.md](../CONTRIBUTING.md) e abra uma issue com contexto.

### 4) Onde encontro exemplos?
Na pasta [examples](../examples/README.md) e em [docs/SPEC.md](SPEC.md#exemplos).

### 5) O que são `Safe<T, ...>`?
Tipos com restrições provadas em *compile-time*, por exemplo `Safe<f64, !nan>`. Veja detalhes em [docs/SPEC.md](SPEC.md#alignment-types-ethical-constraints).

### 6) Como executar o CLI?
Use `cargo run -p tupa-cli -- <comando>` e consulte [docs/GETTING_STARTED.md](GETTING_STARTED.md).

### 7) Existe roadmap?
Sim: [docs/MVP_PLAN.md](MVP_PLAN.md) e [docs/ADOPTION_PLAN.md](ADOPTION_PLAN.md).

### 8) Posso propor mudanças na spec?
Sim. Abra uma issue com prefixo `[RFC]`.

### 9) Como é a interoperabilidade com outras linguagens?
O design prevê FFI (Foreign Function Interface) para integração com Rust, C e Python. Veja [docs/SPEC.md](SPEC.md#7-modules--ffi).

### 10) Como é a performance comparada a outras linguagens?
O objetivo é performance previsível, próxima de Rust/C para código crítico. Benchmarks e exemplos serão publicados em releases futuros.

### 11) Como faço debug ou obtenho diagnósticos detalhados?
Consulte [docs/DIAGNOSTICS_CHECKLIST.md](DIAGNOSTICS_CHECKLIST.md) e [docs/COMMON_ERRORS.md](COMMON_ERRORS.md) para exemplos de mensagens e dicas.

### 12) Existem dicas de uso ou melhores práticas?
Veja [docs/SPEC.md](SPEC.md#comparação) para exemplos e comparações, e [docs/README.md](README.md) para links rápidos.

### 13) Como contribuir com exemplos ou documentação?
Veja [CONTRIBUTING.md](../CONTRIBUTING.md) e [docs/DOCS_CONTRIBUTING.md](DOCS_CONTRIBUTING.md).
