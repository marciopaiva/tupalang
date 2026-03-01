# Glosario de Diagnósticos

## Propósito

Listar códigos de error y advertencia emitidos por el compilador.

## Errores

### E1001 — Tipo desconocido

Emitido cuando un tipo no existe en el lenguaje.

### E1002 — Variable indefinida

Emitido cuando una variable se usa sin declaración previa.

### E1003 — Función indefinida

Emitido cuando una función se llama sin una definición visible.

### E2001 — Tipo incompatible

Emitido cuando el tipo encontrado no coincide con el tipo esperado.

### E2002 — Aridad incorrecta

Emitido cuando el número de argumentos en una llamada no coincide con la firma.

### E2003 — Operación binaria inválida

Emitido cuando un operador binario recibe tipos incompatibles.

### E2004 — Operación unaria inválida

Emitido cuando un operador unario recibe un tipo incompatible.

### E2005 — Objetivo de llamada inválido

Emitido cuando se llama algo que no es una función.

### E2006 — Retorno incompatible

Emitido cuando el tipo devuelto no coincide con el tipo esperado.

### E2007 — Retorno ausente

Emitido cuando una función debería retornar un valor pero no lo hace.

### E3001 — Restricción inválida

Emitido cuando una restricción no es compatible con el tipo base de `Safe<T, ...>`.
Ejemplos: `Safe<f64, !misinformation>`, `Safe<string, !nan>`.

### E3002 — Restricción no comprobada

Emitido cuando una restricción no puede comprobarse en tiempo de compilación.
Ejemplos: `Safe<string, !misinformation>` sin una fuente comprobada.

### E5001 — Match no exhaustivo

Emitido cuando una expresión `match` no cubre todos los patrones posibles.

## Advertencias

### W0001 — Variable no utilizada

Emitido cuando una variable se declara y no se usa.

## Referencias

- [Checklist de Diagnósticos](diagnostics_checklist.md)
