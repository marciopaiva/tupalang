
# Arquitectura

## Propósito

Explicar la organización del repositorio y el flujo principal del compilador.

## Visión general

El proyecto es un workspace Rust con múltiples crates implementando etapas del compilador.

## Estructura de carpetas

- `crates/tupa-lexer`: tokenización del código fuente.
- `crates/tupa-parser`: construcción de AST.
- `crates/tupa-typecheck`: verificación de tipos y restricciones, incluyendo funciones anónimas (lambdas) y valores de función.
- `crates/tupa-codegen`: generación de IR funcional (LLVM-like), con soporte para funciones, lambdas, print, concatenación de strings, arrays, flujo de control y más.
- `crates/tupa-audit`: hash de auditoría determinístico para AST + entradas.
- `crates/tupa-cli`: interfaz de línea de comandos, integración de todas las etapas y ejecución de pruebas goldens.
- `docs/`: documentación del producto y especificación.
- `examples/`: ejemplos ejecutables y pruebas goldens.

## Flujo principal

1) **Lexer**: convierte texto en tokens.
2) **Parser**: convierte tokens en AST.
3) **Verificador de tipos**: valida tipos, restricciones, funciones/lambdas y valores de función.
4) **Codegen**: genera IR funcional (LLVM-like) que cubre features del MVP.
5) **Audit**: genera hashes determinísticos a partir de AST y entradas.
6) **CLI**: integra etapas, expone comandos (`lex`, `parse`, `check`, `codegen`, `audit`) y ejecuta pruebas goldens automatizadas.

## Dependencias entre crates

- `tupa-parser` depende de `tupa-lexer`.
- `tupa-typecheck` depende de `tupa-parser`.
- `tupa-codegen` depende de `tupa-parser` y `tupa-typecheck`.
- `tupa-audit` depende de `tupa-parser`.
- `tupa-cli` depende de todas.

## Notas

- Los diagnósticos siguen spans y errores normalizados por etapa.
- La salida JSON del CLI permite integración de herramientas.
- Las pruebas goldens aseguran la estabilidad del pipeline.
- Ver detalles del verificador de tipos en [typechecker_details.md](../reference/typechecker_details.md).
