# Análise de Estado — Tupã 0.9.4 (2026-05-15)

**Branch:** `release/0.9.4` (após Fases 3–5 concluídas)
**Commit HEAD:** pendente de atualização — Fases 3–5 concluídas
**Workspace:** Limpo

---

## ✅ Implementado e Funcional

### Engine (tupa-engine 0.9.4)

- `StepMetrics` + `StepState` (Running, Completed, Failed, Timeout, Cancelled)
- `Executor::cancel()` + `cancel_token: Arc<AtomicBool>` (cooperative cancellation)
- `ExecutorConfig::from_env()` com `TUPA_STEP_TIMEOUT` e `TUPA_CHANNEL_CAPACITY`
- `parse_duration()` — suporta `ms`, `s`, `m`
- `ExecutionResult::metrics: Vec<StepMetrics>` — coleta timings por passo
- `--metrics-output <FILE>` — exporta métricas como JSON via CLI
- Per-step timeout via `tokio::time::timeout` em workers
- Cancellation check no loop do manager (Ctrl+C via signal handler)
- `Executor::from_env()`, `ExecutorConfig::from_env()`, `Executor::handle()` públicos

**Local:** `crates/tupa-engine/src/lib.rs:527` (StepMetrics), `:542` (StepState), `:181` (cancel), `:86` (from_env)

### CLI (cargo-tupa 0.9.4)

| Subcommand | Status |
|---|---|
| `check` | ✅ build + typecheck via rustc |
| `run` | ✅ executa binário, `--parallel`, `--metrics-output <FILE>` |
| `fmt` | ✅ formata blocos `pipeline!` (indentação 2 espaços) |
| `lint` | ✅ detecta steps duplicados, requires/produces indefinidos, nome/input ausentes |
| `plugin-new` | ✅ scaffold de novo plugin Rust |
| `test` | ❌ **removido** (era alias para `cargo test --examples`) |
| `discover` | ⚠️ módulo existe mas **não registrado** em `Commands` (deferred) |

- `cargo-tupa/src/run.rs:11` — flag `--metrics-output` ativa serialização de `Vec<StepMetrics>` para JSON
- Unit tests: 8 passing (discover, fmt x2, lint x3)
- Integration test: 1 passing (`run_metrics` — `tests/run_metrics.rs`)
- `cargo tupa test` **removido da enum `Commands`** (alineado com DOC_FIX_PLAN Fase 1 – GAP-2)

### Workspace

- 6 crates ativos em versão 0.9.4:
  - `tupa-core-macros` 0.9.4
  - `tupa-core` 0.9.4
  - `tupa-engine` 0.9.4
  - `tupa-plugin` 0.9.4
  - `tupa-pyffi` 0.9.4
  - `cargo-tupa` 0.9.4
- **Legacy `.tp` completamente removido** do workspace

### Plugins (Fase 3 ✓)

- `crates/tupa-plugin/tests/plugin_src/rust_plugin/` — exemplo mínimo `cdylib` com 2 steps
- `crates/tupa-plugin/tests/plugin_src/python_plugin/` — exemplo Python com `tupa-pyffi` (README)
- integração.rs compila plugin de teste (rótulos: `integration_test_plugin`)

### Docs (Fases 3–4 ✓)

- `docs/es/guides/migration_guide.md` — guia de migração completo em ES (novo)
- `docs/es/tutorials/` — pasta criada com `plugin-rust.md` e `plugin-python.md`
- `docs/en/reference/spec.md` — limpo (única referência `.tp` é a nota de depreciação original)
- DOC_REVIEW.md estado desatualizado — não reflete Fases 3–5

### CI

- `./scripts/ci-local.sh` → `All local CI checks passed`
- `docs-parity-check.sh` → `ok`
- `lychee` — 3 `[404]` em `/discussions` (não fatais em modo não-strict)

---

## ❌ Não Implementado / Pendente

| Item | Prioridade | Local / Nota |
|------|-----------|--------------|
| `discover` subcommand no CLI | Baixa | Módulo existe, não registrado na enum `Commands` |
| SCAR (scaffold completo) | Média | `plugin_src/rust_plugin/` é exemplo mínimo, não scaffold CLI |
| CHANGELOGs finalizados | Baixa | Motor e CLI bons; demais crates verificados |
| Observabilidade detalhada | Baixa | Métricas são u64 nanos, não `Instant` |

### Gaps de Documentação (⚠️ Requer atualização)

| Item | Prioridade | Local / Nota |
|------|-----------|--------------|
| `docs/en/OVERVIEW.md` | Alta | Lista crates desatualizados (ver GAP-10 no DOC_FIX_PLAN) |
| `docs/pt-br/OVERVIEW.md` | Média | Idem EN |
| `docs/es/OVERVIEW.md` | Média | Pasta `es/` agora existe |
| DOC_FIX_PLAN.md | Alta | Seção "Fases 3–5" não reflete conclusão |
| STATE_ANALYSIS.md este arquivo desatualizou rapidamente | Alta | Autoreferencial — esta seção, na prática, é snapshots |
| DOC_REVIEW.md | Média | Desatualizado em relação à execução atual |

---

## 📋 Próximos Passos

1. **AtualizarDOC_FIX_PLAN.md** — marcar Fases 3 e 5 como concluídas; Fase 4 parcial
2. **Atualizar DOC_REVIEW.md** — refletir ações já executadas
3. **Revisit arOVERVIEWs** — se existirem em cada idioma
4. **Commit + tag** — `git commit -m "chore(docs): Fases 3–5 concluídas — plugin examples, ES migration guide, golden tests, cargo tupa removido" && git tag v0.9.4 && git push`

---

*Última atualização: 2026-05-15 — Fases 3–5 concluídas; CI local passa.*
