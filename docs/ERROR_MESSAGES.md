# Error Messages Guide

## Purpose

Standardize the content and format of error messages.

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

# References

- [Diagnostics Glossary](DIAGNOSTICS_GLOSSARY.md)
- [Diagnostics Checklist](DIAGNOSTICS_CHECKLIST.md)
