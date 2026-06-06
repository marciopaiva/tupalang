
# Changelog

## Propósito

Registrar cambios relevantes por versión.

## 0.8.2 (2026-05-08)

- Tema del release: sistema de extensiones, plugins y hot reload.
- Referencia de planificación:
  - `.kilo/TUPALANG_EVOLUTION.md`

### Alcance Entregado

- **Built-in Functions (Phase 1)**:
  - `tupa::weighted(score, weight, reason)` — score ponderado con reason
  - `tupa::warn(reason)` — aprobación con advertencia
  - `tupa::pass(reason)` — aprobación pura con razón
  - `tupa::confirm(observed, consecutive, required, reason)` — política de confirmación consecutiva
  - `tupa::cooldown(active, remaining_seconds, reason)` — bloqueo por cooldown temporal
  - Compatibilidad retroactiva: llamadas sin prefijo aún funcionan
- **Schema Registry (Phase 2)**:
  - `SchemaRegistry` en `tupa-codegen/src/schema_registry.rs`
  - Versionado de schemas con migraciones
  - `SchemaDiff` para evolución de tipos
  - Inserción de campos en runtime con advertencias de deprecación
- **Hot Reload (Phase 2)**:
  - `Runtime::watch_and_reload()` en `tupa-runtime/src/hot_reload.rs`
  - Observación de archivos `.tp` via crate `notify`
  - `Runtime::reload_pipeline()` aplica nuevo plan sin reiniciar
  - Feature flag: `--features hot-reload`
- **Extension API (Phase 3)**:
  - Trait `TupaExtension` en `tupa-runtime/src/extensions.rs`
  - `register()` y `name()` para integración de proyectos externos
  - ViperTrade implementa `ViperExtensions` en `vipertrade/services/strategy/src/tupa_extensions.rs`
  - `viper_smart_copy.tp` actualizado para usar prefijo `tupa::`
- **Plugin System (Phase 4)**:
  - Crate `tupa-plugin` con carga dinámica de `.so`/`.dll`
  - Entry points C: `_tupa_plugin_name` y `_tupa_plugin_register`
  - `PluginManager::load_plugin()`, `register_all()`, `list_functions()`
  - `StepFunction` type: `Arc<dyn Fn(Value) -> Result<Value, String> + Send + Sync>`
- **Config DSL (Phase 4)**:
  - Nodos `ConfigDecl` y `ConfigField` en parser (`tupa-parser/src/lib.rs`)
  - Sintaxis `config Nombre { tipo campo, ... }` como AST de primera clase
  - Pre-condiciones declarativas para pipelines
- **Crates actualizadas**:
  - Todas las 10 crates Tupa-Lang para `0.8.2`

### Ingeniería y CI Completados

- Features implementadas y validadas en ViperTrade como prueba integrada.
- Crate `tupa-plugin` agregada al workspace.
- Tests unitarios para `ViperExtensions` (name, trailing_status, position_sizing).
- Paridad de documentación mantenida entre PT-BR, EN y ES.

### Snapshot de Validación (workspace)

- Estado del release: tag `v0.8.2` cortado, crates publicados y artefactos standalone liberados.
- Estado de validación:
  - docs parity verde
  - markdownlint verde
  - CI verde para cambios de lenguaje y runtime mergeados
  - CI local de ViperTrade verde contra la línea del release
  - runtime de ViperTrade alineado con la release oficial del CLI standalone `v0.8.2`

### Deuda Técnica

- Publicación en crates.io bloqueada por dependencies `path =` en manifests.
- Documentación de Config DSL aún puede expandirse con ejemplos prácticos.
- Hot reload depende de feature flag; predeterminado desactivado para throughput.

## 0.8.1 (2026-03-21)

- Tema del release: soporte para estrategias de produccion en sistemas reales de politica.
- Referencia de planificacion:
  - `docs/es/releases/rfc_v0.8.1_trading_strategy_support.md`

### Alcance Entregado

