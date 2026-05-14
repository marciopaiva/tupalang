# Tupã Language Project Overview

**What is Tupã?**

Tupã is a **typed policy and strategy DSL** implemented as a set of Rust crates. It enables writing deterministic, auditable decision pipelines with compile-time guarantees for trading, risk, and AI inference orchestration.

## Core Value Proposition

- **Static validation** — constraints proven at compile time (no runtime surprises)
- **Type-safe dataflow** — Rust's borrow checker ensures no data races
- **Explainable decisions** — every step is auditable and traceable
- **Embedded, not external** — no separate toolchain; `cargo add tupa-core` is enough

## Target Domains

- Trading & risk management (position limits, drawdown caps, regulatory checks)
- AI inference orchestration (model selection, safety guards, gradient tracking)
- Critical decision services (fraud detection, compliance, approval workflows)

## Architecture: Crate-First

Tupã is **not** a standalone language with its own compiler. It is a **set of Rust crates**:

| Crate | Purpose | Status |
|---|---|---|
| `tupa-core` | DSL macros + policy types (`Safe`, `Tensor`) | 🚀 Alpha |
| `tupa-engine` | Pipeline executor (channels, scheduling) | 🚀 Alpha |
| `tupa-fmt` | Standalone formatter for legacy `.tp` files | ✅ Stable |
| `tupa-lint` | Linter for policy code quality | ✅ Stable |
| `tupa-audit` | Execution hashing for reproducibility | ✅ Stable |
| `tupa-plugin` | Dynamic step function loading | ✅ Stable |
| `tupa-conformance` | SPEC validator (CI tool) | ✅ Stable |

## Quick Example

```rust
use tupa_core::{pipeline, step, constraint, metric};

pipeline! {
    name: PreTradeCheck,
    input: Trade,
    steps: [
        step("risk") { risk_score(input) },
        step("limit") { input.size <= 1_000_000.0 }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0),
        metric("max_leverage").le(2.0)
    ]
}
```

## Repository Structure

```
tupalang/
├── crates/               # Rust crates (core, engine, runtime, etc.)
│   ├── tupa-core/
│   ├── tupa-engine/
│   ├── tupa-runtime/
│   ├── tupa-plugin/
│   ├── tupa-fmt/
│   ├── tupa-lint/
│   ├── tupa-audit/
│   ├── tupa-conformance/
│   └── tupa-pyffi/
├── docs/                 # Comprehensive documentation
│   ├── en/
│   │   ├── PROPOSAL.md          # Strategic rationale
│   │   ├── ARCHITECTURE.md      # System design
│   │   ├── IMPLEMENTATION_PLAN.md
│   │   ├── reference/spec.md    # Normative SPEC
│   │   └── guides/
│   └── pt-br/
├── examples/             # Sample pipelines
├── scripts/              # Build and release scripts
└── .gitignore
```

## Current Status (Phase 0 Complete)

- ✅ Minimal formal SPEC (`docs/reference/spec.md`)
- ✅ EBNF grammar and type semantics
- ✅ Conformance test suite (29 tests, 100% pass)
- ✅ JSON diagnostics infrastructure
- ✅ Parallel execution engine
- ✅ Plugin system
- ✅ `cargo-tupa` CLI (check, run, fmt, lint)

**Production deployment:** ViperTrade (live trading platform) uses Tupã as its policy layer.

## Key Documents

- [Implementation Plan](../docs/en/IMPLEMENTATION_PLAN.md) — Roadmap to v1.0.0
- [Architecture](../docs/en/ARCHITECTURE.md) — System design and crate map
- [Proposal](../docs/en/PROPOSAL.md) — Strategic pivot to crate-first
- [SPEC](../docs/reference/spec.md) — Normative language specification

## Getting Started

```bash
# Clone and build
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
cargo build --workspace

# Run examples
cargo run --example minimal

# Check conformance
cargo test -p tupa-conformance

# Use the CLI
cargo install cargo-tupa
cargo tupa check examples/minimal.tp
```

## License

MIT. See top-level [LICENSE](../../LICENSE).
