# MVP Plan

## Purpose

This document delivers a minimal compiler that parses, checks simple types, and generates a native binary for `hello.tp`.

## Index

- [MVP scope](#mvp-scope)
- [Completed](#completed)
- [Next milestones](#next-milestones)
- [Acceptance Criteria](#acceptance-criteria-when-implemented)

## MVP Scope

### Completed

1. Lexer
   - Basic tokens, `//`/`/* */` comments, strings, and numbers.
2. Parser
   - AST for functions, let, if, match, loops, arrays, calls, postfix, and anonymous functions (lambdas).
3. Typechecker
   - Primitive types (`i64`, `f64`, `bool`, `string`) and basic inference.
   - Function types, call checking, function values, lambdas, and return on all paths.
4. Codegen
   - Functional IR generation (LLVM-like) for functions, lambdas, print, string concatenation, arrays, control flow, and more.
5. CLI
   - `tupa-cli` with `lex`, `parse`, `check`, `codegen`, stdin, and golden tests.
6. Diagnostics
   - Span/line/column in lexer/parser/typechecker errors.
   - Messages for arity, types, print, lambdas, and more.
7. Closures
   - Closure support with real variable capture (environment structures, heap allocation).

## Next Milestones

1. Codegen optimizations (dead code elimination, better register use)
2. Generic arrays/slices and more types
3. Test coverage and benchmarks

## Acceptance Criteria (When Implemented)

## Roadmap

- Clear, localized type errors.
- No external runtime dependencies for generated binaries.
