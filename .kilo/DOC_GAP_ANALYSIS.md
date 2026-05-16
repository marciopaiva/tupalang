# Gap Analysis: Documentação vs Implementação (v0.9.4)

**Data:** 2026-05-14
**Versão:** 0.9.4
**Status:** Sprint em andamento — correções necessárias antes de considerar release 0.9.4 complete

---

## Metodologia

Varredura de todos os arquivos `.md` sob `tupalang/` comparando:
- Referências a crates/commands removidos
- Features implementadas mas não documentadas
- Links quebrados para arquivos inexistentes
- Inconsistências de versionamento
- Ausência de CHANGELOGs

**Ferramentas:** `grep`, `glob`, leitura direta de arquivos, análise do código-fonte ativo (`crates/`).

---

## GAPS CRÍTICOS (Prioridade P1)

### 🔴 GAP-1: `cargo tupa fmt` e `lint` documentados para `.tp` (inexistente)

**Severidade:** Crítica — guia usuário a commands que não funcionam para `.tp`

**Arquivos afetados:**
- `docs/en/guides/cargo_tupa_guide.md:48` — `# format all .tp files in the package`
- `docs/en/guides/cargo_tupa_guide.md:53-61` — "Lint Rust-DSL pipeline and .tp files"
- `docs/pt-br/TRANSITION.md:110` — `cargo tupa fmt # formats .tp files if still used`
- `docs/es/TRANSITION.md` (equivalente)

**Evidência de contradição:**
- Implementação de `fmt` (`crates/cargo-tupa/src/fmt.rs`) opera em Rust-DSL apenas (parsing `pipeline!` blocks)
- Implementação de `lint` (`crates/cargo-tupa/src/lint.rs`) analisa Rust DSL
- CHANGELOG do `cargo-tupa` v0.9.3 declara: "Removed all legacy `.tp` support (`fmt`, `lint` subcommands for `.tp` files)"
- Legacy crates (`tupa-fmt`, `tupa-lint`) **removidos** do workspace

**Ação necessária:**
```
Editar docs/en/guides/cargo_tupa_guide.md:
  Linha 48: trocar "# format all .tp files" por "# format Rust-DSL pipeline code"
  Linhas 53-61: especificar que lint analisa apenas Rust DSL (pipeline! macros), não .tp
  Adicionar nota: "Legacy .tp file support removed in v0.9.0. Use Rust DSL exclusively."
```

**Traduções:** Aplicar mesma correção em `docs/pt-br/guides/cargo_tupa_guide.md` e `docs/es/guides/cargo_tupa_guide.md`.

---

### 🔴 GAP-2: `cargo tupa test` descrito como subcommand com functionality própria

**Severidade:** Crítica — documentation enganosa sobre comportamento real

**Arquivos afetados:**
- `docs/en/guides/cargo_tupa_guide.md:63-71`

**Texto atual:**
```markdown
### `cargo tupa test`

Runs pipeline unit tests and example validations.

```bash
cargo tupa test                # run all tests
cargo tupa test --example credit_decision  # test specific example
cargo tupa test -- --nocapture  # pass args through to cargo test
```
```

**Realidade (código-fonte):**
- `crates/cargo-tupa/src/main.rs:87` → `Commands::Test { filter } => test_cmd::run(...)`
- Módulo `test_cmd` (não examinado em profundidade, mas pelo help output) é essencialmente um wrapper em torno de `cargo test --examples`
- Não há lógica específica de pipeline testing além do que `cargo test` já faz

**Ação necessária:**
Duas opções:
1. **Remover** a seção inteiramente — `cargo tupa test` é apenas sugar, não adiciona valor documental
2. **Reescrever** para esclarecer que é alias para `cargo test --examples` e remove-lo se for redundante

Recomendo **opção 1** (remover) para evitar confusão. Se houver testes de pipeline específicos (ex: `tests/run_metrics.rs`), estes rodam como parte do workspace `cargo test` normal.

---

### 🔴 GAP-3: `cargo tupa discover` implementado mas não documentado

**Severidade:** Alta — command está funcional mas usuários não sabem que existe

**Evidência:**
- Código: `crates/cargo-tupa/src/discover.rs` existe e é registrado em `main.rs`
- Help output (`cargo-tupa --help`) **não lista** `discover` como subcommand! (apenas: check, run, test, fmt, lint, plugin-new)
- Possivelmente o subcommand `discover` está implementado mas **não exposto** na enum `Commands`? Precisa verificação.

**Ação imediata:** Verificar se `discover` está realmente disponível.

Se disponível:
```
Editar docs/en/guides/cargo_tupa_guide.md:
  Adicionar nova seção após 'Installation':
  ### `cargo tupa discover`
  Auto-detects the binary target in the current package...
```

