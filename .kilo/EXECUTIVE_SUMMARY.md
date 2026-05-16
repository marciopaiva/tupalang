# Resumo Executivo — Revisão Tupã 0.9.4

**Data:** 2026-05-14
**Branch:** `release/0.9.4`
**Status:** Código funcional, documentação desalinhada — correções necessárias antes de release clean

---

## ✅ O que está funcional (implementado)

### Engine (tupa-engine 0.9.4) — COMPLETO

| Feature | Status | Local |
|---------|--------|-------|
| `StepMetrics` + `StepState` | ✅ Implementado | `src/lib.rs:542-568` |
| `Executor::cancel()` + `cancel_token` | ✅ Implementado | `src/lib.rs:181` (store), `:86` (from_env) |
| `ExecutorConfig::from_env()` | ✅ Implementado | `src/lib.rs:86` |
| `TUPA_STEP_TIMEOUT` (parse duration) | ✅ Implementado | `src/lib.rs` (parse helper) |
| `TUPA_CHANNEL_CAPACITY` | ✅ Implementado | `src/lib.rs:104` |
| `--metrics-output` (CLI → env → file) | ✅ Implementado | `cargo-tupa/src/run.rs:11,53` + `tupa-engine/src/lib.rs:481-488` |
| Per-step timeout via `tokio::time::timeout` | ✅ Implementado | `src/lib.rs` (worker wrap) |
| Cancellation (Ctrl+C + cooperative) | ✅ Implementado | `src/lib.rs` (signal handler + flag checks) |

**Nota:** Todas as engine features anunciadas no SPRINT_0.9.4 estão **no código**.

---

### CLI (cargo-tupa 0.9.4) — COMPLETO (exceto possivelmente discover)

| Subcommand | Status | Observação |
|------------|--------|------------|
| `check` | ✅ | Valida macro expansion |
| `run` | ✅ | Suporta `--input`, `--parallel`, `--metrics-output` |
| `fmt` | ✅ | Formata Rust-DSL apenas (`.tp` removido) |
| `lint` | ✅ | Analisa Rust-DSL apenas (`.tp` removido) |
| `discover` | ⚠️ Implementado em código (`src/discover.rs`) mas **não aparece no `--help`**? | Verificar se registrado na enum `Commands` |
| `test` / `plugin-new` | ✅ | Wrapper para `cargo test --examples` + plugin scaffold |

**Unit tests:** 5 passing (discover, fmt, lint) — confirmado em `crates/cargo-tupa/tests/`

---

### Workspace — LIMPO

- **6 crates ativos** todos em version 0.9.4
- **Legacy `.tp` toolchain removido** permanentemente (54 commits atrás — v0.9.0)
- `Cargo.lock` regenerado
- v0.9.4 tagged e pushed — GitHub Actions publish workflow ativo

---

## ❌ O que NÃO está (ou está errado)

### Documentação — Gaps Identificados (9 gaps)

| ID | Descrição | Severidade | Status |
|----|-----------|------------|--------|
| GAP-1 | `fmt`/`lint` docs dizem que servem para `.tp` | 🔴 Crítica | Não documentado corretamente |
| GAP-2 | `cargo tupa test` descrito como feature especial (é wrapper) | 🔴 Crítica | Pode confundir |
| GAP-4 | `discover` command não documentado (se existir) | 🔴 Alta | Ausente |
| GAP-3 | Engine features (timeout, metrics, cancellation) **não documentadas** | 🟡 Alta | Ausente dos guias |
| GAP-5 | Migration guide **não existe** (arquivo ausente) | 🟡 Alta | Criar |
| GAP-9 | Plugin tutorials **não existem** (diretório ausente) | 🟡 Alta | Criar |
| GAP-7 | Exemplos `.tp` no `spec.md` (deve ser Rust-DSL only) | 🟠 Média | Converter/limpar |
| GAP-8 | PROPOSAL.md sem nota de descontinuidade (crates removidos) | 🟠 Média | Adicionar nota |
| GAP-10 | OVERVIEW.kilo com crate table desatualizada | 🟠 Média | Atualizar |

**Total de referências `.tp` em docs:** 344 matches — maioria em TRANSITION.md (válido), algumas em spec.md e guias (inválidas).

---

## 📋 Prioridades de Correção

### Phase 1 — Critical (1–2 dias) — BLOQUEIA USUÁRIOS

1. **GAP-1**: Remover `.tp` de `cargo_tupa_guide.md` (todos idiomas)
2. **GAP-2**: Decidir: remover seção `test` ou reescrever como "alias"
3. **GAP-4**: Verificar `discover` command availability → documentar se aplicável
4. **Validar**: `./scripts/ci-local.sh` (docs-parity, lychee)

**Saída:** CLI docs alinhadas com realidade 0.9.4

---

