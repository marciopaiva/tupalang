# Engine Examples

These examples demonstrate Tupã pipeline patterns using the Rust DSL (`pipeline!` macro).

## Examples

### credit_decision

A realistic credit decision pipeline with sequential dependencies:

```text
enrich → score → decide
```

Also includes independent metrics: `approval_rate`, `avg_score`, `risk_rate` with constraints.

**Run:**

```bash
cargo run --example credit_decision
```

**Custom input (JSON):**

```bash
TUPA_INPUT='{"amount": 50000}' cargo run --example credit_decision
```

### fraud_complete

Fraud detection pipeline working with enum (`Transaction` types) and branching logic:

```text
enrich → score → decide
```

Independent metrics: `false_negative_rate`, `false_positive_rate`.

Demonstrates pattern matching on enum inputs and producing multiple metrics.

**Run:**

```bash
cargo run --example fraud_complete
```

### minimal

Minimal pipeline with explicit `produces`/`requires` for parallel execution testing:

```text
enrich → score
```

No constraints. Shows basic DAG execution.

**Run:**

```bash
cargo run --example minimal
```

### simple

Single-step risk pipeline. Demonstrates simplest possible pipeline with a constraint.

**Run:**

```bash
cargo run --example simple
```

## Notes

- All examples accept `TUPA_INPUT` env var as JSON to override default input.
- `minimal` and `credit_decision` use `run_parallel` (async); `simple` uses `run` (sync).
- Each example includes unit tests validating metadata and execution results.