Se NÃO disponível (não linkado em `Commands`):
- É um work-in-progress — documentação deve ignorar até ser oficialmente parte do CLI

---

## GAPS ALTOS (Prioridade P2)

### 🟡 GAP-4: Engine features v0.9.4 NÃO documentadas nos guias

**Features implementadas** (CHANGELOG `tupa-engine` v0.9.4):
1. `Executor::from_env()` + `ExecutorConfig::from_env()`
2. Per-step timeout via `TUPA_STEP_TIMEOUT` environment variable
3. `StepMetrics` collection (start/end timestamps, state)
4. Pipeline cancellation (`Executor::cancel()`, Ctrl+C handler)
5. `--metrics-output` flag para `cargo tupa run`

**Arquivos que precisam atualização:**
- `docs/en/guides/pipeline_guide.md` — adicionar seção "Executor Configuration" ou "Advanced Engine Features"
- `docs/en/guides/getting_started.md` — exemplo com timeout/metrics
- `docs/en/guides/cargo_tupa_guide.md` — documentar flag `--metrics-output` no comando `run`

**Sugestão de conteúdo:**
```markdown
## Engine Configuration

### Environment Variables

- `TUPA_STEP_TIMEOUT` — maximum duration per step (e.g., "30s", "1m", "500ms")
- `TUPA_CHANNEL_CAPACITY` — bounded channel size (default: 1000)

### Metrics Export

```bash
cargo tupa run --metrics-output metrics.json
```

Produces JSON with per-step timings:
```json
[
  {"step":"risk","start_nanos":...,"end_nanos":...,"state":"Completed"},
  {"step":"limit","start_nanos":..., ...}
]
```

### Cancellation

Handle Ctrl+C gracefully or call `Executor::cancel()` from another thread.
```

---

### 🟡 GAP-5: Migration guide **não existe**

**Status:** Sprint 0.9.4 lista como deliverable status "🚧 In Progress", mas arquivo `docs/guides/migration_guide.md` **não foi encontrado** na varredura.

**Expectativa:** Documento deve conter:
- Tabela sintaxe `.tp` → Rust DSL
- passo-a-passo de migração manual (e/ou ferramenta `tupa-migrate` se existir)
- exemplos before/after
- validação pós-migração

**Ação:** Criar `docs/guides/migration_guide.md` (e traduções `pt-br/`, `es/`).

---

### 🟡 GAP-6: Plugin tutorials **não existem**

**Status:** Sprint 0.9.4 lista "Plugin tutorials (Rust + Python)" mas diretório `docs/tutorials/` **não existe**.

**Ação:**
1. Criar diretório: `docs/en/tutorials/`
2. Criar `plugin-rust.md` — como escrever plugin Rust (cdylib, `_tupa_plugin_name`, `_tupa_plugin_register`)
3. Criar `plugin-python.md` — como escrever plugin Python (PyO3, `tupa-pyffi`)
4. Criar exemplos funcionais em `crates/tupa-plugin/tests/plugin_src/` ou `examples/plugins/`
5. Traduzir para `pt-br` e `es`

---

## GAPS MÉDIOS (Prioridade P3)

### 🟠 GAP-7: Timeline obsoleta no TRANSITION.md

**Arquivo:** `docs/en/TRANSITION.md:5`

```
**Timeline:** Legacy `.tp` compilation will be supported until **2027-01-01**.
```

**Problema:** O toolchain `.tp` foi **removido** em v0.9.0 (maio 2026). Não há suporte. A linha do tempo fictitious.

**Ação:** Reescrever seção de timeline para refletir realidade:
- Legacy `.tp` **não é mais suportado**
- Foco total em Rust DSL
- Remover tabela de datas 2027 (ou marcar como obsoleto)

---

### 🟠 GAP-8: PROPOSAL.md menciona crates que não existem mais

**Arquivos:** `docs/en/PROPOSAL.md:47-50`

**Trecho problemático:**
```markdown
tupa-runtime   ← runtime primitives (already exists)
tupa-fmt       ← formatter (already exists)
tupa-lint      ← linter (already exists)
tupa-audit     ← audit/hash (already exists)
tupa-conformance ← SPEC validator (standalone bin)
```

**Realidade:** Esses crates **não existem** como crates independentes no workspace atual:
- `tupa-runtime` foi incorporado ao `tupa-core`
- `tupa-fmt` e `tupa-lint` foram removidos; funcionalidade migrada para `cargo-tupa`
- `tupa-audit` não é crate ativo (funcionalidade no `tupa-engine` ou removida)
- `tupa-conformance` foi removido (CONFORMANCE suite integrada ou arquivada)