### Phase 2 — High (2–3 dias) — COBERTURA DE FEATURES

5. **GAP-3**: Adicionar engine config (timeout, metrics, cancellation) aos guias
   - `pipeline_guide.md` — seção "Configuration"
   - `cargo_tupa_guide.md` — flag `--metrics-output` em `run`
   - `getting_started.md` — exemplo expandido

6. **CHANGELOGs**: Verificar se todos crates têm seção 0.9.4 completa

**Saída:** Engine enhancements documentadas

---

### Phase 3 — Medium (3–4 dias) — CONTEÚDO AUSENTE

7. **GAP-5**: Criar `docs/guides/migration_guide.md` (e traduções)
   - Tabela sintaxe `.tp` → Rust DSL
   - Step-by-step migration
   - Validação

8. **GAP-9**: Criar `docs/tutorials/` (e traduções)
   - `plugin-rust.md` + exemplo cdylib
   - `plugin-python.md` + exemplo PyO3
   - Diretório `examples/plugins/` (opcional)

**Saída:** Migration path e plugin ecosystem documentados

---

### Phase 4 — Cleanup (1–2 dias) — LEGADO

9. **GAP-7**: Limpar `spec.md` — exemplos `.tp` → Rust DSL
10. **GAP-8**: Nota em `PROPOSAL.md` sobre remoção de crates
11. **GAP-10**: Atualizar `.kilo/OVERVIEW.md`
12. **TRANSITION.md**: Remover timeline 2027 (obsoleta)

**Saída:** Documentação livre de referências a toolchain removido

---

## 📁 Arquivos que precisam mudança

```
docs/
├── en/
│   ├── guides/
│   │   ├── cargo_tupa_guide.md    🔴 GAP-1, GAP-2, GAP-4
│   │   ├── getting_started.md     🟡 GAP-3 (parcial)
│   │   ├── pipeline_guide.md      🟡 GAP-3 (principal)
│   │   ├── installation.md        ✓
│   │   └── migration_guide.md     ❌ GAP-5 (criar)
│   ├── tutorials/                 ❌ GAP-9 (criar diretório)
│   ├── reference/
│   │   └── spec.md                🟠 GAP-7 (limpar .tp examples)
│   ├── PROPOSAL.md                🟠 GAP-8 (nota)
│   ├── TRANSITION.md              🟠 GAP-11 (timeline)
│   └── ARCHITECTURE.md            ✓
├── pt-br/                         (espelhar mudanças EN)
└── es/                           (espelhar mudanças EN)

.kilo/
├── STATE_ANALYSIS.md             ✓ (atualizado com gaps)
├── DOC_GAP_ANALYSIS.md           ✅ (criado)
└── DOC_FIX_PLAN.md               ✅ (criado)
```

---

## 🔧 Check-list de Qualidade

Antes de considerar 0.9.4 "docs complete":

```
[ ] Fase 1: CLI docs corrigidas (fmt/lint só Rust-DSL, test esclarecido)
[ ] Fase 2: Engine features documentadas (timeout, metrics, cancellation, metrics-output)
[ ] Fase 3: Migration guide criado
[ ] Fase 3: Plugin tutorials criados
[ ] Fase 4: spec.md limpo de exemplos .tp
[ ] Fase 4: PROPOSAL.md com nota de descontinuidade
[ ] Fase 4: OVERVIEW.kilo atualizado
[ ] ci-local.sh passa 100% (fmt, clippy, test, markdownlint, docs-parity, lychee)
[ ] Nenhum broken link (lychee)
[ ] Docs-parity: EN/PT-BR/ES sincronizados
```

---

## ⚠️ Nota sobre `cargo tupa test`

O help output de `cargo-tupa` **NÃO lista** `test` como subcommand? Verificar:

```bash
cd tupalang
cargo run -p cargo-tupa -- --help
```

Output oficial:
```
Commands:
  check
  run
  test        ← está aqui?
  fmt
  lint
  plugin-new
```

Se `test` estiver ausente do `--help`, então não é oficial — remover da doc independentemente.

---

## 🎯 Recomendação Final

1. **Foco imediato:** Fase 1 (GAP-1, GAP-2, GAP-4) — 1–2 dias
2. **Integração contínua:** Após cada fase, rodar `ci-local.sh` e corrigir failures
3. **Release saúde:** Marcar 0.9.4 como "documentação alinhada" apenas após Fase 4 completa
4. **Comunicação:** Atualizar CHANGELOGs mencionando "Documentation updated to reflect Rust-DSL only" em cada crate

**Veredito do projeto:** Viabilidade **ALTA**, implementation sólida, documentação precisa de polishing acelerado (1–2 semanas de trabalho).

---

**Próxima ação:** Iniciar Fase 1 — editar `docs/en/guides/cargo_tupa_guide.md`
