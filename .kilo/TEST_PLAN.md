# Plano de Cobertura de Testes — Tupã 0.9.4 (TDD)

**Data:** 2026-05-15
**Objetivo:** Atingir 100% de cobertura de testes unitários nos 3 crates atualmente sem cobertura:
- `tupa-engine` (0 testes)
- `tupa-core-macros` (1 teste apenas — parsing)
- `tupa-core` (0 testes unitários)

---

## Resumo de Cobertura Atual

| Crate | Unit tests | Doc tests | Target |
|---|---|---|---|
| `cargo-tupa` | ✅ 11 + 1 integ | — | ✅ feito |
| `tupa-plugin` | ✅ 4 + 6 integ | — | ✅ feito |
| `tupa-pyffi` | ✅ 1 | — | ✅ feito |
| **`tupa-engine`** | ❌ **0** | ✅ 2 doc | 🎯 **este plano** |
| **`tupa-core-macros`** | ⚠️ 1 (parse) | ⚠️ 2 ignorados | 🎯 **este plano** |
| **`tupa-core`** | ❌ **0** | ⚠️ 1 ignorado | 🎯 **este plano** |

---

## Estratégia TDD

Para cada crate:
1. **Arquivo de teste** `src/tests.rs` (unit) ou `tests/<nome>.rs` (integration)
2. Cada `#[test]` testa um único comportamento
3. Usar `cargo tarpaulin` ou `cargo llvm-cov` para medir cobertura (opcional)
4. Rodar `ci-local.sh` após cada crate para garantir que nada quebra

---

## tupa-engine — Plano de Testes Unitários

**Arquivo:** `crates/tupa-engine/tests/unit_tests.rs`

### Classe 1 — `ExecutorConfig` (8 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-01 | `new_default` | `ExecutorConfig::new()` retorna zero timeout, capacidade 10000, sem output |
| TC-02 | `with_step_timeout` | `with_step_timeout(Duration::from_secs(30))` define o campo |
| TC-03 | `with_channel_capacity` | `with_channel_capacity(500)` redefine capacidade |
| TC-04 | `with_metrics_output` | `with_metrics_output(path)` armazena o PathBuf |
| TC-05 | `from_env_no_vars` | Sem env vars, retorna defaults |
| TC-06 | `from_env_with_timeout` | `TUPA_STEP_TIMEOUT=30s` define timeout de 30s |
| TC-07 | `from_env_with_capacity` | `TUPA_CHANNEL_CAPACITY=2000` redefine capacidade |
| TC-08 | `from_env_with_metrics` | `TUPA_METRICS_OUTPUT=/tmp/metrics.json` define o caminho |
| TC-09 | `from_env_invalid_timeout` | `TUPA_STEP_TIMEOUT=foo` é ignorado (fallback default) |
| TC-10 | `from_env_invalid_capacity` | `TUPA_CHANNEL_CAPACITY=abc` é ignorado |
| TC-11 | `from_env_all_vars` | Todas as variáveis combinadas |

### Classe 2 — `Executor::new` e `Executor::with_config` (3 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-12 | `new_default` | `Executor::new()` herda defaults de config |
| TC-13 | `with_config_applies_fields` | `with_config` repassa todos campos corretamente |
| TC-14 | `from_env` | `Executor::from_env()` combina `from_env()` de config + novo cancel_token |

### Classe 3 — `parse_duration` (8 testes)

| # | Teste | Entrada | Esperado |
|---|---|---|---|
| TC-15 | `parse_ms` | `"500ms"` | 500ms |
| TC-16 | `parse_s` | `"30s"` | 30s |
| TC-17 | `parse_m` | `"1m"` | 60s |
| TC-18 | `parse_bare_number` | `"5"` | 5s (assume segundos) |
| TC-19 | `parse_zero_ms` | `"0ms"` | 0ms |
| TC-20 | `parse_fractional_ms` | `"1500ms"` | 1500ms |
| TC-21 | `parse_multi_min` | `"5m"` | 300s |
| TC-22 | `parse_invalid` | `"abc"` | erro ParseIntError |

### Classe 4 — `StepState` (3 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-23 | `discriminant_values` | Verifica valores discriminantes dos 5 estados |
| TC-24 | `derive_partial_eq` | `Running == Running`, `Running != Completed` |
| TC-25 | `serialize_variant_name` | `serde_json` serializa como string de variante |

