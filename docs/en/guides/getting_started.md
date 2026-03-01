# Quick Start

## Purpose

This document provides the shortest path to build the project and run your first example.

## Prerequisites

- Stable Rust (via rustup)
- Git

## Steps

1) Clone the repository:

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

2) Run the basic example:

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse examples/lambda_basic.tp
```

3) Run the typechecker:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check examples/lambda_basic.tp
```

4) JSON output (optional):

```bash
cargo run -p tupa-cli -- parse --format json examples/hello.tp
```

5) Run golden tests (recommended for full pipeline validation):

```bash
cargo test -p tupa-cli --test cli_golden
```

## Next Steps

- Read the [SPEC](../reference/spec.md)
- Explore [Examples](../../examples/README.md)
- Set up the environment in [Dev Environment](dev_env.md)
