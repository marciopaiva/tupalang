# Phase 2 & 0.9.4 Release — Current Sprint

**Status:** In Progress (Week 13–16, 2026-05-14 → 2026-05-29)  
**Milestone:** Tupã 0.9.4 release  
**Focus:** cargo-tupa CLI maturation, engine enhancements, migration guide, plugin tutorials

---

## 0.9.4 Feature Set

### cargo-tupa CLI Maturation

| Feature | Status | Location |
|---|---|---|
| `discover` subcommand (auto-detect binary target) | ✅ Complete | `crates/cargo-tupa/src/discover.rs` |
| `fmt` subcommand (pipeline! formatting) | ✅ Complete | `crates/cargo-tupa/src/fmt.rs` |
| `lint` subcommand (step name/reference checking) | ✅ Complete | `crates/cargo-tupa/src/lint.rs` |
| Unit tests (5 tests passing) | ✅ Complete | `crates/cargo-tupa/tests/` |
| Golden tests for fmt/lint output | 🚧 In Progress | `crates/cargo-tupa/tests/golden/` |

**Decision:** `run` builds via `cargo build --release --bin <name>` then executes binary. Avoids re-implementing Rust compiler.

**Critical:** `cargo-tupa` currently compiles with warnings — needs `cargo fix` cleanup before publish.

---

### Engine Enhancements

| Feature | Status | Location |
|---|---|---|
| `Executor::from_env()` + `ExecutorConfig::from_env()` | ✅ Complete | `crates/tupa-engine/src/lib.rs` |
| Per-step timeout via `TUPA_STEP_TIMEOUT` | ✅ Complete | `crates/tupa-engine/src/lib.rs` |
| Channel capacity via `TUPA_CHANNEL_CAPACITY` | ✅ Complete | `crates/tupa-engine/src/lib.rs` |
| `parse_duration` helper (ms, s, m) | ✅ Complete | `crates/tupa-engine/src/lib.rs` |
| StepMetrics collection (timestamps) | 🚧 In Progress | `crates/tupa-engine/src/metrics.rs` |
| Pipeline cancellation (Ctrl+C, `Executor::cancel()`) | 🚧 In Progress | `crates/tupa-engine/src/cancellation.rs` |
| `--metrics-output` JSON export | ⏳ Planned | `crates/cargo-tupa/src/run.rs` |

**Implementation details:**
- Timeout wraps each step future via `tokio::time::timeout`
- Metrics collected in manager task; records start/end `Instant` and state (Completed, Failed, Timeout)
- Cancellation via `Drop` on Executor + `tokio::signal::ctrl_c` handler; stores `AbortHandle`

---

### Documentation & Tutorials

| Doc | Status | Location |
|---|---|---|
| Migration guide (.tp → Rust-DSL) | 🚧 In Progress | `docs/guides/migration_guide.md` |
| Plugin tutorial (Rust) | 🏗️ Scaffolded | `docs/tutorials/plugin-rust.md` |
| Plugin tutorial (Python) | 🏗️ Scaffolded | `docs/tutorials/plugin-python.md` |
| Working plugin examples | ⏳ Planned | `examples/plugins/` |

---

## Completed Pre-Release Tasks

- ✅ Released 0.9.2 and 0.9.3 to crates.io (publish workflow fixes)
- ✅ Removed all legacy .tp compiler/tooling crates from workspace
  - Removed: `tupa-lexer`, `tupa-parser`, `tupa-typecheck`, `tupa-codegen`, `tupa-cli`, `tupa-runtime` (old), `tupa-effects`, `tupa-audit` (legacy), `tupa-conformance` (legacy), `tupa-fmt` (legacy), `tupa-lint` (legacy)
  - Active crates: `tupa-core-macros`, `tupa-core`, `tupa-engine`, `tupa-plugin`, `tupa-pyffi`, `cargo-tupa`
- ✅ Bumped all active crates to 0.9.4, regenerated Cargo.lock
- ✅ Unit tests for cargo-tupa (discover, fmt, lint) — 5 tests passing

---

## ⚠️ Release Process Rule

**MANDATORY:** Never create/push a git tag without running `ci-local` first.

```bash
# From tupalang/ repository root
./scripts/ci-local.sh

# Or inside Docker container (matches GitHub Actions environment)
./scripts/ci-local-container.sh
```

**ci-local validates:**