### Classe 5 — `StepMetrics` (4 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-26 | `new_running` | Criação com start_nanos, sem end_nanos, estado Running |
| TC-27 | `completed_filled` | Preenchimento de end_nanos e duration_nanos |
| TC-28 | `serialize_roundtrip` | Ser/de com `serde_json` preserva todos campos |
| TC-29 | `clone_independent` | Clone não compartilha estado |

### Classe 6 — `PipelineResult` (4 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-30 | `default_passes` | `PipelineResult::default()` tem `passed = true` |
| TC-31 | `default_empty_collections` | values/failures/metrics vazios |
| TC-32 | `new_alias` | `PipelineResult::new()` == default |
| TC-33 | `serialize_roundtrip` | Ser/de preserva todos campos |

### Classe 7 — `ConstraintFailure` (3 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-34 | `display_format` | Formato: `constraint failed: X >= 1.0 (actual: 0.5)` |
| TC-35 | `clone` | Clone independente |
| TC-36 | `display_all_ops` | ge/le/eq/ne/gt/lt são exibidos corretamente |

### Classe 8 — `EngineError` discriminantes (7 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-37 | `step_panic_display` | `"Step 'x' panicked: reason"` |
| TC-38 | `constraint_failed_display` | `"Constraint failed: X ge 1.0 (actual 0.0)"` |
| TC-39 | `cycle_detected_display` | `"Dependency cycle detected: unsatisfied steps: a, b"` |
| TC-40 | `step_timeout_display` | `"Step 'x' timed out after 30s"` |
| TC-41 | `other_display` | `"Pipeline execution error: msg"` |
| TC-42 | `cancelled_display` | `"Pipeline cancelled"` |
| TC-43 | `step_timeout_source` | `.source()` contém a mensagem (thiserror) |

### Classe 9 — Execução sequencial `Executor::run` (mock pipeline) (3 testes)

Usa a trait `ExecutorPipeline` e `ParallelPipeline` manualmente.

| # | Teste | Descrição |
|---|---|---|
| TC-44 | `run_empty_pipeline_passes` | Pipeline sem steps, sem constraints → ok |
| TC-45 | `run_passing_constraint` | Step produz 10.0, constraint ge(5.0) → ok |
| TC-46 | `run_failing_constraint` | Step produz 1.0, constraint ge(5.0) → Err(ConstraintFailed) |

### Classe 10 — Execução paralela `run_parallel` (integration via `#[tokio::test]`) (8 testes)

Necessita tokio runtime.

| # | Teste | Descrição |
|---|---|---|
| TC-47 | `run_parallel_empty_passes` | 0 steps → ok |
| TC-48 | `run_parallel_single_step` | 1 step independente → ok |
| TC-49 | `run_parallel_two_independent` | 2 steps independentes são agendados em paralelo |
| TC-50 | `run_parallel_dag_respected` | step B depende de A → B só roda após A |
| TC-51 | `run_parallel_cycle_detected` | DAG com ciclo → Err(CycleDetected) |
| TC-52 | `run_parallel_step_timeout` | Step demora > timeout → Err(StepTimeout) |
| TC-53 | `run_parallel_with_metrics_output` | Arquivo de metrica escrito com campos válidos |
| TC-54 | `run_parallel_produces_defaults` | Sem `produces` declarado, usa nome do step como métrica |

### Classe 11 — Execução paralela com `Cancelled` (3 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-55 | `cancel_during_execution_stops` | Cancel token setado → Err(Cancelled) |
| TC-56 | `cancel_before_execution_fails_immediately` | Cancel pré-definido → Err(Cancelled) |
| TC-57 | `executor_config_from_env_combined` | Múltiplas env vars combinadas corretamente |

**Total `tupa-engine`: 57 testes unitários**

---

## tupa-core-macros — Plano de Testes

**Arquivo:** `crates/tupa-core-macros/src/tests.rs`

### Classe 1 — Parsing de `PipelineInput` (5 testes)

