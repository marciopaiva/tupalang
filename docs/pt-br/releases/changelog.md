
# Changelog

## Propósito

Registrar mudanças relevantes por versão.

## 0.8.2 (2026-05-08)

- Tema do release: sistema de extensões, plugins e hot reload.
- Referência de planejamento:
  - `.kilo/TUPALANG_EVOLUTION.md`

### Escopo Entregue

- **Built-in Functions (Phase 1)**:
  - `tupa::weighted(score, weight, reason)` — score ponderado com reason
  - `tupa::warn(reason)` — aprovação com aviso
  - `tupa::pass(reason)` — aprovação pura com motivo
  - `tupa::confirm(observed, consecutive, required, reason)` — política de confirmação consecutiva
  - `tupa::cooldown(active, remaining_seconds, reason)` — bloqueio por cooldown temporal
  - Compatibilidade retroativa: chamadas sem prefixo ainda funcionam
- **Schema Registry (Phase 2)**:
  - `SchemaRegistry` em `tupa-codegen/src/schema_registry.rs`
  - Versionamento de schemas com migrações
  - `SchemaDiff` para evolução de tipos
  - Inserção de campos em runtime com warnings de depreciação
- **Hot Reload (Phase 2)**:
  - `Runtime::watch_and_reload()` em `tupa-runtime/src/hot_reload.rs`
  - Observação de arquivos `.tp` via crate `notify`
  - `Runtime::reload_pipeline()` aplica novo plano sem reiniciar
  - Feature flag: `--features hot-reload`
- **Extension API (Phase 3)**:
  - Trait `TupaExtension` em `tupa-runtime/src/extensions.rs`
  - `register()` e `name()` para integração de projetos externos
  - ViperTrade implementa `ViperExtensions` em `vipertrade/services/strategy/src/tupa_extensions.rs`
  - `viper_smart_copy.tp` atualizado para usar prefixo `tupa::`
- **Plugin System (Phase 4)**:
  - Crate `tupa-plugin` com carregamento dinâmico de `.so`/`.dll`
  - Entry points C: `_tupa_plugin_name` e `_tupa_plugin_register`
  - `PluginManager::load_plugin()`, `register_all()`, `list_functions()`
  - `StepFunction` type: `Arc<dyn Fn(Value) -> Result<Value, String> + Send + Sync>`
- **Config DSL (Phase 4)**:
  - Nós `ConfigDecl` e `ConfigField` no parser (`tupa-parser/src/lib.rs`)
  - Sintaxe `config Nome { tipo campo, ... }` como AST de primeira classe
  - Pré-condições declarativas para pipelines
- **Crates atualizadas**:
  - Todas as 10 crates Tupa-Lang para `0.8.2`

### Engenharia e CI Entregues

- Funcionalidades implementadas e validadas no ViperTrade como prova integrada.
- Crate `tupa-plugin` adicionada ao workspace.
- Testes unitários para `ViperExtensions` (name, trailing_status, position_sizing).
- Parity de documentação mantida entre PT-BR e EN.

### Snapshot de Validação (workspace)

- Status do release: tag `v0.8.2` cortada, crates publicados e artefatos standalone liberados.
- Status de validação:
  - docs parity verde
  - markdownlint verde
  - CI verde para mudanças de linguagem e runtime mergeadas
  - CI local do ViperTrade verde contra a linha do release
  - runtime do ViperTrade alinhado com a release oficial do CLI standalone `v0.8.2`

### Débito Técnico

- Publicação no crates.io bloqueada por dependencies `path =` nos manifests.
- Documentação de configuração DSL ainda pode ser expandida com exemplos práticos.
- Hot reload depende de feature flag; padrão desligado para throughput.

## 0.8.1 (2026-03-21)

- Tema do release: suporte a estratégias de produção para sistemas reais de política.
- Referência de planejamento:
  - `docs/pt-br/releases/rfc_v0.8.1_trading_strategy_support.md`

### Escopo Entregue

- Suporte de linguagem e runtime para sistemas de estratégia de produção.
- Melhorias para modelagem declarativa de estratégia:
  - outputs estruturados por step
  - `reason` de primeira classe
  - suporte a score ponderado
  - padrão de input tipado para configuração com records aninhados
  - suporte declarativo a políticas temporais
- Slices de type system e runtime entregues:
  - record types
  - record literals
  - acesso tipado a campos
  - validação de schema no runtime para inputs e outputs estruturados
- Builtins temporais entregues:
  - `confirm(...)`
  - `cooldown(...)`