- `cargo fmt --check` — code formatting
- `cargo clippy -D warnings` — zero warnings
- `cargo test --workspace --locked` — all tests
- `markdownlint` — all .md files
- `scripts/docs-parity-check.sh` — docs parity EN/ES/PT-BR
- Golden output comparison (`scripts/update-goldens.sh`)
- Link checking via `lychee`
- Optional: commit message convention (`CI_LOCAL_CHECK_COMMIT=1`)
- Optional: publish dry-run (`CI_LOCAL_CHECK_PUBLISH=1`)

**If any check fails:** Fix, re-run ci-local, only then tag.

---

## Next Steps (Immediate)

1. **Engine metrics**
   - [ ] Add `StepMetrics` struct with `start: Instant`, `end: Instant`, `state: StepState`
   - [ ] Instrument `Executor::run_parallel` to record per-step timings
   - [ ] Add `ExecutionResult::metrics` field (Vec\<StepMetrics\>)

2. **Cancellation**
   - [ ] Store `AbortHandle` in `Executor` struct
   - [ ] Implement `Drop` for `Executor` to signal cancellation
   - [ ] Check cancellation flag during step execution loop
   - [ ] Handle Ctrl+C gracefully (tokio::signal)

3. **cargo-tupa run integration test**
   - [ ] Create sample pipeline package in `crates/cargo-tupa/tests/fixtures/`
   - [ ] Test `cargo tupa run` builds and executes correctly
   - [ ] Test binary auto-discovery works

4. **Documentation**
   - [ ] Write `docs/guides/migration_guide.md` with .tp → Rust-DSL mapping table
   - [ ] Complete `docs/tutorials/plugin-rust.md` (step-by-step)
   - [ ] Complete `docs/tutorials/plugin-python.md`
   - [ ] Add plugin example to `examples/` (both Rust and Python)

5. **Quality gates**
   - [ ] Run `cargo fix --allow-dirty` to clean warnings in `cargo-tupa`
   - [ ] Ensure `cargo clippy -D warnings` passes on all active crates
   - [ ] Ensure `cargo fmt --check` passes
   - [ ] Ensure `cargo test --workspace` passes
   - [ ] Update CHANGELOGs for 0.9.4 in each active crate

6. **Publish**
   - [ ] Tag `v0.9.4` (GitHub Actions publish workflow triggers)
   - [ ] Publish order: tupa-core-macros → tupa-core → tupa-engine → tupa-plugin → tupa-pyffi → cargo-tupa
   - [ ] Verify crates.io versions updated
   - [ ] Update workspace Cargo.lock

---

## Blockers / Dependencies

- **None** — all tasks independent.

---

## Key Decisions Recap

- **Backward compatibility:** All new features opt-in (env vars, CLI flags).
- **Legacy removal:** All .tp toolchain permanently removed; clean workspace.
- **Engine design:** Timeout per step; cancellation cooperative; metrics aggregated centrally.
- **CLI philosophy:** `cargo-tupa` delegates building to `cargo` itself; focuses on DSL validation and execution.

---

## Definition of Done (0.9.4 Release)

- [ ] `cargo tupa` subcommands (discover, fmt, lint, run) fully functional
- [ ] Engine per-step timeout working (TUPA_STEP_TIMEOUT)
- [ ] Engine cancellation working (Ctrl+C, `Executor::cancel()`)
- [ ] StepMetrics collected and exportable via `--metrics-output`
- [ ] Migration guide published
- [ ] Plugin tutorials (Rust + Python) complete with working examples
- [ ] Golden tests for fmt/lint output passing
- [ ] **`./scripts/ci-local.sh` passes all checks** (fmt, clippy -D, test, markdownlint, docs parity, goldens, lychee)
- [ ] All active crates at version 0.9.4 on crates.io (publish workflow successful for tag `v0.9.4`)

---

## Team Assignment

| Area | Owner | Tasks |
|---|---|---|
| cargo-tupa CLI | @cli-team | discover, fmt, lint, run, integration tests, golden tests |
| Engine metrics/cancellation | @runtime-team | StepMetrics, AbortHandle, signal handling |
| Documentation | @docs-team | Migration guide, plugin tutorials (Rust/Python) |
| Release engineering | @infra | Version bump, CHANGELOGs, publish workflow, CI validation |

---

*Last updated: 2026-05-14 — Sprint active, targeting 0.9.4 release*
