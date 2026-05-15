# Pipeline Examples (Legacy `.tp` Files)

**⚠️ DEPRECATED:** These `.tp` examples are from the pre-0.9.0 era. They are **no longer supported** in the current workspace. New development should use Rust-DSL examples instead.

## Legacy Files

These examples are kept for historical reference only:

- `fraud_complete.tp` — fraud pipeline with constraints and validation
- `credit_decision.tp` — credit decision with 3 constraints
- `loan_underwriting.tp` — underwriting with risk metrics
- `customer_churn.tp` — churn and retention metrics
- `config_driven_strategy.tp` — typed nested input pattern
- `temporal_policy.tp` — temporal policy pattern with confirmation/cooldown

## Legacy Commands (do not use)

```bash
# These commands relied on tupa-cli which was removed in 0.9.0
tupa codegen --format=json examples/pipeline/fraud_complete.tp
tupa run --pipeline=FraudDetection --input examples/pipeline/tx.json examples/pipeline/fraud_complete.tp
```

## Current Examples (Rust-DSL)

See active Rust examples in the workspace:

- `crates/tupa-engine/examples/minimal.rs` — basic sequential pipeline
- `crates/tupa-engine/examples/vipertrade_smoke.rs` — ViperTrade-style pipeline

**Run current examples:**

```bash
cargo run --example minimal --package tupa-engine
cargo run --example vipertrade_smoke --package tupa-engine
```

For custom pipelines, create your own Rust crate with `tupa-core` and `tupa-engine` dependencies and write a `pipeline!` macro block. See [Getting Started](../docs/en/guides/getting_started.md).
