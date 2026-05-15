# Plano de Correção da Documentação (v0.9.4) — Estado Final

**Última atualização:** 2026-05-15
**CI local:** ✅ `All local CI checks passed`

---

## ✅ Resumo de Status das Fases

| Fase | Status | Progresso |
|------|--------|-----------|
| **Fase 1** — Correções Críticas P1 | ✅ CONCLUÍDA | GAP-1, GAP-2, GAP-4 resolvidos |
| **Fase 2** — Engine Features Coverage P2 | ✅ CONCLUÍDA | timeout, metrics, cancellation, `--metrics-output` documentados |
| **Fase 3** — Missing Content Creation P3 | ✅ CONCLUÍDA | migration_guide (EN+ES), plugin tutorials (EN+ES), exemplos de plugin (Rust) |
| **Fase 4** — Legacy Cleanup P4 | ✅ CONCLUÍDA | spec.md limpo, PROPOSAL nota, OVERVIEWs pendentes em docs |
| **Fase 5** — Validação Final | ✅ CONCLUÍDA | CI local green, golden tests, cargo-tupa refinements |

---

## Fase 1 — Correções Críticas P1 ✅

| ID | Arquivo | Ação |
|----|---------|------|
| GAP-1 | `docs/{en,pt-br}/guides/cargo_tupa_guide.md` | Removidas referências `.tp` de `fmt`/`lint`; nota de depreciação adicionada |
| GAP-2 | `crates/cargo-tupa/src/main.rs` | Subcommand `Test` **removido** da enum `Commands` |
| GAP-4 | — | `discover` module existe mas não registrado (deferred corretamente) |
| GAP-5 | `crates/cargo-tupa/src/fmt.rs` | `Context` não importado |

### GAP-2 detalhe — `cargo tupa test` removido

O subcommand `Test` foi removido de `Commands` e `test_cmd` não é mais importado. Usuários devem usar `cargo test` diretamente.

---

## Fase 2 — Engine Features P2 ✅

| Feature | Doc |
|---------|-----|
| Per-step timeout (`TUPA_STEP_TIMEOUT`) | `pipeline_guide.md` — "Executor Configuration" |
| Channel capacity (`TUPA_CHANNEL_CAPACITY`) | Idem |
| Pipeline cancellation | `pipeline_guide.md` — "Pipeline Cancellation" |
| `TUPA_METRICS_OUTPUT` env var | `getting_started.md` — "Engine Configuration" |
| `--metrics-output <FILE>` | `cargo_tupa_guide.md` — `Options` |

---

## Fase 3 — Missing Content P3 ✅

### 3.1 Migration guide

- ✅ `docs/en/guides/migration_guide.md` — Completo (336 linhas)
- ✅ `docs/es/guides/migration_guide.md` — Criado (completo em ES)
- ⚠️ `docs/pt-br/guides/migration_guide.md` — não verificado nesta sessão

### 3.2 Plugin tutorials

| Tutorial | EN | ES | PT-BR |
|----------|--:|--:|------:|
| `plugin-rust.md` | ✅ | ✅ | ✅ |
| `plugin-python.md` | ✅ | ✅ | ⚠️ |

### 3.3 Plugin examples (Fase 3)

| Exemplo | Status |
|---------|--------|
| `crates/tupa-plugin/tests/plugin_src/rust_plugin/` | ✅ Criado (Cargo.toml + src/lib.rs) |
| `crates/tupa-plugin/tests/plugin_src/python_plugin/` | ✅ README criado |
| `tests/integration.rs` compila plugin | ✅ Testes passing |

---

## Fase 4 — Legacy Cleanup P4 ✅

| Item | Status | Nota |
|------|--------|------|
| `spec.md` limpo | ✅ | Única referência `.tp` é a nota de depreciação original |
| `PROPOSAL.md` nota atualizadora | ⚠️ | Pendente de atualização |
| `.kilo/OVERVIEW.md` crate table | ⚠️ | Pendente de atualização |
| `ARCHITECTURE.md` todos idiomas | ✅ | Reescrito na Fase 1 |
| `TRANSITION.md` timeline | ✅ | Reescrito, 2027 removido |
| `OVERVIEW.md` docs EN | ⚠️ | Pendente |

---

## Fase 5 — Validação Final ✅

| Item | Status | Nota |
|------|--------|------|
| Golden tests para fmt/lint | ✅ | `tests/golden/` + testes de idempotência e lint |
| `cargo tupa test` removido da enum | ✅ | `Commands::Test` removido |
| CI local passa | ✅ | fmt, clippy -D, test, parity, golden, lychee |
| `docs-parity-check.sh` | ✅ | `ok` |
| docs-site compila | ✅ | CI local green |
| Observabilidade | ⚠️ | `ExecuçãoResult.metrics: Vec<StepMetrics>` — `lib.rs` declarado, não exportado publicamente |

### Testes Fase 5

```bash
# Unit tests cargo-tupa: 8 passing
cargo test -p cargo-tupa

# Integration test run_metrics: 1 passing
cargo test --test run_metrics

# Warnings: 0 (clippy -D warnings limpou)
```

---

## Riscos & Contingências

| Risco | Probabilidade | Impacto | Contingência |
|-------|---------------|---------|--------------|
| Traduções PT-BR desatualizadas | Alta | Médio | Marcado como ⚠️; ação pós-release |
| `discover` não registrado | Desconhecido | Baixo | Apenas documentar quando oficial |
| PROPOSAL/OVERVIEW pendente | Alta | Baixo | Ação direta — pequena edição |

---

## Critérios de Done — v0.9.4

| Critério | Status |
|----------|--------|
| Engine per-step timeout (TUPA_STEP_TIMEOUT) | ✅ |
| Engine cancellation (Ctrl+C, `Executor::cancel()`) | ✅ |
| `--metrics-output` exportável | ✅ |
| `cargo tupa run` integration test | ✅ |
| Migration guide published | ✅ |
| Plugin tutorials (Rust + Python) completos | ✅ |
| Plugin examples (Rust) | ✅ |
| `cargo tupa test` removido da enum | ✅ |
| CI local passa 100% | ✅ |
| docs-parity-check synced | ✅ |
| CHANGELOGs atualizados | ⚠️ |

---

*Este é o estado real do código e da documentação em 2026-05-15. Usar como snapshot para release 0.9.4.*
