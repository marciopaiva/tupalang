# Guia de Testes

## Propósito

Comandos de teste padrão e dicas de triagem de falhas para o Tupã 0.9.x (era Rust-DSL).

---

## Comandos principais

```bash
# Suite completa do workspace
cargo test --workspace --locked

# Por crate (apenas crates ativos)
cargo test -p tupa-core
cargo test -p tupa-core-macros
cargo test -p tupa-engine
cargo test -p tupa-plugin
cargo test -p tupa-pyffi
cargo test -p cargo-tupa
```

---

## Testes do cargo-tupa

```bash
# Testes unitários dos subcomandos do CLI
cargo test -p cargo-tupa

# Teste de integração (saída de métricas)
cargo test -p cargo-tupa --test run_metrics
```

---

## Benchmarks de desempenho

A suite de benchmarks do `tupa-engine` (com `criterion`) roda com:

```bash
cargo bench -p tupa-engine
```

Para medições rigorosas, use builds de release e `hyperfine` com aquecimento.

---

## Dicas de triagem

- Rode o teste isolado antes da suite completa.
- Distinga erros de compilação (`rustc` / macro `pipeline!`) de erros de execução
  (`Executor::run` retorna `PipelineResult`).
- Compare mensagens e códigos de diagnóstico com a saída esperada.
