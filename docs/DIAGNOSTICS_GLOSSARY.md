# Diagnostics Glossary

## Purpose

List error and warning codes emitted by the compiler.

## Errors

### E1001 — Unknown type

Emitted when a type does not exist in the language.

### E1002 — Undefined variable

Emitted when a variable is used without a prior declaration.

### E1003 — Undefined function

Emitted when a function is called without a visible definition.

### E2001 — Type mismatch

Emitted when the found type does not match the expected type.

### E2002 — Incorrect arity

Emitted when the number of arguments in a call does not match the signature.

### E2003 — Invalid binary operation

Emitted when a binary operator receives incompatible types.

### E2004 — Invalid unary operation

Emitted when a unary operator receives an incompatible type.

### E2005 — Invalid call target

Emitted when something that is not a function is called.

### E2006 — Incompatible return

Emitted when the returned type does not match the expected type.

### E2007 — Missing return

Emitted when a function should return a value but does not.

### E3001 — Invalid constraint

Emitted when a constraint is not compatible with the base type of `Safe<T, ...>`.
Examples: `Safe<f64, !misinformation>`, `Safe<string, !nan>`.

### E3002 — Unproven constraint

Emitted when a constraint cannot be proven at compile time.
Examples: `Safe<string, !misinformation>` without a proven source.

### E5001 — Non-exhaustive match

Emitted when a `match` expression does not cover all possible patterns.

## Warnings

### W0001 — Unused variable

Emitted when a variable is declared and not used.

## References

- [Diagnostics Checklist](DIAGNOSTICS_CHECKLIST.md)
