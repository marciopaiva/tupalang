# Plan MVP

## Propósito

Entregar un compilador mínimo que parsea, chequea tipos simples y genera un binario nativo para `hello.tp`.

## Índice

- [Alcance del MVP](#alcance-del-mvp)
- [Completado](#completado)
- [Próximos hitos](#próximos-hitos)
- [Criterios de aceptación](#criterios-de-aceptación-cuando-se-implemente)

## Alcance del MVP

### Completado

1. Lexer
   - Tokens básicos, comentarios `//`/`/* */`, strings y números.
2. Parser
   - AST para funciones, let, if, match, loops, arrays, llamadas, postfix y funciones anónimas (lambdas).
3. Verificador de tipos
   - Tipos primitivos (`i64`, `f64`, `bool`, `string`) e inferencia básica.
   - Tipos de función, chequeo de llamadas, valores de función, lambdas y retorno en todos los caminos.
4. Generación de código
   - Generación de IR funcional (LLVM-like) para funciones, lambdas, print, concatenación de strings, arrays, flujo de control y más.
5. CLI
   - `tupa-cli` con `lex`, `parse`, `check`, `codegen`, stdin y pruebas goldens.
6. Diagnósticos
   - Span/línea/columna en errores de lexer/parser/typechecker.
   - Mensajes para aridad, tipos, print, lambdas y más.
7. Closures
   - Soporte de closures con captura real de variables (estructuras de entorno, asignación en heap).

## MVP

### Próximos hitos

1. Optimizaciones de generación de código (eliminación de código muerto, mejor uso de registros)
2. Arrays/slices genéricos y más tipos
3. Cobertura de pruebas y benchmarks

## Criterios de aceptación (cuando se implemente)

## Roadmap

- Errores de tipo claros y localizados.
- Sin dependencias externas en tiempo de ejecución para binarios generados.
