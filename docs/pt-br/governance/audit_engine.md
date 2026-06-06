# Motor de Auditoria (descontinuado)

> **Removido na 0.9.0.** A auditoria por hash determinístico descrita aqui fazia
> parte do toolchain `.tp` standalone (crates `tupa-audit` e `tupa-parser`),
> removido na 0.9.0. A arquitetura atual baseada em crates **não** inclui esse
> recurso de hash de execução.

## O que existia

O recurso gerava uma impressão SHA3-256 estável de uma execução combinando a AST
normalizada, as entradas JSON canônicas e a versão do compilador (via
`tupa-audit::hash_execution`). Como o compilador `.tp` foi removido, essa função
e seus crates não existem mais no workspace.

## Mecanismo atual de observabilidade

Para rastrear execuções no Rust-DSL atual, use as métricas por passo do
`tupa-engine`:

- `PipelineResult::metrics` — um `Vec<StepMetrics>` com `step_id`, timestamps de
  início/fim e duração de cada passo.
- `PipelineResult::passed` — resultado agregado da avaliação de constraints.

Para reprodutibilidade ou integridade no estilo de auditoria, faça o hashing no
nível da sua aplicação sobre a entrada serializada e os valores resultantes
(`PipelineResult::values`). Veja também o [TRANSITION.md](../TRANSITION.md) para a
migração do fluxo `.tp` legado.