- Soporte de lenguaje y runtime para sistemas de estrategia de produccion.
- Mejoras para modelado declarativo de estrategia:
  - salidas estructuradas por step
  - `reason` de primera clase
  - soporte para score ponderado
  - patron de input tipado para configuracion con records anidados
  - soporte declarativo para politicas temporales
- Slices de type system y runtime entregados:
  - record types
  - record literals
  - acceso tipado a campos
  - validacion de schema en runtime para inputs y outputs estructurados
- Builtins temporales entregados:
  - `confirm(...)`
  - `cooldown(...)`

### Ingeniería y CI Completados

- RFC agregada en ingles, PT-BR y espanol para preservar la paridad de docs.
- Paridad de docs mantenida durante el ciclo de planificacion e implementacion.
- CI local containerizado agregado para reducir drift entre host y GitHub Actions.
- Docs y ejemplos de trading ampliados con:
  - ejemplo de pipeline guiado por configuracion
  - ejemplo de politica temporal
- La integracion con ViperTrade se uso como prueba funcional de los slices de `0.8.1`.

### Snapshot de Validación del Workspace

- Estado del release: tag `v0.8.1` cortado, crates publicados y artefactos standalone liberados.
- Estado de validacion:
  - docs parity en verde
  - markdownlint en verde
  - CI en verde para los cambios de lenguaje y runtime mergeados
  - CI local de ViperTrade en verde contra la linea del release
  - runtime de ViperTrade alineado con la release oficial del CLI standalone `v0.8.1`

### Deuda Técnica

- El acceso tipado a configuracion esta resuelto de forma pragmatica mediante `input` estructurado, no por sintaxis dedicada.
- La politica temporal sigue siendo declarativa en la capa de policy; el estado del host sigue fuera del runtime del lenguaje.
- La ergonomia de politica reutilizable sigue dependiendo sobre todo de funciones normales y composicion explicita de records.

## 0.8.0-rc.5 (2026-03-07)

- Correcciones de compatibilidad del parser para adopción de pipelines de ViperTrade:
  - tolerar declaraciones `type` en nivel superior
  - tolerar declaraciones `extern fn ...;` en nivel superior
  - aceptar nombres de step sin comillas (`step(name)`) en pipelines
- Mejora de documentación de publicación de crates:
  - se añadió `README.md` en todos los crates publicables
  - se añadió `readme = "README.md"` en todos los manifests de crates

## 0.8.0 (2026-03-05)

- Tema del release: integración Python controlada y auditable para pipelines de producción.
- Principio guía: "Integrar sin perder gobernanza - cada llamada Python es rastreada, validada y auditable."

### Alcance Entregado

- Interoperabilidad Python (`tupa-pyffi`) para invocación segura de pasos `py:module.func`.
- Resiliencia de runtime con circuit breaker y soporte async/await.
- Flujo de backtesting con evaluación de PnL/riesgo y logging de auditoría estructurado.
- Mejoras de validación para shapes de tensores, atributos de pipeline y robustez de parser/typechecker.

### Ingeniería y CI Completados

