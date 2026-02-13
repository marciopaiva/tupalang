# ⚡ Tupã (TupaLang)

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-wip-orange)](docs/ROADMAP.md)
[![Wiki](https://img.shields.io/badge/wiki-Tup%C3%A3-7b5cff)](https://github.com/marciopaiva/tupalang/wiki)
[![CI](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml/badge.svg)](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/marciopaiva/tupalang?display_name=tag)](https://github.com/marciopaiva/tupalang/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Brazil](https://img.shields.io/badge/made_in-Brazil-009739?logo=brazil)](https://github.com/marciopaiva/tupalang)

> Deterministic language for orchestration, validation, and auditing of critical AI pipelines.

## Index

- [Vision](#vision)
- [The problem](#the-problem)
- [The Tupã proposal](#the-tupã-proposal)
- [Current status](#current-status)
- [Quick example](#quick-example)
- [How to run](#how-to-run)
- [Roadmap](#roadmap)
- [Resources](#resources)
- [Contributing](#contributing)

## Vision

Tupã is an application language focused on governance, determinism, and auditability for AI pipelines. It does not replace PyTorch, TensorFlow, or JAX. It coordinates, validates, and formalizes their use.

## The problem

AI pipelines are still fragile for regulated environments:

- Loose, non-deterministic scripts
- Executions that are hard to audit
- Weak reproducibility
- Inconsistent validators

In fintech, healthcare, defense, and insurance, this is not acceptable.

## The Tupã proposal

Tupã provides:

- Explicit determinism
- Formal constraints as first-class citizens
- Integrated auditing
- Safe orchestration of the existing ecosystem

## Current status

Implemented:

- Lexer, parser, typechecker, and CLI
- JSON output in the CLI
- Functional codegen (textual IR)

In progress (0.6.0):

- Pipeline language and orchestration
- Audit and hashing engine
- Controlled Python integration
- Language Server

## Quick example

```tupa
fn safe_text(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}

let inc: fn(i64) -> i64 = |x| x + 1
print(inc(41))
```

## How to run

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
cargo test
```

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
```

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

## Roadmap

- [docs/MVP_PLAN.md](docs/MVP_PLAN.md)
- [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)

## Resources

### For users

- [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)
- [examples/README.md](examples/README.md)
- [docs/SPEC.md](docs/SPEC.md)
- [docs/GLOSSARY.md](docs/GLOSSARY.md)
- [docs/FAQ.md](docs/FAQ.md)
- [docs/README.md](docs/README.md)
- [Wiki](https://github.com/marciopaiva/tupalang/wiki)

### For contributors

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- [docs/DEV_ENV.md](docs/DEV_ENV.md)
- [docs/DIAGNOSTICS_CHECKLIST.md](docs/DIAGNOSTICS_CHECKLIST.md)
- [docs/DIAGNOSTICS_GLOSSARY.md](docs/DIAGNOSTICS_GLOSSARY.md)
- [docs/TESTING.md](docs/TESTING.md)
- [docs/ERROR_MESSAGES.md](docs/ERROR_MESSAGES.md)

## Contributing

1. Read [CONTRIBUTING.md](CONTRIBUTING.md).
2. See examples in [examples/README.md](examples/README.md).
3. Suggestions and questions: open an issue or use the [FAQ](docs/FAQ.md).
4. Documentation: follow [docs/DOCS_CONTRIBUTING.md](docs/DOCS_CONTRIBUTING.md).

## License

Apache License 2.0. See [LICENSE](LICENSE).