### Engenharia e CI Entregues

- RFC adicionada em inglês, PT-BR e espanhol para preservar a paridade de docs.
- Paridade de docs mantida durante o ciclo de planejamento e implementação.
- CI local containerizado adicionado para reduzir drift entre host e GitHub Actions.
- Docs e exemplos de trading expandidos com:
  - exemplo de pipeline guiado por configuração
  - exemplo de política temporal
- A integração com o ViperTrade foi usada como prova funcional dos slices da `0.8.1`.

### Snapshot de Validação do Workspace

- Status do release: tag `v0.8.1` cortada, crates publicados e artefatos standalone liberados.
- Status de validação:
  - docs parity verde
  - markdownlint verde
  - CI verde para as mudanças de linguagem e runtime mergeadas
  - CI local do ViperTrade verde contra a linha do release
  - runtime do ViperTrade alinhado com a release oficial do CLI standalone `v0.8.1`

### Débito Técnico

- O acesso tipado à configuração foi resolvido pragmaticamente por meio de `input` estruturado, não por sintaxe dedicada.
- A política temporal continua declarativa na camada de policy; o estado do host continua fora do runtime da linguagem.
- A ergonomia de política reutilizável ainda depende majoritariamente de funções normais e composição explícita de records.

## 0.8.0-rc.5 (2026-03-07)

- Correções de compatibilidade do parser para adoção dos pipelines do ViperTrade:
  - tolera declarações `type` em nível superior
  - tolera declarações `extern fn ...;` em nível superior
  - aceita nomes de step sem aspas (`step(name)`) em pipelines
- Melhoria da documentação de publicação de crates:
  - adicionado `README.md` em todos os crates publicáveis
  - adicionado `readme = "README.md"` em todos os manifests de crates

## 0.8.0 (2026-03-05)

- Tema do release: integração Python controlada e auditável para pipelines de produção.
- Princípio guia: "Integrar sem perder governança - toda chamada Python é rastreada, validada e auditável."

### Escopo Entregue

- Interoperabilidade Python (`tupa-pyffi`) para invocação segura de passos `py:module.func`.
- Resiliência de runtime com circuit breaker e suporte a async/await.
- Fluxo de backtesting com avaliação de PnL/risco e logging de auditoria estruturado.
- Melhorias de validação para shapes de tensores, atributos de pipeline e robustez de parser/typechecker.

### Engenharia e CI Entregues