| # | Teste | Descrição |
|---|---|---|
| TM-01 | `parse_simple_pipeline` | ✅ já existe — manter |
| TM-02 | `parse_two_steps` | 2 steps e 2 constraints |
| TM-03 | `parse_no_constraints` | `constraints: []` aceito |
| TM-04 | `parse_step_with_produces` | `produces: ["a", "b"]` dentro do step |
| TM-05 | `parse_step_with_requires` | `requires: ["x"]` dentro do step |
| TM-06 | `parse_step_with_both_metadata` | produces + requires no mesmo step |
| TM-07 | `parse_missing_name_errors` | falta `name:` → syn::Error |
| TM-08 | `parse_missing_input_errors` | falta `input:` → syn::Error |
| TM-09 | `parse_unknown_keyword_errors` | `foo:` → erro |
| TM-10 | `parse_bad_step_keyword_errors` | `func()` em vez de `step()` → erro |

### Classe 2 — Parsing de `ConstraintDecl` (6 testes)

| # | Teste | Descrição |
|---|---|---|
| TM-11 | `parse_ge_constraint` | `metric("x").ge(10)` |
| TM-12 | `parse_le_constraint` | `metric("y").le(5.0)` |
| TM-13 | `parse_eq_constraint` | `metric("z").eq(0.0)` |
| TM-14 | `parse_ne_constraint` | `metric("a").ne(0.0)` |
| TM-15 | `parse_gt_constraint` | `metric("b").gt(1.0)` |
| TM-16 | `parse_lt_constraint` | `metric("c").lt(100.0)` |
| TM-17 | `parse_constraint_int_value` | `.ge(42)` com inteiro |
| TM-18 | `parse_constraint_float_value` | `.ge(0.5)` com float |
| TM-19 | `parse_constraint_missing_metric` | `ge(1.0)` sem `metric()` |
| TM-20 | `parse_constraint_non_numeric` | `.ge("hello")` → erro |
| TM-21 | `parse_constraint_unknown_method` | `metric("x").foo(1.0)` → erro |

### Classe 3 — Parsing de `StepDecl` (4 testes)

| # | Teste | Descrição |
|---|---|---|
| TM-22 | `parse_step_expression` | Corpo é expressão válida |
| TM-23 | `parse_step_without_metadata` | `step("a") { expr }` sem produces/requires |
| TM-24 | `parse_step_missing_parens` | `step "a" { expr }` → erro |
| TM-25 | `parse_step_missing_braces` | `step("a") expr` → erro |
| TM-26 | `parse_unexpected_trailing` | após step, token inválido → erro |

### Classe 4 — Codegen (execute inside `pipeline!` macro via `syn` expansion) (3 testes)

Usar `proc_macro2::TokenStream` e `quote!` para verificar o output expandido.

| # | Teste | Descrição |
|---|---|---|
| TM-27 | `expand_basic_pipeline` | Verificar se estrutura de output contém `impl tupa_core::Pipeline` |
| TM-28 | `expand_produces_default` | Sem `produces`, método retorna `&[step_id]` |
| TM-29 | `expand_produces_explicit` | Com `produces: ["m1"]`, método retorna `&["m1"]` |
| TM-30 | `expand_constraint_op_ge` | ge(0) gera `v >= 0f64` no expandido |
| TM-31 | `expand_constraint_all_ops` | ge/le/eq/ne/gt/lt todos presentes |
| TM-32 | `idempotent_expansion` | expandir duas vezes dá resultado idêntico |

**Total `tupa-core-macros`: 32 testes unitários**

---

## tupa-core — Plano de Testes Unitários

**Arquivo:** `crates/tupa-core/src/tests.rs` (lib-unit)  
**Ou:** `crates/tupa-core/tests/core_types.rs`

### Classe 1 — `Safe<T, C>` (7 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-57 | `new_creates_value` | `Safe::new(42.0)` armazena 42.0 |
| TC-58 | `into_inner_extracts` | `.into_inner()` retorna o valor original |
| TC-59 | `new_and_into_inner_roundtrip` | new → into_inner = valor original |
| TC-60 | `clone_independent` | Clone não compartilha o T interior |
| TC-58bis | `safe_equality` | `Safe::new(v1) == Safe::new(v1)` e `!= Safe::new(v2)` |
| TC-59bis | `different_constraint_markers_are_unrelated` | `Safe<f64, !nan>` == `Safe<f64, !nan>` (mesmo T) |
| TC-60bis | `tens_commutative` | Same T with different C types should work for comparison |

