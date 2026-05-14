# Análise de Estado — Tupã 0.9.4 (2026-05-14)

**Branch:** `release/0.9.4`  
**Commit HEAD:** `2021863` — "chore(release): 0.9.4 — engine metrics, cancellation, cargo-tupa matured"  
**Workspace:** Limpo (apenas `.kilo/` não rastreado)

---

## ✅ Implementado e Funcional

### Engine (tupa-engine 0.9.4)

- `StepMetrics` + `StepState` (Running, Completed, Failed, Timeout, Cancelled)
- `Executor::cancel()` + `cancel_token: Arc<AtomicBool>` (cooperative cancellation)
- `ExecutorConfig::from_env()` com `TUPA_STEP_TIMEOUT` e `TUPA_CHANNEL_CAPACITY`
- `parse_duration()` — suporta `ms`, `s`, `m`
- `ExecutionResult::metrics: Vec<StepMetrics>` — coleta timings por passo
- Per-step timeout via `tokio::time::timeout` em workers
- Cancellation check no loop do manager (Ctrl+C via signal handler)

**Local:** `crates/tupa-engine/src/lib.rs:527` (StepMetrics), `:542` (StepState), `:181` (cancel), `:86` (from_env)

### CLI (cargo-tupa 0.9.4)

- `discover` — auto-detecta binário target no Cargo.toml
- `fmt` — formatação básica de blocos `pipeline!` (indentação)
- `lint` — detecta steps duplicados, requires/produces indefinidos, nome/input ausentes
- `run` — executa binário após `cargo build --release --bin <name>`
- 5 unit tests passing (discover, fmt, lint)
- `--parallel` flag (repassa para engine)

**Local:** `crates/cargo-tupa/src/{discover.rs,fmt.rs,lint.rs,run.rs,main.rs}`

### Workspace

- 6 crates ativos em versão 0.9.4:
  - `tupa-core-macros` 0.9.4
  - `tupa-core` 0.9.4
  - `tupa-engine` 0.9.4
  - `tupa-plugin` 0.9.4
  - `tupa-pyffi` 0.9.4
  - `cargo-tupa` 0.9.4
- `Cargo.lock` atualizado
- **Legacy `.tp` completamente removido** (tupa-parser, typecheck, codegen, cli, runtime, effects, audit, conformance, fmt, lint antigos)

---

## ❌ Não Implementado / Pendente

| Item | Prioridade | Local |
|------|-----------|-------|
| `--metrics-output` em `cargo tupa run` | Alta | `crates/cargo-tupa/src/run.rs` |
| Integration test para `cargo tupa run` | Alta | `crates/cargo-tupa/tests/` |
| Migration guide (.tp → Rust-DSL) | Média | `docs/guides/migration_guide.md` |
| Plugin tutorials (Rust + Python) | Média | `docs/tutorials/plugin-*.md` |
| Plugin examples ( Rust + Python ) | Média | `examples/plugins/` |
| Golden tests para fmt/lint | Baixa | `crates/cargo-tupa/tests/golden/` |
| CHANGELOGs 0.9.4 (todos crates) | Alta | `crates/*/CHANGELOG.md` |
| Fix warning: `unused import: Context` | Alta | `crates/cargo-tupa/src/fmt.rs:1` |

---

## 📊 Status dos CHANGELOGs

| Crate | Versão 0.9.4 no Cargo.toml | CHANGELOG tem seção 0.9.4? |
|-------|--------------------------|---------------------------|
| tupa-engine | ✅ 0.9.4 | ✅ Tem seção `## [0.9.4]` (vazia) |
| tupa-plugin | ✅ 0.9.4 | ⚠️ Não verificado |
| tupa-pyffi | ✅ 0.9.4 | ⚠️ Não verificado |
| tupa-core | ✅ 0.9.4 | ⚠️ Não verificado |
| tupa-core-macros | ✅ 0.9.4 | ⚠️ Não verificado |
| cargo-tupa | ✅ 0.9.4 | ✅ Tem seção `## [0.9.4]` (com conteúdo) |

**Ação:** Preencher seções 0.9.4 em todos CHANGELOGs antes do release.

---

## 🧪 Testes Atuais

```bash
cargo test --workspace --locked
# Result: ok
# - Unit tests: cargo-tupa (5 passing)
# - Doc-tests: engine (1 passing), core-macros (2 ignored)
# - Integration tests: 0
```

**Falta:**
- Integration test de `cargo tupa run` (sample pipeline package)
- Testes de timeout/cancellation no engine
- Testes de métricas (StepMetrics)
- Golden tests para fmt/lint output

---

## ⚠️ Bloqueios Imediatos

