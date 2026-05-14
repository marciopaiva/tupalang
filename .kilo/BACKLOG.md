# Phase 2 Backlog — Developer Experience (Weeks 9–20)

**Status:** Sprint 5 (0.9.4) Active (Weeks 13–16)  
**Active sprint:** See [SPRINT_0.9.4.md](./SPRINT_0.9.4.md) for current work

---

## Sprint 3: Didactic Error Messages (Weeks 13–16) 🚧 DEFERRED

**Owner:** Compiler team  
**Goal:** All `pipeline!` macro errors emit clear, actionable diagnostics with `E####` codes.

*Note: Advanced to the extent needed for 0.9.4; full implementation continues after release.*

**Owner:** Compiler team  
**Goal:** All `pipeline!` macro errors emit clear, actionable diagnostics with `E####` codes.

### Tasks

- [ ] **E3001:** Step function not found — suggest import or correct name
- [ ] **E3002:** Constraint not provable at compile time — explain why, suggest `const fn`
- [ ] **E3003:** Type mismatch in step body — show expected vs actual
- [ ] **E3004:** Duplicate step name — highlight conflicting definitions
- [ ] **E3005:** Missing constraint annotation — suggest adding `metric("...")`
- [ ] **E3006:** Invalid metric reference — step doesn't produce that metric
- [ ] **E3007:** Async step without `@async` annotation (or vice versa)
- [ ] **E3008:** Impure function in purity-required context
- [ ] **E3009:** Circular dependency detected (`produces`/`requires` cycle)
- [ ] **E3010:** Constraint operator misuse (e.g., `metric("x").ge("not_a_number")`)

### Non-Functional Requirements

- [ ] Implement `tupa-expand --pretty` to show macro expansion for debugging
- [ ] All spans point to original DSL location (not generated code)
- [ ] Hints include `fix` suggestions where applicable (e.g., `did you mean: ...?`)
- [ ] `cargo tupa explain E3002` prints detailed explanation
- [ ] JSON error format stable: `{ code, message, spans[], hints[] }`

### Acceptance

```bash
# All errors have codes
cargo tupa check examples/bad_pipeline.rs 2>&1 | grep -E 'error\[E[0-9]+\]'

# Explain works
cargo tupa explain E3002

# JSON output
cargo tupa check --format json > errors.json
jq '.[] | .code' errors.json | sort -u
```

**Target completion:** Week 16 (2026-05-29)

---

## Sprint 4: Linter Rules Expansion (Weeks 17–20)

**Owner:** Lint team  
**Goal:** ≥10 lint rules, all documented, integrated into `cargo tupa lint`.

### Existing Rules (Legacy, need porting)

- [ ] Unused variable detection
- [ ] Shadowed variable warning
- [ ] Missing constraint annotation
- [ ] Impure in sensitive context
- [ ] Snake/PascalCase naming conventions

### New Rules for DSL

- [ ] **Step purity verification** — pure functions only (no side effects)
- [ ] **Constraint complexity** — cyclomatic complexity limit per constraint expr
- [ ] **Metric naming conventions** — snake_case, no reserved prefixes
- [ ] **Unreachable steps detection** — steps that never execute (cfg gating)
- [ ] **Explicit dependencies required** — `produces`/`requires` must be declared for all metrics
- [ ] **No mutable statics** — disallow `static mut` in step functions
- [ ] **No `unsafe` in steps** — safety-critical code should be audited separately
- [ ] **Prefer `tupa::` built-ins** — enforce use of `tupa::warn()`, `tupa::pass()`, etc.
- [ ] **Avoid `unwrap()`** — suggest `?` or explicit error handling
- [ ] **Max pipeline length** — warn if >50 steps (maintainability)

### Tasks

- [ ] Port legacy linter rules from `tupa-lint` to new AST
- [ ] Implement rule registry + configuration (`tupa.toml`)
- [ ] Add `cargo tupa lint` command (delegates to `tupa-lint` crate)
- [ ] Document each rule in `docs/en/guides/lint_rules.md`
- [ ] Add `--deny`/`--warn`/`--allow` per-rule configuration
- [ ] CI integration example (`.github/workflows/lint.yml`)

### Acceptance

```bash
# Lint passes on clean examples
cargo tupa lint examples/viper_smart_copy.rs

# Lint fails on bad examples with correct codes
cargo tupa lint examples/bad_practice.rs 2>&1 | grep 'warning\[L[0-9]+\]'

# JSON output
cargo tupa lint --format json > warnings.json
jq '.[] | .code' warnings.json | sort -u | wc -l  # >= 10
```

**Target completion:** Week 20 (2026-06-06)

---

## Blockers / Dependencies

- [x] Macro expansion stable (Sprint 1) → ✅ Done
- [x] Engine executor stable (Sprint 3) → ✅ Done
- [ ] `tupa-lint` crate updated for new AST — **IN PROGRESS**
- [ ] Diagnostic infrastructure in `tupa-core` — **IN PROGRESS**

---

## Definition of Done (DoD)

For each lint rule:
- [ ] Code implemented in `crates/tupa-lint/src/`
- [ ] Unit test with positive and negative cases
- [ ] Documentation page with examples
- [ ] CI workflow runs `cargo tupa lint` on PRs
- [ ] Rule enabled by default at `warn` level

For error messages:
- [ ] Every error has a unique code (E3001–E3020)
- [ ] `cargo tupa explain <code>` works
- [ ] Spans point to user code (not expanded code)
- [ ] At least one hint provided where applicable
- [ ] Internationalization placeholder (future i18n)

---

## Team Assignment

| Task | Team | Contact |
|---|---|---|
| Diagnostics (E3001–E3010) | Compiler | @compiler-team |
| Lint rules port + new | Lint | @lint-team |
| `tupa-expand` tool | Tooling | @dx-team |
| Documentation update | Docs | @docs-team |

---

*Last updated: 2026-05-14 — Sprint 3 active*
