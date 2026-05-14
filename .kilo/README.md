# Tupã Development Planning — Index

## ⚠️ Mandatory Release Rule

**NEVER publish a git tag without running `ci-local` first.**

```bash
./scripts/ci-local.sh          # local execution
./scripts/ci-local-container.sh # inside Docker (matches GH Actions)
```

All checks must pass before creating `v*` tags.

---

## 📋 Project Overview

- **[OVERVIEW.md](./OVERVIEW.md)** — What Tupã is, core value proposition, architecture summary, quick start
  - Read this first to understand the project in 5 minutes

## 🗺️ Roadmap

- **[ROADMAP.md](./ROADMAP.md)** — Full development roadmap to v1.0.0
  - Phases 0–4, timelines, decision gates, success metrics
  - Sprint breakdown and implementation status

## 📝 Current Work

- **[SPRINT_0.9.4.md](./SPRINT_0.9.4.md)** — Sprint 0.9.4 Release (Weeks 13–16) — 🚧 ACTIVE
- **[STATE_ANALYSIS.md](./STATE_ANALYSIS.md)** — Análise detalhada do estado atual, pendências, próximos passos

## 🔗 External Documentation

The `.kilo` directory inside `tupalang/` contains **planning documents** for development. Full project documentation is in:

- `tupalang/docs/en/` — Comprehensive user and developer guides
  - `PROPOSAL.md` — Strategic rationale for crate-first architecture
  - `ARCHITECTURE.md` — System design and crate map
  - `IMPLEMENTATION_PLAN.md` — Detailed week-by-week execution plan
  - `reference/spec.md` — Normative language specification

## 📊 Status Dashboard

| Phase | Status | Completion |
|---|---|---|
| Phase 0 — Baseline | ✅ Complete | 100% |
| Phase 1 — Basic Toolchain | ✅ Complete | 100% |
| Phase 2 — Developer Experience | 🚧 In Progress | ~40% |
| Phase 3 — Interoperability | ⏳ Planned | 0% |
| Phase 4 — Quality & Trust | ⏳ Planned | 0% |

**Next milestone:** End of Sprint 5 (0.9.4 release) — CLI maturation & engine enhancements complete (2026-05-29)

---

## ⚠️ Release Process Rule

**CRITICAL:** Never create/push a git tag without running `ci-local` first.

```bash
# From repository root
./scripts/ci-local.sh

# Or inside Docker container (matches GitHub Actions environment)
./scripts/ci-local-container.sh
```

The local CI script runs:
- `cargo fmt --check`
- `cargo clippy -D warnings`
- `cargo test --workspace`
- markdownlint
- docs parity check
- golden output verification
- link checking (lychee)
- optional: publish dry-run

If any check fails, **DO NOT TAG**. Fix issues first, re-run ci-local, then tag.

**Publish workflow:** GitHub Actions `publish-crates.yml` triggers on tag `v*` and publishes crates in dependency order. Ensure all checks pass locally before pushing tags.

---

*Planning documents maintained by the Tupã core team.*