- CI agora exige convenção de título de PR (`type(scope): subject`) e convenção de mensagem de commit.
- Rotulagem automática de PR por tipo de mudança (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`, `breaking`).
- Release Drafter habilitado com categorização automática.
- Proteção de branch em `main` reforçada:
  - checks obrigatórios (`pr-title-convention`, `commit-message-convention`, `lint`, `test`)
  - exigência estrita de branch atualizada
  - resolução de conversas obrigatória
  - revisão de CODEOWNERS e 1 aprovação obrigatórias
  - descarte de reviews obsoletas habilitado
- CODEOWNERS adicionado para arquivos críticos de governança e workflows.
- Governança de backport implementada:
  - validação de labels `backport-X.Y`
  - criação automática de issue de acompanhamento para PRs mergeadas com label de backport
- Operação de release documentada em `release_guide.md` e `release_cut_checklist.md`.
- Validação local padronizada com `scripts/ci-local.sh` (código + lint de docs/links).

### Snapshot de Validação do Workspace

- Checagem local completa executada em 2026-03-05: `./scripts/ci-local.sh`.
- Resultado: pass (`fmt`, `clippy`, `test`, `markdownlint`, `lychee`).
- Estado do working tree durante a validação: limpo na `main`.

### Débito Técnico

- A validação de convenção de commit ainda depende do contexto de PR; pushes diretos para branches protegidas devem permanecer bloqueados por política.
- Os quality gates de docs são fortes no CI, e a paridade multilíngue de estrutura e versão mais recente já está automatizada; a paridade semântica completa do conteúdo traduzido ainda é manual.
- O workflow de backport cria issues de acompanhamento, mas a automação de cherry-pick de backport ainda não foi implementada.
- As metas de performance estão documentadas, mas não existe dashboard de tendência no CI com histórico de latência e throughput.

## 0.7.0 (2026-02-20)

- Release: engine híbrido com governança nativa de pipelines
- CLI: `tupa run` com `--plan`, `--plan-only`, `--output`
- Runtime: relatório JSON com métricas e restrições (pass/fail), hash de auditoria
- Determinismo: `@deterministic(seed=...)` analisado e seed propagada para o PRNG
- Codegen: `ExecutionPlan` JSON com `steps`, `constraints`, `metrics`, `metric_plans`
- Validação: entrada JSON validada contra `TypeSchema` antes da execução

### Adicionado

- Backend híbrido:
  - ExecutionPlan JSON para pipelines
  - CLI `tupa codegen --format=llvm` emite `.ll` e `.plan.json`
  - Runtime de pipeline (`tupa-runtime`) e comando `tupa run`
- Validador de pipeline:
  - `@deterministic` rejeita `Random`/`Time` (E2005)
  - Restrições com métricas indefinidas (E2006)
- Sem breaking changes

### Desempenho

- Tempo de compilação (exemplo médio): alvo < 200ms
- Status: não benchmarkado explicitamente no CI; acompanhado como meta de produto
- Como medir localmente:
  - Faça build do CLI: `cargo build --quiet`
  - Comandos de benchmark (exemplo):
    - `tupa codegen --format=llvm examples/pipeline/minimal.tp`
    - `tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json`
  - Opcional: use `hyperfine` para benchmark:
    - `hyperfine --warmup 3 'tupa codegen --format=llvm examples/pipeline/minimal.tp' 'tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json'`
  - Condições: Linux, Rust stable (>=1.75), builds release quando aplicável
- Hardware e condições:
  - Linux x86_64, Rust stable, máquina local de dev, cold run
- Referência de teste (imprime tempo):
  - `cargo test -p tupa-cli perf -- --nocapture`
  - Observado localmente: `codegen fraud_complete ~= 1ms`, `run fraud_complete ~= 3ms` (fora do CI, ilustrativo)

## 0.6.0 (2026-02-13)

- Inferência de construtor de enum com genéricos e restrições Safe em variants.
- Padrões de match agora suportam destructuring de construtor com padrões de tupla.
- Uso de binding em guard de match validado no typechecker.
- Diagnósticos de match não exaustivo agora apontam para spans do scrutinee.
- Adicionados testes para restrições de construtor de enum e destructuring/guards de match.
- Protótipo do motor de auditoria com hash determinístico para AST e entradas.
- Comando `tupa audit` no CLI com saída JSON para hashes.
- CLI de auditoria agora usa SHA3-256 e flag `--input`.
- Adicionado suporte a anotações `@safety` no parsing.
- Exemplo de auditoria `fraud_pipeline.tp` alinhado às restrições Safe atuais.
- Aviso `private_interfaces` do typechecker resolvido para `Ty::Enum`.

## 0.5.0 (2026-02-12)

- Conclusão das restrições do typechecker e correções de validação.
- Restrições Safe<string, ...>: diagnósticos para !hate_speech e !misinformation.
- Melhoria de clareza de diagnósticos e revisão de consistência.
- Cobertura de testes expandida com casos negativos.
- Adicionados exemplos de misinformation e goldens para Safe<string, ...>.
- Docs atualizadas com exemplos safe e referências de diagnósticos.
- Docs alinhadas com posicionamento do README e atualizações do roadmap.
- Docs incluem um exemplo rascunho de orquestração de pipeline.
- Plano de release alinhado com o roadmap de governança de pipelines.
- Diagnósticos de match agora apontam para spans de padrão inválido; adicionada cobertura de testes negativos.
- Anotações Safe agora validam restrições base; adicionados exemplos de parâmetros/retorno inválidos.
- Casos negativos de lex/parse e saídas de erro JSON adicionados aos goldens.
- Script de atualização de goldens agora cobre todos os exemplos negativos.

## 0.4.0 (2026-02-11)

- Melhorias no codegen de closures e correções de captura de ambiente.
- Melhorias de restrições no typechecker e melhor inferência de lambdas.
- Atualizações de fluxo do CLI para o pipeline typecheck/codegen.
- SPEC e erros comuns atualizados para o novo comportamento.
- Limpeza de documentação: inglês canônico, índices consolidados e entrada PT-BR.

## 0.3.0 (2026-02-07)

- Suporte a closures com captura real de variáveis (estruturas de ambiente, alocação em heap).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipo Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correções de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos de docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correções de TODOs residuais no codegen para maior robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (ainda sem captura de ambiente).
- Correções de golden tests para casos de erro (mensagens do cargo removidas).

## 0.2.0 (2026-02-06)

- Suporte a closures com captura real de variáveis (estruturas de ambiente, alocação em heap).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipo Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correções de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos de docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correções de TODOs residuais no codegen para maior robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (ainda sem captura de ambiente).
- Correções de golden tests para casos de erro (mensagens do cargo removidas).

## 0.9.6 (2026-06-06)

- Tema da versão: limpeza do legado `.tp` e bump de versão coordenado.

### Escopo Entregue

- **Remoção do legado `.tp`**: removidos todos os fontes de exemplo `.tp` (~100 arquivos) e seus assets de suporte (helpers FFI de Python, entradas JSON, scripts geradores) de `examples/`.
- **Limpeza de goldens**: removidas as saídas golden obsoletas em `examples/expected/` geradas pelo CLI `.tp` descontinuado; mantido apenas o golden de Rust-DSL (`expand_simple_pipeline.txt`).
- **Organização do repositório**: removidos artefatos legados soltos da raiz (`update_golden.py`, `data.json`, `tx.json`, `my_test_plugin.rs`, `my_fixed_plugin.rs`, `integration_test.tupa`, `test_pipe.tupa`, `vipertrade_smoke.plan.json`, `test_find.md`).
- **Exemplos reorganizados**: `examples/` agora contém apenas material Rust-DSL; atualizados `examples/README.md` e `examples/migration/README.md`; removidos os subdiretórios obsoletos `pipeline/`, `production/` e `playground/`.
- **Bump de versão**: todos os crates ativos para 0.9.6 (sem mudanças funcionais ou de API).
- **Docs de features reescritos para Rust DSL**: `features/trading_support.md` (EN/ES/PT-BR) agora reflete os crates atuais com um exemplo executável `pipeline!` + `Executor` e marca explicitamente as funcionalidades do runtime 0.8.2 removidas (backtest, circuit breaker, hot reload, registro de schemas); `governance/audit_engine.md` (ES/PT-BR) substituído por nota de descontinuação apontando para as métricas por passo do `tupa-engine`.
- **READMEs de crates corrigidos** para precisão no crates.io (divergências de API em `tupa-core`, `tupa-pyffi`, `tupa-plugin`, `tupa-engine`; `tupa-lints` reposicionado como constantes string, não lints do rustc).
- **Experimental — constraints em nível de tipo (PoC)**: enforcement real de `Safe<T, C>` via `Constraint`/`ConstraintError`, markers embutidos (`tupa_core::constraints::{NonNan, NonInf, Finite}`), `Safe::try_new`/`new_unchecked`, e uma macro `safe!` que prova `!nan`/`!inf` em expressões `f64` constantes em tempo de compilação (guard em runtime caso contrário). Superfície instável — primeiro passo do roadmap spec→crates.

### Engenharia e CI Entregues

- Corrigido o workflow `examples-golden.yml` para comparar goldens recém-gerados com os versionados (antes comparava o diretório consigo mesmo, mascarando divergências).
- `cargo fmt`, `cargo clippy --workspace --all-targets`, `cargo test --workspace` no verde.

### Snapshot de Validação (workspace)

- Build: `cargo build --workspace` ok.
- Testes: `cargo test --workspace` no verde (167 testes).
- Smoke: `scripts/vipertrade-smoke.sh` ok.
- Goldens: `scripts/update-goldens.sh` não produz diff contra `examples/expected/`.

### Débito Técnico

- Vários docs instrucionais ainda invocam o removido `tupa-cli` (`reference/codegen.md`, `guides/testing.md`, `guides/tutorials.md`, `guides/faq.md`, `governance/issues_guide.md`, `guides/examples_guide.md`); devem ser migrados para fluxos `cargo-tupa` / Rust-DSL em um follow-up. Referências históricas em `ARCHITECTURE.md`, `PROPOSAL.md`, `roadmap.md`, arquivos e changelogs anteriores são intencionais e mantidas como estão.

## 0.9.5 (2026-05-16)

- Tema do release: cobertura de testes, operações Safe/Tensor, paths de cargo-tupa, e estabilidade de tupa-pyffi.

### Escopo Entregue

- **Conclusão da cobertura de testes** (TC-51, TC-54, TC-55, TC-56):
  - Corrigido `tc41_step_panic_display` — removida asserção incorreta `source()`
  - Corrigido `tc46_step_timeout` / `tc52_from_env_timeout_caught_by_executor` — sleep de SlowP alterado de 10ms para 200ms com `spawn_blocking`
  - Corrigido `tc51_no_produces_for_single_step` — `SingleP::produces` retorna array vazio para step desconhecido
- **Novos testes unitários**: 32 testes em `tupa-core-macros/tests.rs` e 30 testes em `tupa-core/src/tests.rs` (TC-C54..TC-C81)
- **Testes de cancelamento de Executor**: TC-55 e TC-56 para comportamento de `Executor::cancel()`
- **Benchmarks criterion**: `engine_bench.rs` com benchmarks de sequencial, paralelo, DAG, constraint, metrics e executor_new
- **Operadores aritméticos Safe**: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` para `Safe<T,C>`
- **Métodos Tensor**: `new()`, `get()`, `into_inner()`, implementação `PartialEq`
- **Melhorias tupa-pyffi**: `call_with_multiple_args()` para chamadas multi-arg, `reset_python_bridge()` para reset de estado global, tipos estendidos (i32, u64, u32, f32, Vec<u8>, Vec<Value>)

### Engenharia e CI Entregues

- 162 testes passando no workspace
- `cargo fmt`, `cargo clippy`, `cargo test --workspace` todos verdes
- Bump de versão para 0.9.5 em todos os crates ativos

## 0.9.0 (2026-05-11)

### Escopo Entregue

- **Nova arquitetura crate-first**: `tupa-core` (macro pipeline! + tipos), `tupa-engine` (executor paralelo), `tupa-plugin` (carregamento dinâmico), `cargo-tupa` (CLI)
- **Execução paralela**: Scheduler DAG baseado em canais com detecção de ciclos (`Executor::run_parallel`)
- **Sistema de constraints**: Verificação em compile-time + runtime com DSL `metric("name").op(valor)`
- **Plugin FFI**: ABI C para registro de step functions (`libloading` + `extern "C"`)
- **Ferramentas de migração**: Exemplos e guias para conversão `.tp` → Rust DSL
- **Paridade de documentação**: EN, ES, PT-BR com links cruzados completos

### Engenharia e CI Entregues

- Workflows CI: lint (clippy, rustfmt), test (workspace), docs-lint (markdownlint, parity, lychee), smoke gate vipertrade
- Golden tests regenerados com `RUSTFLAGS="-Awarnings"` para suprimir warnings de depreciação
- Todos os links relativos quebrados corrigidos (grammar.ebnf, type_semantics, PROPOSAL, TRANSITION, etc.)
- URLs externas atualizadas (caminhos ViperTrade, GitHub Discussions → Issues)
- `tupa-cli` preservado para fluxo `.tp` legacy; `cargo-tupa` para Rust DSL
- Bump de versões: `tupa-core` 0.9.0, `tupa-core-macros` 0.9.0, `tupa-engine` 0.9.0, `tupa-plugin` 0.9.0, `cargo-tupa` 0.9.0, `tupa-template` 0.9.0

### Snapshot de Validação (workspace)

- **Status do release**: Tag `v0.9.0` criada; crates publicados no crates.io (core, engine, plugin, cargo-tupa)
- **Status de validação**:
  - docs parity: verde (todos arquivos necessários presentes em EN/ES/PT-BR)
  - markdownlint: verde
  - link-check (lychee): 0 erros
  - CI: todos jobs passando (lint, test, vipertrade-smoke)
  - ViperTrade smoke gate valida `tupa-cli` check + codegen para `vipertrade_smoke.tp`
- **Crates publicados**: `tupa-core@0.9.0`, `tupa-engine@0.9.0`, `tupa-plugin@0.9.0`, `cargo-tupa@0.9.0`
- **Crates legacy mantidos**: `tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-runtime`, `tupa-effects`, `tupa-audit`, `tupa-fmt`, `tupa-lint` em 0.8.x

### Dívida Técnica

- `tupa-conformance` não publicado (validador SPEC — artifact Phase 0, pode permanecer como dev-dependency)
- `tupa-core-macros` sem CHANGELOG.md (deve ser adicionado)
- `crates/tupa-template` usa path dependencies no Cargo.toml template — precisa de patch para projetos gerados
- PyFFI (`tupa-pyffi`) ainda em 0.8.2 — migração para API 0.9.0 pendente (Phase 3)
- LSP (`tupa-lsp`) não implementado (adiado; rust-analyzer cobre DSL)
- Suite de benchmarks (`criterion`) não criada (Phase 4)
- Alguns itens públicos em `tupa-core`/`tupa-engine` carecem de docs `///` (necessita pass de文档 antes de 1.0)
