# ⚡ Tupã

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-wip-orange)](docs/ROADMAP.md)
[![Wiki](https://img.shields.io/badge/wiki-Tup%C3%A3-7b5cff)](https://github.com/marciopaiva/tupalang/wiki)
[![CI](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml/badge.svg)](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/marciopaiva/tupalang?display_name=tag)](https://github.com/marciopaiva/tupalang/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Brasil](https://img.shields.io/badge/made_in-Brazil-009739?logo=brazil)](https://github.com/marciopaiva/tupalang)

## Quick Index

- [Status](#status)
- [Features](#features)
- [Roadmap](#roadmap)
- [CLI](#cli)

## Status

- [x] Basic lexer, parser, typechecker and CLI
- [x] JSON output in CLI
- [x] Functional codegen (textual IR)
- [ ] Language Server

## Features

- **Is it production ready?** Not yet, still in development.
- **Where do I start?** See [docs/GETTING_STARTED.en.md](docs/GETTING_STARTED.en.md).
- **How to contribute?** Read [CONTRIBUTING.md](CONTRIBUTING.md).

## Roadmap

- `tupa-lexer` → tokens
- `tupa-parser` → AST
- `tupa-typecheck` → types and constraints

## CLI

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- check examples/hello.tp
```

```tupa
// AI responsible from the first character
fn summarize(article: Text) -> SafeText<!misinformation> {
  return llm.generate(f"Summarize objectively: {article}")
}
```

## ▶️ How to run locally

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
cargo test

# parse
cargo run -p tupa-cli -- parse examples/hello.tp
```