- CI ahora exige convención de título de PR (`type(scope): subject`) y convención de mensajes de commit.
- Etiquetado automático de PR por tipo de cambio (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`, `breaking`).
- Release Drafter habilitado con categorización automática.
- Protección de rama en `main` reforzada:
  - checks requeridos (`pr-title-convention`, `commit-message-convention`, `lint`, `test`)
  - requisito estricto de rama actualizada
  - resolución de conversaciones requerida
  - revisión de CODEOWNERS y 1 aprobación requeridas
  - descarte de revisiones obsoletas habilitado
- CODEOWNERS agregado para archivos críticos de gobernanza y workflows.
- Gobernanza de backport implementada:
  - validación de etiquetas `backport-X.Y`
  - creación automática de issue de seguimiento para PRs mergeados con etiqueta de backport
- Operación de releases documentada en `release_guide.md` y `release_cut_checklist.md`.
- Validación local estandarizada con `scripts/ci-local.sh` (código + lint de docs/links).

### Snapshot de Validación del Workspace

- Chequeo local completo ejecutado en 2026-03-05: `./scripts/ci-local.sh`.
- Resultado: pass (`fmt`, `clippy`, `test`, `markdownlint`, `lychee`).
- Estado del working tree durante la validación: limpio en `main`.

### Deuda Técnica

- La validación de convención de commit aún depende del contexto de PR; los pushes directos a ramas protegidas deben permanecer bloqueados por política.
- Los quality gates de docs son sólidos en CI, y la paridad multilenguaje de estructura y versión más reciente ya está automatizada; la paridad semántica completa del contenido traducido sigue siendo manual.
- El workflow de backport crea issues de seguimiento, pero la automatización de cherry-pick de backport aún no está implementada.
- Los objetivos de rendimiento están documentados, pero no existe dashboard de tendencia en CI con histórico de latencia y throughput.

## 0.7.0 (2026-02-20)

- Release: motor híbrido con gobernanza nativa de pipelines
- CLI: `tupa run` con `--plan`, `--plan-only`, `--output`
- Runtime: reporte JSON con métricas y restricciones (pass/fail), hash de auditoría
- Determinismo: `@deterministic(seed=...)` parseado y seed propagada al PRNG
- Codegen: `ExecutionPlan` JSON con `steps`, `constraints`, `metrics`, `metric_plans`
- Validación: entrada JSON validada contra `TypeSchema` antes de ejecutar

### Añadido

- Backend híbrido:
  - ExecutionPlan JSON para pipelines
  - CLI `tupa codegen --format=llvm` emite `.ll` y `.plan.json`
  - Runtime de pipeline (`tupa-runtime`) y comando `tupa run`
- Validador de pipeline:
  - `@deterministic` rechaza `Random`/`Time` (E2005)
  - Restricciones con métricas indefinidas (E2006)
- Sin breaking changes

### Rendimiento

- Tiempo de compilación (ejemplo medio): objetivo < 200ms
- Estado: no benchmarkeado explícitamente en CI; seguido como objetivo de producto
- Cómo medir localmente:
  - Construye el CLI: `cargo build --quiet`
  - Comandos de benchmark (ejemplo):
    - `tupa codegen --format=llvm examples/pipeline/minimal.tp`
    - `tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json`
  - Opcional: usa `hyperfine` para benchmark:
    - `hyperfine --warmup 3 'tupa codegen --format=llvm examples/pipeline/minimal.tp' 'tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json'`
  - Condiciones: Linux, Rust stable (>=1.75), builds release cuando aplique
- Hardware y condiciones:
  - Linux x86_64, Rust stable, máquina local de dev, cold run
- Referencia de test (imprime timing):
  - `cargo test -p tupa-cli perf -- --nocapture`
  - Observado localmente: `codegen fraud_complete  1ms`, `run fraud_complete  3ms` (fuera de CI, ilustrativo)

## 0.6.0 (2026-02-13)

- Inferencia de constructor de enum con genéricos y restricciones Safe en variants.
- Los patrones de match ahora soportan destructuring de constructor con patrones de tupla.
- Uso de binding en guard de match validado en el typechecker.
- Diagnósticos de match no exhaustivo ahora apuntan a spans del scrutinee.
- Pruebas añadidas para restricciones de constructor de enum y destructuring/guards de match.
- Prototipo del motor de auditoría con hash determinístico para AST y entradas.
- Comando `tupa audit` en el CLI con salida JSON para hashes.
- El CLI de auditoría ahora usa SHA3-256 y flag `--input`.
- Soporte añadido a anotaciones `@safety` en el parsing.
- Ejemplo de auditoría `fraud_pipeline.tp` alineado con las restricciones Safe actuales.
- Warning `private_interfaces` del typechecker resuelto para `Ty::Enum`.

## 0.5.0 (2026-02-12)

- Finalización de restricciones del typechecker y correcciones de validación.
- Restricciones Safe<string, ...>: diagnósticos para !hate_speech y !misinformation.
- Mejoras de claridad en diagnósticos y pase de consistencia.
- Cobertura de pruebas ampliada con casos negativos.
- Ejemplos de misinformation y goldens añadidos para Safe<string, ...>.
- Docs actualizadas con ejemplos safe y referencias de diagnósticos.
- Docs alineadas con el posicionamiento del README y actualizaciones de la hoja de ruta.
- Docs incluyen un ejemplo borrador de orquestación de pipeline.
- Plan de release alineado con la hoja de ruta de gobernanza de pipelines.
- Diagnósticos de match ahora apuntan a spans de patrón inválido; cobertura de pruebas negativas añadida.
- Anotaciones Safe ahora validan restricciones base; ejemplos de parámetros/retorno inválidos añadidos.
- Casos negativos de lex/parse y salidas de error JSON añadidos a los goldens.
- El script de actualización de goldens ahora cubre todos los ejemplos negativos.

## 0.4.0 (2026-02-11)

- Mejoras de codegen de closures y correcciones de captura de entorno.
- Mejoras de restricciones del typechecker y mejor inferencia de lambdas.
- Actualizaciones del flujo del CLI para el pipeline typecheck/codegen.
- SPEC y errores comunes actualizados para el nuevo comportamiento.
- Limpieza de documentación: inglés canónico, índices consolidados y entrada PT-BR.

## 0.3.0 (2026-02-07)

- Soporte de closures con captura real de variables (estructuras de entorno, asignación en heap).
- Mejoras en inferencia de tipos para lambdas con parámetros Unknown.
- Soporte para compatibilidad de tipo Func con parámetros Unknown en llamadas de función.
- Mejoras de calidad de código: Clippy y rustfmt en CI, correcciones de warnings.
- Soporte básico para traits (parsing, typechecking, codegen).
- Soporte básico para enums (parsing, typechecking, codegen).
- Tests unitarios añadidos a codegen.
- Ejemplo de enum añadido a la documentación.
- Índice/SUMMARY centralizado y enlaces internos de docs.
- Sincronización de CHANGELOG, VERSIONING y RELEASE_GUIDE.
- Detección de captura de variables en lambdas (closures en desarrollo).
- Correcciones de TODOs residuales en codegen para mayor robustez.
- Implementación de inferencia de tipos para parámetros de lambda.
- Soporte básico de closures en codegen (aún sin captura de entorno).
- Correcciones de golden tests para casos de error (mensajes de cargo removidos).

## 0.2.0 (2026-02-06)

- Soporte de closures con captura real de variables (estructuras de entorno, asignación en heap).
- Mejoras en inferencia de tipos para lambdas con parámetros Unknown.
- Soporte para compatibilidad de tipo Func con parámetros Unknown en llamadas de función.
- Mejoras de calidad de código: Clippy y rustfmt en CI, correcciones de warnings.
- Soporte básico para traits (parsing, typechecking, codegen).
- Soporte básico para enums (parsing, typechecking, codegen).
- Tests unitarios añadidos a codegen.
- Ejemplo de enum añadido a la documentación.
- Índice/SUMMARY centralizado y enlaces internos de docs.
- Sincronización de CHANGELOG, VERSIONING y RELEASE_GUIDE.
- Detección de captura de variables en lambdas (closures en desarrollo).
- Correcciones de TODOs residuales en codegen para mayor robustez.
- Implementación de inferencia de tipos para parámetros de lambda.
- Soporte básico de closures en codegen (aún sin captura de entorno).
- Correcciones de golden tests para casos de error (mensajes de cargo removidos).

## 0.9.6 (2026-06-06)

- Tema de la versión: limpieza del legado `.tp` y bump de versión coordinado.

### Alcance Entregado

- **Eliminación del legado `.tp`**: se borraron todos los fuentes de ejemplo `.tp` (~100 archivos) y sus recursos de soporte (helpers FFI de Python, entradas JSON, scripts generadores) de `examples/`.
- **Limpieza de goldens**: se eliminaron las salidas golden obsoletas en `examples/expected/` generadas por el CLI `.tp` descontinuado; se conservó solo el golden de Rust-DSL (`expand_simple_pipeline.txt`).
- **Orden del repositorio**: se eliminaron artefactos legacy sueltos de la raíz (`update_golden.py`, `data.json`, `tx.json`, `my_test_plugin.rs`, `my_fixed_plugin.rs`, `integration_test.tupa`, `test_pipe.tupa`, `vipertrade_smoke.plan.json`, `test_find.md`).
- **Ejemplos reorganizados**: `examples/` ahora contiene solo material Rust-DSL; se actualizaron `examples/README.md` y `examples/migration/README.md`; se eliminaron los subdirectorios obsoletos `pipeline/`, `production/` y `playground/`.
- **Bump de versión**: todos los crates activos a 0.9.6 (sin cambios funcionales ni de API).
- **Docs de features reescritos a Rust DSL**: `features/trading_support.md` (EN/ES/PT-BR) ahora refleja los crates actuales con un ejemplo ejecutable `pipeline!` + `Executor` y marca explícitamente las funciones del runtime 0.8.2 eliminadas (backtest, circuit breaker, hot reload, registro de esquemas); `governance/audit_engine.md` (ES/PT-BR) reemplazado por una nota de descontinuación que apunta a las métricas por paso de `tupa-engine`.
- **READMEs de crates corregidos** para precisión en crates.io (desajustes de API en `tupa-core`, `tupa-pyffi`, `tupa-plugin`, `tupa-engine`; `tupa-lints` re-encuadrado como constantes string, no lints de rustc).
- **Experimental — constraints a nivel de tipo (PoC)**: enforcement real de `Safe<T, C>` vía `Constraint`/`ConstraintError`, markers integrados (`tupa_core::constraints::{NonNan, NonInf, Finite}`), `Safe::try_new`/`new_unchecked`, y una macro `safe!` que prueba `!nan`/`!inf` en expresiones `f64` constantes en tiempo de compilación (guard en runtime en otro caso). Superficie inestable — primer paso del roadmap spec→crates.

### Ingeniería y CI Completados

- Se corrigió el workflow `examples-golden.yml` para comparar goldens recién generados contra los versionados (antes comparaba el directorio consigo mismo, ocultando desvíos).
- `cargo fmt`, `cargo clippy --workspace --all-targets`, `cargo test --workspace` en verde.

### Snapshot de Validación (workspace)

- Build: `cargo build --workspace` ok.
- Tests: `cargo test --workspace` en verde (167 tests).
- Smoke: `scripts/vipertrade-smoke.sh` ok.
- Goldens: `scripts/update-goldens.sh` no produce diff contra `examples/expected/`.

### Deuda Técnica

- Varios docs instructivos todavía invocan el removido `tupa-cli` (`reference/codegen.md`, `guides/testing.md`, `guides/tutorials.md`, `guides/faq.md`, `governance/issues_guide.md`, `guides/examples_guide.md`); deben migrarse a flujos `cargo-tupa` / Rust-DSL en un seguimiento. Las referencias históricas en `ARCHITECTURE.md`, `PROPOSAL.md`, `roadmap.md`, archivos y changelogs previos son intencionales y se dejan como están.

## 0.9.5 (2026-05-16)

- Tema del release: cobertura de tests, operaciones Safe/Tensor, paths de cargo-tupa, y estabilidad de tupa-pyffi.

### Alcance Entregado

- **Completado de cobertura de tests** (TC-51, TC-54, TC-55, TC-56):
  - Corregido `tc41_step_panic_display` — eliminada aserción incorrecta `source()`
  - Corregido `tc46_step_timeout` / `tc52_from_env_timeout_caught_by_executor` — sleep de SlowP cambiado de 10ms a 200ms con `spawn_blocking`
  - Corregido `tc51_no_produces_for_single_step` — `SingleP::produces` retorna array vacío para step desconocido
- **Nuevos tests unitarios**: 32 tests en `tupa-core-macros/tests.rs` y 30 tests en `tupa-core/src/tests.rs` (TC-C54..TC-C81)
- **Tests de cancelación de Executor**: TC-55 y TC-56 para comportamiento de `Executor::cancel()`
- **Benchmarks criterion**: `engine_bench.rs` con benchmarks de secuencial, paralelo, DAG, constraint, metrics y executor_new
- **Operadores aritméticos Safe**: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `AddAssign`, `SubAssign`, `MulAssign`, `DivAssign` para `Safe<T,C>`
- **Métodos Tensor**: `new()`, `get()`, `into_inner()`, implementación `PartialEq`
- **Mejoras tupa-pyffi**: `call_with_multiple_args()` para llamadas multi-arg, `reset_python_bridge()` para reset de estado global, tipos extendidos (i32, u64, u32, f32, Vec<u8>, Vec<Value>)

### Ingeniería y CI Completados

- 162 tests pasando en el workspace
- `cargo fmt`, `cargo clippy`, `cargo test --workspace` todos verdes
- Bump de versión a 0.9.5 en todos los crates activos

## 0.9.0 (2026-05-11)

### Alcance Entregado

- **Nueva arquitectura crate-first**: `tupa-core` (macro pipeline! + tipos), `tupa-engine` (ejecutor paralelo), `tupa-plugin` (carga dinâmica), `cargo-tupa` (CLI)
- **Ejecución paralela**: Scheduler DAG basado en canales con detección de ciclos (`Executor::run_parallel`)
- **Sistema de constraints**: Verificación en compile-time + runtime con DSL `metric("name").op(valor)`
- **Plugin FFI**: ABI C para registro de step functions (`libloading` + `extern "C"`)
- **Herramientas de migración**: Ejemplos y guías para conversión `.tp` → Rust DSL
- **Paridad de documentación**: EN, ES, PT-BR con enlaces cruzados completos

### Ingeniería y CI Completados

- Workflows CI: lint (clippy, rustfmt), test (workspace), docs-lint (markdownlint, parity, lychee), smoke gate vipertrade
- Golden tests regenerados con `RUSTFLAGS="-Awarnings"` para suprimir warnings de deprecación
- Todos los enlaces relativos rotos corregidos (grammar.ebnf, type_semantics, PROPOSAL, TRANSITION, etc.)
- URLs externas actualizadas (rutas ViperTrade, GitHub Discussions → Issues)
- `tupa-cli` preservado para flujo `.tp` legacy; `cargo-tupa` para Rust DSL
- Bump de versiones: `tupa-core` 0.9.0, `tupa-core-macros` 0.9.0, `tupa-engine` 0.9.0, `tupa-plugin` 0.9.0, `cargo-tupa` 0.9.0, `tupa-template` 0.9.0

### Snapshot de Validación (workspace)

- **Estado del release**: Tag `v0.9.0` creada; crates publicados en crates.io (core, engine, plugin, cargo-tupa)
- **Estado de validación**:
  - docs parity: verde (todos los archivos requeridos presentes en EN/ES/PT-BR)
  - markdownlint: verde
  - link-check (lychee): 0 errores
  - CI: todos los jobs pasando (lint, test, vipertrade-smoke)
  - ViperTrade smoke gate valida `tupa-cli` check + codegen para `vipertrade_smoke.tp`
- **Crates publicados**: `tupa-core@0.9.0`, `tupa-engine@0.9.0`, `tupa-plugin@0.9.0`, `cargo-tupa@0.9.0`
- **Crates legacy mantenidos**: `tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-runtime`, `tupa-effects`, `tupa-audit`, `tupa-fmt`, `tupa-lint` en 0.8.x

### Deuda Técnica

- `tupa-conformance` no publicado (validador SPEC — artifact Phase 0, puede quedar como dev-dependency)
- `tupa-core-macros` sin CHANGELOG.md (debe agregarse)
- `crates/tupa-template` usa path dependencies en Cargo.toml template — necesita parche para proyectos generados
- PyFFI (`tupa-pyffi`) aún en 0.8.2 — migración a API 0.9.0 pendiente (Phase 3)
- LSP (`tupa-lsp`) no implementado (diferido; rust-analyzer cubre DSL)
- Suite de benchmarks (`criterion`) no creada (Phase 4)
- Algunos items públicos en `tupa-core`/`tupa-engine` carecen de docs `///` (necesita pass de文档 antes de 1.0)
