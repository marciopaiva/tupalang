
# FAQ

## Propósito

Responder perguntas comuns sobre o projeto e a linguagem.

## Perguntas frequentes

### 1) O projeto está pronto para produção?

Ainda não. A especificação v0.1 está completa, mas o compilador ainda está sendo implementado.

### 2) Qual é o foco principal da linguagem?

Governança e determinismo para pipelines de IA em sistemas críticos, com segurança formal e desempenho previsível.

### 3) Como posso contribuir?

Veja [CONTRIBUTING.md](../../CONTRIBUTING.md) e abra uma issue com contexto.

### 4) Onde encontro exemplos?

Em [examples](../../examples/README.md) e em [docs/spec.md](../reference/spec.md#10-validated-examples).

### 4.1) Há exemplos safe/alinhamento?

Sim. Veja os exemplos `safe_*` em [examples](../../examples/README.md) e a seção `Safe<string, ...>` em [docs/spec.md](../reference/spec.md#alignment-types-ethical-constraints).

### 5) O que são os tipos `Safe<T, ...>`?

Tipos com restrições provadas em tempo de compilação, por exemplo `Safe<f64, !nan>` ou `Safe<string, !misinformation>`. Veja detalhes em [docs/spec.md](../reference/spec.md#alignment-types-ethical-constraints).

### 6) Como executo o CLI?

Use `cargo run -p tupa-cli -- <command>` e veja [docs/getting_started.md](getting_started.md).

### 7) Existe um roadmap?

Sim: [docs/mvp_plan.md](../overview/mvp_plan.md) e [docs/adoption_plan.md](../governance/adoption_plan.md).

### 8) Posso propor mudanças na spec?

Sim. Abra uma issue com o prefixo `[RFC]`.

### 9) Como funciona a interoperabilidade com outras linguagens?

O design inclui FFI (Foreign Function Interface) para integração com Rust, C e Python. Veja [docs/spec.md](../reference/spec.md#7-modules--ffi).

### 10) Como é o desempenho em comparação a outras linguagens?

O objetivo é desempenho previsível, próximo de Rust/C para código crítico. Benchmarks e exemplos serão publicados em releases futuras.

### 11) Como depuro ou obtenho diagnósticos detalhados?

Veja [docs/diagnostics_checklist.md](../reference/diagnostics_checklist.md) e [docs/common_errors.md](../reference/common_errors.md) para exemplos de mensagens e dicas.

### 12) Há dicas de uso ou boas práticas?

Veja [docs/spec.md](../reference/spec.md#comparison) para exemplos e comparações e [docs/README.md](../index.md) para links rápidos.

### 13) Como contribuo com exemplos ou documentação?

Veja [CONTRIBUTING.md](../../CONTRIBUTING.md) e [docs/docs_contributing.md](docs_contributing.md).