**Ação:** Adicionar nota no topo do PROPOSAL.md:
> **Nota:** Este documento descreve a proposta arquitetural original. A implementação atual (v0.9.x) consolidou funcionalidades em 6 crates ativos. Consulte `ARCHITECTURE.md` para arquitetura corrente.

E remover ou riscar referências a crates inexistentes.

---

### 🟠 GAP-9: Exemplos `.tp` ainda no `spec.md` e outros docs

**Arquivos:** `docs/en/reference/spec.md` contém inúmeras referências a arquivos `.tp`:
- Linha 238: `load("digit.tp")`
- Linha 241: `load("llama3.tp")`
- Linha 655: `// math.tp`
- Linhas 827, 893, 904, 910, 946, 965, 1009, 1032, 1042 — todos exemplos com `.tp`

**Problema:** SPEC é normativo e deve refletir **Rust DSL only**. `.tp` foi removido.

**Ação:**
1. Converter todos os exemplos `.tp` para Rust DSL (usando `pipeline!` macro)
2. Mudar extensões de arquivos referenciados para `.rs` ou remover extensão completamente
3. Atualizar code blocks para syntax Rust válida

**Exemplo de conversão:**
```tupa
// math.tp
fn add(a: i32, b: i32): i32 { a + b }
```
↓
```rust
// math.rs (module)
fn add(a: i32, b: i32) -> i32 { a + b }
```
ou incorporar diretamente no exemplo do pipeline.

---

### 🟠 GAP-10: Crate list desatualizada no OVERVIEW.md (.kilo)

**Arquivo:** `.kilo/OVERVIEW.md:24-33`

**Tabela atual:**
```
| `tupa-fmt` | ✅ Stable |
| `tupa-lint` | ✅ Stable |
| `tupa-audit` | ✅ Stable |
| `tupa-conformance` | ✅ Stable |
```

**Problema:** Esses crates foram removidos. A tabela deve listar apenas crates ativos.

**Ação:** Atualizar para:
```
| Crate | Purpose | Status |
| tupa-core-macros | Procedural macro implementation | 🚀 Alpha |
| tupa-core | DSL macros + policy types | 🚀 Alpha |
| tupa-engine | Pipeline executor | 🚀 Alpha |
| tupa-plugin | Dynamic step loading | 🚀 Alpha |
| tupa-pyffi | Python bindings | 🚀 Alpha |
| cargo-tupa | CLI tool | 🚀 Alpha |
```

---

### 🟠 GAP-11: CHANGELOGs ausentes em alguns crates?

**Verificação:** Todos os 6 crates ativos possuem `CHANGELOG.md` (conforme varredura). Nenhuma ação necessária.

**Nota:** CHANGELOGs precisam conter entrada 0.9.4 completa — verificar se estão alinhados com implementação.

---

## GAPS BAIXOS (Prioridade P4)

### 🟡 GAP-12: `docs/en/guides/cargo_tupa_guide.md` — comando `test` mal explicado

Já contemplado no GAP-2. Pode ser resolvido em conjunto.

---

## CHECKS ADICIONAIS

### Links quebrados

**Rodar:** `lychee` (já parte do `ci-local.sh`) para detectar:
- Referências a arquivos `.tp` que não existem mais em `examples/`
- Links internos quebrados (ex: `guides/migration_guide.md` que não existe)

**Ação consertar:** Eliminar ou corrigir links quebrados.

### Paridade EN/PT-BR/ES

**Rodar:** `./scripts/docs-parity-check.sh`

Garantir que correções aplicadas em `docs/en/` sejam refletidas em `docs/pt-br/` e `docs/es/`.

---

## PLANO DE CORREÇÃO (sequência recomendada)

### Fase 1 — Critical Fixes (1–2 dias)

1. **GAP-1** — `cargo_tupa_guide.md`: remover `.tp` de fmt/lint (todos os idiomas)
2. **GAP-2** — Decidir fate de `cargo tupa test` section (remover ou reescrever)
3. **GAP-4** — Adicionar `discover` command documentation (se command realmente exposto)
4. **Executar ci-local.sh** — validar que mudanças não quebram docs-parity/lychee

**Entregável:** docs do CLI alinhadas com implementação 0.9.4

---

### Fase 2 — Engine Features Coverage (2–3 dias)

5. **GAP-3** — Adicionar seção "Engine Configuration" em:
   - `pipeline_guide.md` (timeout, metrics, cancellation)
   - `getting_started.md` (exemplo mínimo expandido)
   - `cargo_tupa_guide.md` (flag `--metrics-output` em `run`)
6. Atualizar CHANGELOGs se necessário para refletir 0.9.4 features (já estão ok?)

**Entregável:** Engine enhancements documentadas

---

### Fase 3 — Missing Content Creation (3–4 dias)