### Classe 2 — `Tensor<T>` (5 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-61 | `new_wraps_value` | `Tensor(42.0)` armazena o valor |
| TC-62 | `into_inner_not_needed` | Acesso direto por `.0` |
| TC-63 | `clone_copies_inner` | `clone()` cria novo Tensor com mesmo valor |
| TC-64 | `tensor_f32` | Funciona com `f32` |
| TC-65 | `tensor_i32` | Funciona com `i32` |

### Classe 3 — Re-exports e compatibilidade de tipos (5 testes)

| # | Teste | Descrição |
|---|---|---|
| TC-66 | `pipeline_macro_importable` | `use tupa_core::pipeline;` compila |
| TC-67 | `serde_json_reexport` | `tupa_core::serde_json::Value` acessível |
| TC-68 | `safe_debug_format` | `format!("{:?}", Safe::new(1.0))` contém o valor |
| TC-69 | `tensor_debug_format` | `format!("{:?}", Tensor(1.0))` contém "Tensor" |
| TC-70 | `safe_copy_trait` | `Safe<f64, C>` é Copy quando `T: Copy` |

### Classe 4 — Integração tipo `Pipeline` trait bounds (2 testes)

Compila uma pipeline mínima e valida que as traits são implementáveis.

| # | Teste | Descrição |
|---|---|---|
| TC-71 | `pipeline_trait_is_object_safe` | `dyn Pipeline` compila (object safety check) |
| TC-72 | `executor_pipeline_from_core` | `impl ExecutorPipeline for T where T: Pipeline` compila |

**Total `tupa-core`: 20 testes unitários**

---

## Cobertura Total Planejada

| Crate | Atual | Após Plano |
|---|---|---|
| `tupa-engine` | 2 doc | **62 unit** ✅ |
| `tupa-core-macros` | 1 unit / 2 doc ignorados | **32 unit** ✅ |
| `tupa-core` | 1 doc ignorado | **19 unit** ✅ |
| **Workspace** | 24 total | **113 novos + 24 existentes = 137** ✅ |

---

## Ordem de Implementação (TDD — Vermelho/Verde/Refatorar)

1. **Ciclo 1: `tupa-engine`** (maior cobertura de risco) ✅ CONCLUÍDA
   1. [x] Criar `tests/unit_tests.rs`
   2. [x] Classe 1 — `ExecutorConfig` (17 testes TC-01..TC-17)
   3. [x] Classe 2 — `Executor` construtores (4 testes TC-18..TC-21)
   4. [x] Classe 4–8 — tipos de valor (StepState, StepMetrics, PipelineResult, ConstraintFailure, EngineError) (25 testes TC-22..TC-47)
   5. [x] Classe 9 — `Executor::run` mock (4 testes TC-39..TC-42)
   6. [x] Classe 10 — `run_parallel` #[tokio::test] (10 testes TC-43..TC-53)

2. **Ciclo 2: `tupa-core-macros`** ✅ CONCLUÍDA
   1. [x] Criar `src/tests.rs`
   2. [x] Classe 1 — PipelineInput parsing (10 testes TM-01..TM-10)
   3. [x] Classe 2 — ConstraintDecl parsing (11 testes TM-11..TM-21)
   4. [x] Classe 3 — StepDecl parsing (5 testes TM-22..TM-26)
   5. [x] Classe 4 — codegen expand (6 testes TM-27..TM-32)

3. **Ciclo 3: `tupa-core`** ✅ CONCLUÍDA
   1. [x] Criar `src/tests.rs`
   2. [x] Classe 1–4 — Safe, Tensor, re-exports, trait bounds (19 testes TC-C57..TC-C75)

4. **Validação final** ✅ CONCLUÍDA
   1. [x] `cargo test --workspace` todos passam (113 tests)
   2. [x] `cargo clippy -D warnings` 0 warnings
   3. [x] `cargo fmt --check` 0 diff
   4. [x] `./scripts/ci-local.sh` local CI completo
   5. [x] Atualizar `.kilo/TEST_PLAN.md` com resultado final (tests adicionados + números)

---

## Critérios de Aceitação

- [x] Todos os testes acima escritos e passando (113 tests)
- [x] `cargo test --workspace` > 0 FAILED = 0
- [x] `cargo clippy -D warnings` output < 10 linhas
- [x] `ci-local.sh` passa sem erros
- [x] Nenhum `#[allow(dead_code)]` adicionado para fingir cobertura
- [ ] Nenhum `#[test]` que sempre passa sem testar nada (snapshot sem asserts)
