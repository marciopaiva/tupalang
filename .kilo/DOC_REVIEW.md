# Documentação — Aderência 0.9.4 (Pós-Fase 3–5)

**Status:** Todas as fases concluídas — aguardando release/tag
**Última atualização:** 2026-05-15

---

## Progresso

### ✅ Fase 1 — Correções Críticas P1 (concluída)

- GAP-1: `fmt`/`lint` docs limpam de `.tp` (EN + PT-BR)
- GAP-2: `cargo tupa test` removido da enum `Commands` — usuários usam `cargo test`
- GAP-4: `discover` não registrado; documentação postergada corretamente

### ✅ Fase 2 — Engine Features P2 (concluída)

| Doc | Engine feature documentada |
|-----|---------------------------|
| `docs/en/guides/pipeline_guide.md` | Executor Configuration (timeout, channel, cancellation, `--metrics-output`) |
| `docs/en/guides/getting_started.md` | Engine Configuration (`TUPA_STEP_TIMEOUT`, `TUPA_METRICS_OUTPUT`) |
| `docs/en/guides/cargo_tupa_guide.md` | `run --metrics-output <FILE>` |

### ✅ Fase 3 — Missing Content P3 (concluída)

| Conteúdo | EN | ES | PT-BR |
|----------|--:|--:|------:|
| `migration_guide.md` | ✅ | ✅ | ⚠️ pendente tradução |
| `plugin-rust.md` tutorial | ✅ | ✅ | ✅ |
| `plugin-python.md` tutorial | ✅ | ✅ | ✅ |
| Plugin examples (Rust) | ✅ `rust_plugin/` | — | — |

### ✅ Fase 4 — Legacy Cleanup P4 (concluída)

| Item | Status |
|------|--------|
| `spec.md` limpo (só nota depreciação) | ✅ |
| `PROPOSAL.md` nota atualizadora EN | ⚠️ não revisado |
| ` TRANSITION.md` timeline EN/PT-BR/ES | ✅ Fase 1 |
| `.kilo/OVERVIEW.md` tabela 6 crates | ✅ atualizado |
| `ARCHITECTURE.md` todos idiomas | ✅ Fase 1 |

### ✅ Fase 5 — Validação Final P5 (concluída)

```bash
 cargo fmt --check          ✅
 cargo clippy -D warnings    ✅
 cargo test --workspace      ✅ (31 testes)
 docs-parity-check           ✅
 golden tests (fmt idempotência, lint) ✅
```

---

## Pendências menores (não bloqueiam 0.9.4)

| Item | Prioridade |
|------|-----------|
| Tradução PT-BR de `migration_guide.md` | Baixa |
| Nota atualizadora em `PROPOSAL.md` (todos idiomas) | Baixa |
| `discover` subcommand CLI | Deferred |
| `DOC_SUMMARY.md` | Auto-create na release |
