# Quick Start Guide

## Purpose

Provide the minimal path to build the project and run the first example.

## Prerequisites

- Stable Rust (via rustup)
- Git

## Steps

1) Clone the repository:

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

1) Run the basic example:

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse examples/lambda_basic.tp
```

1) Run the typechecker:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check examples/lambda_basic.tp
```

1) JSON output (optional):

```bash
cargo run -p tupa-cli -- parse --format json examples/hello.tp
```

1) Run golden tests (recommended to validate the full pipeline):

```bash
cargo test -p tupa-cli --test cli_golden
```

## Next steps

- Read the [SPEC](SPEC.md)
- Explore [Examples](../examples/README.md)
- Set up the environment in [DEV_ENV](DEV_ENV.md)
