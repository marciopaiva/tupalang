# Checklist de Diagnósticos

## Propósito

Mantener una lista verificable de requisitos de diagnósticos por fase del compilador.

## Lexer

- [x] Reporta error con posición absoluta (offset en bytes)
- [x] Convierte offset a línea/columna (base 1)
- [x] Fragmento de código con caret apuntando al token
- [x] Mensaje corto y objetivo

## Parser

- [x] Error de token inesperado con span válido
- [x] EOF apunta al final del archivo
- [x] Muestra el token esperado (cuando aplica)

## Verificador de tipos

- [x] Errores incluyen tipos esperado/encontrado
- [x] Mensajes para aridad incorrecta
- [x] `return` ausente en funciones no-`unit`
- [x] Spans (línea/columna) cuando están disponibles
- [x] Diagnósticos para funciones anónimas (lambdas), valores de función y print

## CLI

- [x] Formato estándar consistente con la SPEC
- [x] Incluye archivo/línea/columna
- [x] Soporta salida limpia para pipes (sin ruido extra)

## Futuro

- [ ] Mensajes de error aún más detallados y sugerencias automáticas
