# Error Messages Guide

## Purpose

This document standardizes the content and format of error messages.

## Standard

- Short, objective message.
- Include expected/found types when applicable.
- Show code (`E####`) when available.
- Point to the correct span (line/column).

## Examples

### Type mismatch

```text
error[E2001]: type mismatch: expected I64, got F64
  --> examples/types.tp:4:10
```

### Undefined variable

```text
error[E1002]: undefined variable 'x'
  --> examples/types.tp:2:1
```

### Unproven constraint

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> examples/invalid_safe_hate_speech.tp:2:38
```

### Unproven constraint (misinformation)

```text
error[E3002]: cannot prove constraint 'misinformation' at compile time
  --> examples/invalid_safe_misinformation.tp:2:41
```

### Unproven constraint with suggestion

```text
error[E3002]: constraint !misinformation not proven
  --> pipeline.tp:42:15
   |
42 | let summary = summarize(article)
   |               ^^^^^^^^^^^^^^^^^^^
   |
   = help: add safety proof: `@safety(score=0.98, dataset="factcheck-v3")`
   = note: required by return type `Safe<string, !misinformation>`
```

### Unproven constraint (misinformation, JSON)

```json
{
  "error": {
    "code": "E3002",
    "col": 41,
    "label": "examples/invalid_safe_misinformation.tp",
    "line": 2,
    "line_text": "\tlet x: Safe<string, !misinformation> = \"ok\";",
    "message": "cannot prove constraint 'misinformation' at compile time\nhelp: constraint must be provable at compile time; use a provable literal or pass a Safe value already proven",
    "span": {
      "end": 56,
      "start": 52
    }
  }
}
```

### Invalid constraint

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> examples/invalid_safe_hate_speech_base.tp:2:35
```

### Invalid constraint (misinformation)

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> examples/invalid_safe_misinformation_base.tp:2:38
```

## References

- [Diagnostics Glossary](diagnostics_glossary.md)
- [Diagnostics Checklist](diagnostics_checklist.md)
- [invalid_safe_misinformation.tp](../../examples/invalid_safe_misinformation.tp)
- [invalid_safe_misinformation_base.tp](../../examples/invalid_safe_misinformation_base.tp)