7. **GAP-5** — Criar `docs/guides/migration_guide.md` (e traduções)
   - Template baseado em `TRANSITION.md` mas focado em "how-to" prático
   - Mapeamento sintaxe: tabela comparativa `.tp` → Rust DSL
   - Checklist de validação pós-migração

8. **GAP-9** — Criar `docs/tutorials/` (e traduções)
   - `plugin-rust.md` — passo-a-passo: projeto, export symbols, carregamento
   - `plugin-python.md` — usando `tupa-pyffi` ou PyO3 direto
   - Acompanhar exemplos em `crates/tupa-plugin/tests/` ou `examples/`

**Entregável:** Migration guide e plugin tutorials publicados

---

### Fase 4 — Legacy Cleanup (1–2 dias)

9. **GAP-7** — Limpar `spec.md` de exemplos `.tp`
   - Substituir todos `*.tp` por `*.rs` ou código inline
   - Garantir exemplos compiláveis (Rust syntax válida)

10. **GAP-8** — Nota de rodapé em `PROPOSAL.md` sobre descontinuidade

11. **GAP-10** — Atualizar `.kilo/OVERVIEW.md` crate table

**Entregável:** Documentação livre de referências a toolchain legado

---

### Fase 5 — Validação Final

12. Rodar `./scripts/ci-local.sh` — todos checks devem passar
13. Verificar `lychee` links (0 broken)
14. Verificar `markdownlint` (0 warnings)
15. Verificar docs-parity (EN/PT-BR/ES sync)

---

## RISCOS DO PLANO

| Risco | Mitigação |
|-------|-----------|
| Escopo muito grande (muitos arquivos) | Focar em P1 primeiro; P2-P4 podem ser postergados se necessário |
| Traduções pt-br/es desatualizadas | Aplicar patches em todos idiomas simultaneamente |
| `spec.md` extenso (1083 linhas) — conversão manual tediosa | Buscar-and-replace sistemático; validar syntax com `rustc` em exemplos |
| Possível dependência de ferramenta `tupa-expand` (não existe) | Se não existir, documentar apenas macro expansion conceitual |

---

## DECISÕES ABERTAS

1. **GAP-2:** `cargo tupa test` — remover section ou manter com nota "wrapper for cargo test --examples"?
   - **Recomendação:** Remover. Não agrega valor.

2. **GAP-4:** `discover` command — está realmente funcional? Precisa verificar código.
   - Se não estiver linkado em `Commands`, documentação deve esperar até ser oficial.

3. **GAP-7:** `spec.md` examples — converter todos ou apenas marked as `.tp`?
   - **Recomendação:** Todos os exemplos que usam extensão `.tp` devem ser convertidos, pois spec é normativo para Rust-DSL.

4. **GAP-5 (migration guide):** Criar tool `tupa-migrate` ou guia manual?
   - **Recomendação:** Guia manual + tabela de mapeamento. Ferramenta automática pode ser futura.

---

## CHECKLIST DE IMPLEMENTAÇÃO

```
[ ] GAP-1: cargo_tupa_guide.md — fmt/lint only Rust DSL (EN + PT-BR + ES)
[ ] GAP-2: Decisão sobre cargo tupa test section
[ ] GAP-4: discover command documented (ou ignorar se não oficial)
[ ] GAP-3: Engine config (timeout, metrics, cancellation) em pipeline_guide.md
[ ] GAP-3: --metrics-output em cargo_tupa_guide.md (run command)
[ ] GAP-5: Criar migration_guide.md (e traduções)
[ ] GAP-9: Criar docs/tutorials/plugin-*.md (e traduções) + exemplos
[ ] GAP-7: spec.md — limpar todos exemplos .tp
[ ] GAP-8: PROPOSAL.md — nota de descontinuidade
[ ] GAP-10: .kilo/OVERVIEW.md — tabela crates atualizada
[ ] ci-local.sh passa sem erros
[ ] lychee 0 broken links
[ ] docs-parity-check sync across languages
```

---

## ANEXO: Evidence Log

- **Code inspection:** `crates/cargo-tupa/src/main.rs` (commands enum), `fmt.rs`, `lint.rs`
- **CHANGELOG verification:** `crates/cargo-tupa/CHANGELOG.md`, `crates/tupa-engine/CHANGELOG.md`, `crates/tupa-core/CHANGELOG.md`
- **Workspace inventory:** `crates/` listing (only 6 active crates)
- **Version check:** All crates at 0.9.4
- **Documentation grep:** 344 matches for `\.tp\b` across `docs/` — many are valid (transition guide), many are obsolete (spec examples, fmt/lint docs)

---

** Próxima ação:** Aprovar este plano e iniciar **Fase 1** (GAP-1, GAP-2, GAP-4).
