# Plan Mínimo de Adopción Técnica

## Propósito

Definir un camino incremental para hacer la linguagem usable y confiable, sin comprometer fechas.

## Índice

- [Fase 0: Núcleo mínimo](#fase-0-núcleo-mínimo)
- [Fase 1: Toolchain básica](#fase-1-toolchain-básica)
- [Fase 2: Experiencia del desarrollador](#fase-2-experiencia-del-desarrollador)
- [Fase 3: Interoperabilidad](#fase-3-interoperabilidad)
- [Fase 4: Calidad y confianza](#fase-4-calidad-y-confianza)
- [Entregables mínimos](#entregables-mínimos)

## Fase 0: Núcleo mínimo

- Definir el subconjunto central (sintaxis y tipos básicos).
- Especificación formal mínima (EBNF + semántica de tipos).
- Suite de conformidad (parser + type checker).
- Salida de diagnósticos consumible por herramientas (JSON).

## Fase 1: Toolchain básica

- Formateador oficial.
- Linter con reglas mínimas.
- Language server (autocomplete, diagnósticos, go-to-definition).

## Fase 2: Experiencia del desarrollador

- Plantillas de proyecto (CLI y servicio).
- CLI estable con `build`, `run`, `fmt`, `check`.
- Mensajes de error didácticos y consistentes.

## Fase 3: Interoperabilidad

- FFI con C/Rust.
- ABI documentada.
- Bindings mínimos para bibliotecas esenciales.

## Fase 4: Calidad y confianza

- Benchmarks públicos y reproducibles.
- Pruebas de regresión de rendimiento.
- Política de versionado y compatibilidad.

## Entregables mínimos

- SPEC con EBNF y reglas de tipos.
- Pruebas automatizadas de parser/type checker.
- CLI funcional con ejemplos simples y `--format pretty|json`.
