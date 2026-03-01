
# Changelog

## Propósito

Registrar cambios relevantes por versión.

## 0.8.0 (No lanzado)

- Tema del release: integración Python controlada y auditable para pipelines de producción.
- Principio guía: "Integrar sin perder gobernanza — cada llamada Python es rastreada, validada y auditable".
- Alcance: orquestación de PyTorch/TensorFlow vía adaptadores auditados.
- Enfoque: rastreo de ejecución, hooks de validación y esquema de log de auditoría para llamadas Python.

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
  - Observado localmente: `codegen fraud_complete ≈ 1ms`, `run fraud_complete ≈ 3ms` (fuera de CI, ilustrativo)

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

## 0.1.0

- Specification v0.1 publicada.
- Lexer, parser, typechecker y CLI básicos.