1. **Cargo-tupa warning** — `fmt.rs` importa `Context` mas não usa:
   ```rust
   // crates/cargo-tupa/src/fmt.rs:1
   use anyhow::{Result, Context}; // Context unused
   ```
   **Fix:** `cargo fix --bin cargo-tupa -p cargo-tupa` ou editar manualmente.

2. **`--metrics-output` não implementado** — Sprint 0.9.4 requer métricas exportáveis.
   - `cargo-tupa/src/run.rs:50` chama `execute_binary()` sem flags
   - `execute_binary()` em `discover.rs` não captura `ExecutionResult.metrics`
   - **Solução:** Adicionar flag `--metrics-output <path>` e serializar `result.metrics` como JSON.

3. **CHANGELOGs incompletos** — Motor e CLI têm rascunho, outros crates não.

4. **Documentação ausente** — migration guide, plugin tutorials, examples.

---

## 🎯 Critérios de "Done" (0.9.4)

**Definition of Done da sprint:**

- [x] Engine per-step timeout (TUPA_STEP_TIMEOUT) — ✅
- [x] Engine cancellation (Ctrl+C, `Executor::cancel()`) — ✅
- [x] StepMetrics coletados internamente — ✅
- [ ] StepMetrics exportáveis via `--metrics-output` — ❌
- [ ] `cargo tupa run` integration test — ❌
- [ ] Migration guide published — ❌
- [ ] Plugin tutorials (Rust + Python) completos — ❌
- [ ] Golden tests para fmt/lint — ❌?
- [ ] CI local passa (fmt, clippy -D, test, goldens, lychee, parity) — ⏳
- [ ] CHANGELOGs atualizados — ❌
- [ ] Todos crates 0.9.4 no crates.io — ⏳ (aguarda tag)

---

## 📋 Próximos Passos (Ordenados)

### Fase 1: Correções Críticas (20 min)

1. **Fix warning no cargo-tupa**
   ```bash
   cd tupalang/crates/cargo-tupa
   cargo fix --bin cargo-tupa -p cargo-tupa
   # Ou editar: remover `Context` de fmt.rs:1
   ```

2. **Atualizar CHANGELOGs**
   - `tupa-engine/CHANGELOG.md` — adicionar 0.9.4: metrics, cancellation, timeout
   - `cargo-tupa/CHANGELOG.md` — já tem, revisar
   - Outros crates: adicionar seção genérica (no breaking changes)

3. **Implementar `--metrics-output`**
   - Adicionar flag em `cargo-tupa/src/run.rs`:
     ```rust
     #[arg(long)]
     metrics_output: Option<PathBuf>,
     ```
   - Modificar `execute_binary()` para retornar `ExecutionResult`
   - Se `metrics_output` fornecido, serializar `result.metrics` para JSON

4. **Integration test para `cargo tupa run`**
   - Criar `crates/cargo-tupa/tests/integration/` com pipeline mínimo
   - Testar auto-discovery, build, execution, métricas

### Fase 2: Documentação (1–2h)

5. **Migration guide** (`docs/guides/migration_guide.md`)
   - Tabela mapeando .tp → Rust-DSL
   - Exemplo completo: fraud_complete.tp → Rust
   - Notas sobre diferenças semânticas

6. **Plugin tutorials**
   - `docs/tutorials/plugin-rust.md` — passo-a-passo Rust
   - `docs/tutorials/plugin-python.md` — Python com `tupa-pyffi`
   - Exemplo funcional em `examples/plugins/`

### Fase 3: Validação Final (30 min)

7. **Golden tests**
   - Verificar se `crates/cargo-tupa/tests/golden/` existe
   - Se não, criar fixtures expected para fmt/lint
   - `cargo tupa fmt --check` + diff vs expected

8. **Executar ci-local**
   ```bash
   cd tupalang
   ./scripts/ci-local.sh
   # Corrigir tudo que falhar
   ```

9. **Commit + tag**
   ```bash
   git add -A
   git commit -m "chore(release): prepare 0.9.4 — metrics, cancellation, CLI matured"
   git tag v0.9.4
   git push --tags
   # GitHub Actions publish workflow dispara
   ```

---

## 🚀 Execução Imediata Sugerida

**Ordem:** Fase 1 (críticas) → Fase 3 (validação) → Fase 2 (docs pode ser post-release, mas idealmente antes)

Vou começar implementando o que falta técnico:

1. Fix warning + CHANGELOGs
2. `--metrics-output`
3. Integration test para `run`
4. Rodar ci-local e corrigir
5. Commit + tag

**Você quer que eu execute essas tarefas agora?**
