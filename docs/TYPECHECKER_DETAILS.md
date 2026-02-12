# Typechecker Details

## Purpose

Explain internal operation, design decisions, and algorithms of the Tupã typechecker.

## Overview

The typechecker walks the AST validating types, arity, constraints, and inferring types when possible. It supports anonymous functions (lambdas), function values, print as a built-in, strings, arrays, and composite types.

## Main algorithm

1. Traverse the AST in post-order.
2. For each node:
   - Check expected vs found type.
   - Check arity of functions and lambdas.
   - Propagate constraints (for example, Safe<T, !nan>).
   - Type inference for `let` without annotation.
   - Detailed diagnostics with spans.
3. Errors are accumulated and reported at the end.

## Flow example

```tupa
let f: fn(int) -> int = |x| x + 1
let y = f(10) // y: int
print("Result: " + y)
```

- The typechecker validates the type of `f`, infers the type of `y`, and ensures `print` receives a string.

## Design decisions

- **Local inference**: types are inferred only where there is no ambiguity.
- **Print as a built-in**: simplifies diagnostics and CLI integration.
- **Detailed spans**: all errors include precise location.
- **Extensible**: easy to add new types and constraints.

## Flow diagram

```mermaid
graph TD;
    A[AST] --> B[Typechecker]
    B --> C{Valid type?}
    C -- Yes --> D[Next node]
    C -- No --> E[Error with span]
    D --> F[Codegen]
    E --> F
```

## Useful links

- [Architecture](ARCHITECTURE.md)
- [Diagnostics](DIAGNOSTICS_CHECKLIST.md)
- [SPEC: Types](SPEC.md#type-system)
