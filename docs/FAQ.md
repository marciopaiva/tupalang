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

Na pasta [examples](../examples/README.md).

### 5) O que são `Safe<T, ...>`?

Tipos com restrições provadas em *compile-time*, por exemplo `Safe<f64, !nan>`.

### 6) Como executar o CLI?

Use `cargo run -p tupa-cli -- <comando>` e consulte [docs/GETTING_STARTED.md](GETTING_STARTED.md).

### 7) Existe roadmap?

Sim: [docs/MVP_PLAN.md](MVP_PLAN.md) e [docs/ADOPTION_PLAN.md](ADOPTION_PLAN.md).

### 8) Posso propor mudanças na spec?

Sim. Abra uma issue com prefixo `[RFC]`.
