# Security Audit: Constraint Solver

## Audit Date: 2026-05

### Scope
This audit covers the `tupa-core-macros` procedural macro implementation and the constraint checking in `tupa-engine`.

## Components Audited

### 1. `pipeline!` Macro Expansion
- **File**: `crates/tupa-core-macros/src/lib.rs`
- **Mechanism**: Procedural macro parses DSL and generates `impl Pipeline` blocks
- **Surface Area**: Parse errors, attribute injection, constraint codegen

#### Findings
- ✅ No unsafe code in macro expansion
- ✅ Uses `syn` crate for AST parsing (well-maintained, no unsafe in safe config)
- ✅ Constraint expressions are Rust expressions executed at runtime
- ⚠️ Constraint failures return `ConstraintFailed` error without sanitization (information disclosure via error message)

#### Recommendations
1. Add `display` impl for `ConstraintFailure` that truncates expected/actual values

### 2. Executor Constraint Checking
- **File**: `crates/tupa-engine/src/lib.rs`
- **Mechanism**: `ParallelPipeline::check_constraints()` called after step execution
- **Surface Area**: JSON value comparison, error propagation

#### Findings
- ✅ Constraints run after step execution (no early abort based on partial data)
- ✅ All constraint checks use `serde_json::Value` for type-safe comparison
- ✅ No network I/O in constraint solver

#### Recommendations
1. Consider constant folding for compile-time constraint evaluation where possible

### 3. Type Safety
- **Issue**: `Safe<T, C>` type marker `C` is phantom - no runtime enforcement
- **Mitigation**: Documentation states marker types are compile-time only

## Security Model

| Layer | Protection |
|-------|------------|
| Parse | `syn` crate, no unsafe in safe mode |
| Expansion | Pure codegen, no runtime |
| Execution | Standard Rust safety |
| Constraints | JSON comparison, error isolation |

## Conclusion

The constraint solver is **low risk** for security vulnerabilities. Primary concerns:
1. Error message information disclosure (low severity)
2. Potential panic in constraint expressions (user code, not solver)

## Next Steps
- [ ] Add constraint value truncation in error messages
- [ ] Document constraint expression safety