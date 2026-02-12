
# Changelog

## Purpose

Record relevant changes per version.

## 0.5.0 (Unreleased)

- Typechecker constraints completion and validation fixes.
- Safe<string, ...> constraints: !hate_speech and !misinformation diagnostics.
- Diagnostics clarity improvements and consistency pass.
- Expanded test coverage with negative cases.
- Added misinformation examples and goldens for Safe<string, ...>.
- Docs updated with safe examples and diagnostics references.
- Docs aligned with README positioning and roadmap updates.
- Docs include a draft pipeline orchestration example.
- Release plan aligned with pipeline governance roadmap.
- Match diagnostics now point to invalid pattern spans; added negative test coverage.
- Safe annotations now validate base constraints; added invalid param/return examples.
- Negative lex/parse cases and JSON error outputs added to goldens.
- Golden update script now covers all negative examples.

## 0.4.0 (2026-02-11)

- Closure codegen improvements and environment capture fixes.
- Typechecker constraint improvements and better lambda inference.
- CLI flow updates for the typecheck/codegen pipeline.
- SPEC and common errors refreshed for the new behavior.
- Documentation cleanup: canonical English, consolidated indices, and PT-BR entrypoint.

## 0.3.0 (2026-02-07)

- Closure support with real variable capture (environment structures, heap allocation).
- Improvements in type inference for lambdas with Unknown parameters.
- Support for Func type compatibility with Unknown parameters in function calls.
- Code quality improvements: Clippy and rustfmt in CI, warning fixes.
- Basic support for traits (parsing, typechecking, codegen).
- Basic support for enums (parsing, typechecking, codegen).
- Unit tests added to codegen.
- Enum example added to documentation.
- Centralized index/SUMMARY and internal doc links.
- Sync of CHANGELOG, VERSIONING, and RELEASE_GUIDE.
- Variable capture detection in lambdas (closures in development).
- Fixes for residual TODOs in codegen for better robustness.
- Implementation of type inference for lambda parameters.
- Basic closure support in codegen (without environment capture yet).
- Golden test fixes for error cases (removed cargo messages).

## 0.2.0 (2026-02-06)

- Closure support with real variable capture (environment structures, heap allocation).
- Improvements in type inference for lambdas with Unknown parameters.
- Support for Func type compatibility with Unknown parameters in function calls.
- Code quality improvements: Clippy and rustfmt in CI, warning fixes.
- Basic support for traits (parsing, typechecking, codegen).
- Basic support for enums (parsing, typechecking, codegen).
- Unit tests added to codegen.
- Enum example added to documentation.
- Centralized index/SUMMARY and internal doc links.
- Sync of CHANGELOG, VERSIONING, and RELEASE_GUIDE.
- Variable capture detection in lambdas (closures in development).
- Fixes for residual TODOs in codegen for better robustness.
- Implementation of type inference for lambda parameters.
- Basic closure support in codegen (without environment capture yet).
- Golden test fixes for error cases (removed cargo messages).

## 0.1.0

- Specification v0.1 published.
- Basic lexer, parser, typechecker, and CLI.
