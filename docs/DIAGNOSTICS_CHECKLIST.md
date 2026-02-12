# Diagnostics Checklist

## Purpose

Maintain a verifiable list of diagnostic requirements per compiler phase.

## Lexer

- [x] Reports error with absolute position (byte offset)
- [x] Converts offset to line/column (1-based)
- [x] Code excerpt with caret pointing to the token
- [x] Short, objective message

## Parser

- [x] Unexpected token error with valid span
- [x] EOF points to end of file
- [x] Shows expected token (when applicable)

## Typechecker

- [x] Errors include expected/found types
- [x] Messages for incorrect arity
- [x] Missing `return` in non-`unit` functions
- [x] Spans (line/column) when available
- [x] Diagnostics for anonymous functions (lambdas), function values, and print

## CLI

- [x] Standard format consistent with SPEC
- [x] Includes file/line/column
- [x] Supports clean output for pipes (no extra noise)

## Future

- [ ] Even more detailed error messages and automatic suggestions
