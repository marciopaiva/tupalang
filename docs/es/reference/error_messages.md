# Guía de Mensajes de Error

## Propósito

Estandarizar el contenido y el formato de los mensajes de error.

## Estándar

- Mensaje corto y objetivo.
- Incluye tipos esperado/encontrado cuando aplique.
- Muestra código (`E####`) cuando esté disponible.
- Apunta al span correcto (línea/columna).

## Ejemplos

### Tipo incompatible

```text
error[E2001]: type mismatch: expected I64, got F64
  --> examples/types.rs:4:10
```text

### Variable indefinida

```text
error[E1002]: undefined variable 'x'
  --> examples/types.rs:2:1
```text

### Restricción no comprobada

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> examples/invalid_safe_hate_speech.rs:2:38
```text

### Restricción no comprobada (misinformation)

```text
error[E3002]: cannot prove constraint 'misinformation' at compile time
  --> examples/invalid_safe_misinformation.rs:2:41
```text

### Restricción no comprobada con sugerencia

```text
error[E3002]: constraint !misinformation not proven
  --> pipeline.rs:42:15
   |
42 | let summary = summarize(article)
   |               ^^^^^^^^^^^^^^^^^^^
   |
   = help: add safety proof: `@safety(score=0.98, dataset="factcheck-v3")`
   = note: required by return type `Safe<string, !misinformation>`
```text

### Restricción no comprobada (misinformation, JSON)

```json
{
  "error": {
    "code": "E3002",
    "col": 41,
    "label": "examples/invalid_safe_misinformation.rs",
    "line": 2,
    "line_text": "\tlet x: Safe<string, !misinformation> = \"ok\";",
    "message": "cannot prove constraint 'misinformation' at compile time\nhelp: constraint must be provable at compile time; use a provable literal or pass a Safe value already proven",
    "span": {
      "end": 56,
      "start": 52
    }
  }
}
```text

### Restricción inválida

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> examples/invalid_safe_hate_speech_base.rs:2:35
```text

### Restricción inválida (misinformation)

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> examples/invalid_safe_misinformation_base.rs:2:38
```text

## Referencias

- [Glosario de Diagnósticos](diagnostics_glossary.md)
- [Checklist de Diagnósticos](diagnostics_checklist.md)
