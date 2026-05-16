# Análise de Estado — Tupã 0.9.4 (2026-05-15, atualizado pós‑fix)

**Branch:** `release/0.9.4` (Fases 3–5 concluídas)
**Workspace:** Limpo (não é git repo; snapshot local)
**Última verificação CI:** `ci-local.sh` → `All local CI checks passed` ✓

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

**Local:** `crates/tupa-engine/src/lib.rs`

### CLI (cargo-tupa 0.9.4)

| Subcommand | Status |
|---|---|
| `check` | ✅ build + typecheck via rustc |
| `run` | ✅ executa binário, `--parallel`, `--metrics-output <FILE>` |
| `fmt` | ✅ formata blocos `pipeline!` (indentação 2 espaços) |
| `lint` | ✅ detecta steps duplicados, requires/produces indefinidos, nome/input ausentes |
| `plugin-new` | ✅ scaffold de novo plugin Rust |
| `expand` | ✅ expande `pipeline!` macro, `--pretty` |
| `discover` | ✅ registrado na enum `Commands`, impl em `discover.rs` |
| `test` | ❌ **removido** da enum `Commands`; módulo `test_cmd.rs` existe mas não é importado |

- Unit tests: **142 total** (cargo-tupa 15 + tupa-plugin 4 + tupa-pyffi 1 + tupa-engine 62 + tupa-core-macros 32 + tupa-core 19 + diagnostics 1)
- Integration tests: **6 passing** (tupa-plugin integration)
- Fn name collision fix em `expand.rs`: função interna renomeada de `expand_pipeline_block` → `generate_pipeline_impl`
- Assertion fix: `contains("impl tupa_core::Pipeline")` → `contains("tupa_core :: Pipeline")` (reflete saída de `TokenStream::to_string`)

### Workspace

- **7 crates no Cargo.toml** (sendo 6 ativos em 0.9.4):
  - `tupa-core-macros` 0.9.4
  - `tupa-core` 0.9.4
  - `tupa-engine` 0.9.4
  - `tupa-plugin` 0.9.4
  - `tupa-pyffi` 0.9.4
  - `cargo-tupa` 0.9.4
  - `tupa-template` 0.9.0 — **template** de crate de pipeline (não builda diretamente; usa placeholders `{{crate_name}}`, `{{authors}}`)
- **Legacy `.tp` completamente removido** do workspace

### CI

| Check | Result |
|---|---|
| `cargo fmt --check` | ✅ 0 diff |
| `cargo clippy -D warnings` | ✅ 0 warnings |
| `cargo test --workspace` | ✅ **142 tests pass** (unit: 133 + integ: 6 + doc: 2) |
| `markdownlint` | ✅ ok |
| `docs-parity-check.sh` | ✅ ok |
| `vipertrade-smoke.sh` | ✅ ok |
| `lychee` | ⚠️ 4 `[404]` em `/discussions` (non-strict, acceptable) |
| golden check | ⚠️ skip (tupa-cli removido; cascade pattern) |

---

## ❌ Não Implementado / Pendente

| Item | Prioridade | Local / Nota |
|------|-----------|--------------|
| `tupa-template` versão para 0.9.4 | Baixa | Constraint pins `=0.9.0` no próprio `Cargo.toml` do template |
| CHANGELOGs finais | Baixa | `CHANGELOG.md` de crates core coerentes |
| Observabilidade com `Instant` | Baixa | Métricas são u64 nanos, não timestamps com fusos |
| `discover` test com Cargo.toml complexo | Baixa | Apenas binário simples é testado atualmente |
