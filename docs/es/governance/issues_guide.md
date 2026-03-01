# Guía de Issues

## Propósito

Estandarizar la creación de issues con información útil de triage.

## Cuándo abrir una issue

- Bugs y errores inesperados.
- Propuestas de mejora (usa `[RFC]`).
- Preguntas sobre spec o comportamiento.

## Checklist

- [ ] Título claro y específico.
- [ ] Pasos para reproducir (si es bug).
- [ ] Resultado esperado vs actual.
- [ ] Logs/prints relevantes.
- [ ] Versión de Rust y del proyecto.

## Ejemplo (bug)

**Título**: `Parser falla con match anidado`

**Description**:

- Pasos: `tupa-cli -- parse examples/match.tp`
- Esperado: AST válido
- Actual: error `unexpected token`

## Ejemplo (RFC)

**Título**: `[RFC] Tipos opcionales`

**Description**:

- Motivación
- Alternativas
- Impacto en la SPEC
