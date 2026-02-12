# ⚡ Tupã

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-wip-orange)](docs/ROADMAP.md)
[![Wiki](https://img.shields.io/badge/wiki-Tup%C3%A3-7b5cff)](https://github.com/marciopaiva/tupalang/wiki)
[![CI](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml/badge.svg)](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/marciopaiva/tupalang?display_name=tag)](https://github.com/marciopaiva/tupalang/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![Brazil](https://img.shields.io/badge/made_in-Brazil-009739?logo=brazil)](https://github.com/marciopaiva/tupalang)

## Quick Index

- [Status](#status)
- [Features](#features)
- [Roadmap](#roadmap)
- [CLI](#cli-dev)
- [Resources](#resources)

## Status

- [x] Basic lexer, parser, typechecker, and CLI
- [x] JSON output in CLI
- [x] Functional codegen (textual IR)
- [ ] Language Server

## Quick FAQ

- **Is it production-ready?** Not yet, it is still under development.
- **Where do I start?** See [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md).
- **How do I contribute?** Read [CONTRIBUTING.md](CONTRIBUTING.md).

## One-minute architecture

- `tupa-lexer` → tokens
- `tupa-parser` → AST
- `tupa-typecheck` → types and constraints

## Quick demo

```tupa
let inc: fn(i64) -> i64 = |x| x + 1
print(inc(41))

let name: string = "Tupã"
print("Hello, " + name)

enum Color {
  Red,
  Green,
  Blue
}

trait Printable {
}

fn safe(x: f64): Safe<f64, !nan> {
  return x
}
```

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
cargo test
```

## parse

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
```

## check

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

## Build and coverage status

![CI](https://github.com/marciopaiva/tupalang/actions/workflows/ci.yml/badge.svg)

## Contributing

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and best practices.
2. See examples in [examples/README.md](examples/README.md).
3. Suggestions and questions: open an issue or discuss in the [FAQ](docs/FAQ.md).
4. For documentation: follow [docs/DOCS_CONTRIBUTING.md](docs/DOCS_CONTRIBUTING.md).

## lex

```bash
cargo run -p tupa-cli -- lex examples/hello.tp
```

```tupa
let age = 28
let name: string = "Ana"

let add: fn(i64, i64) -> i64 = sum

match http_status {
  200 => print("OK"),
  404 => print("Not Found"),
  code if code >= 500 => print(f"Server error: {code}"),
  _ => print("Other status")
}

spawn async fn worker(id: i64) {
  let data = await db.query(id)
  process(data)
}
```

✅ Familiar to Python/JS developers  
✅ Safe like Rust  
✅ Fast like C

---

## Use case: Fraud Detection Microservice

```tupa
@differentiable
fn risk_score(tx: Transaction) -> f64 {
  let neural = fraud_net.infer(tx.features)
  let symbolic = if tx.country == "BR" && tx.amount > 1000 { 0.8 } else { 0.2 }
  return 0.7 * neural + 0.3 * symbolic
}

@service(port=8080)
fn main() {
  route.post("/predict", |req: Request| {
    let score: Safe<f64, !nan> = risk_score(req.transaction)
    return Response::json(score)
  })
}
```

**Expected results** (vs Python + PyTorch):

| Metric | Python | Tupã | Gain |
| ------ | ------ | ---- | ---- |
| P99 latency | 45 ms | 8 ms | **5.6x faster** |
| Energy use | 100% | 12% | **88% less carbon** |
| Data leakage | Possible (runtime) | Impossible (compile time) | **Formal safety** |

---

## Next 30 days (help now)

| Task | Area | Difficulty |
| ---- | ---- | ---------- |
| Diagnostics with span/line/column (spec + implementation) | `docs/SPEC.md` | ⭐⭐ |
| Evolve typechecker (return, match, loops, function types) | `crates/tupa-typecheck/` | ⭐⭐⭐ |
| MVP codegen prototype (LLVM) | `crates/tupa-codegen/` | ⭐⭐⭐⭐ |
| Expand real examples + edge cases | `examples/` | ⭐ |

Start here: open an issue with `[RFC]` in the title to propose spec changes.

---

## Roadmap

- [docs/MVP_PLAN.md](docs/MVP_PLAN.md)
- [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md)

## Brazilian roots, global ambition

Tupã is the first Brazilian language with global ambition since Lua (1993). While Lua focused on embeddability, Tupã was born to tackle the biggest challenges in modern computing:

- **Tupi-Guarani roots**: name, logo, and philosophy inspired by ancestral wisdom
- **Sustainability**: native sparsity reduces AI carbon footprint
- **Ethical safety**: alignment is a foundation, not a feature
- **Performance**: LLVM + zero-cost abstractions = as fast as C

---

## Resources

### For users

- [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)
- [examples/README.md](examples/README.md)
- [docs/SPEC.md](docs/SPEC.md)
- [docs/GLOSSARY.md](docs/GLOSSARY.md)
- [docs/FAQ.md](docs/FAQ.md)
- [docs/ROADMAP.md](docs/ROADMAP.md)
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
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/STYLE_GUIDE.md](docs/STYLE_GUIDE.md)
- [docs/DOCS_CONTRIBUTING.md](docs/DOCS_CONTRIBUTING.md)
- [docs/CI_GUIDE.md](docs/CI_GUIDE.md)
- [docs/GOVERNANCE.md](docs/GOVERNANCE.md)
- [docs/CONTRIBUTING_FAQ.md](docs/CONTRIBUTING_FAQ.md)
- [docs/ISSUES_GUIDE.md](docs/ISSUES_GUIDE.md)

### Internals and planning

- [docs/CODEGEN.md](docs/CODEGEN.md)
- [docs/MVP_PLAN.md](docs/MVP_PLAN.md)
- [docs/ADOPTION_PLAN.md](docs/ADOPTION_PLAN.md)
- [docs/DESIGN_NOTES.md](docs/DESIGN_NOTES.md)
- [docs/CHANGELOG.md](docs/CHANGELOG.md)
- [docs/RELEASE_CHECKLIST.md](docs/RELEASE_CHECKLIST.md)
- [docs/RELEASE_GUIDE.md](docs/RELEASE_GUIDE.md)
- [docs/VERSIONING.md](docs/VERSIONING.md)

## Contributing quick checklist

- [ ] Open an issue (or `[RFC]` for large changes)
- [ ] Run `cargo test`
- [ ] Update relevant docs

---

## CLI (dev)

Demo: ![Demo](assets/demo.svg)

```bash
cargo run -p tupa-cli -- lex examples/hello.tp
cargo run -p tupa-cli -- lex --format json examples/hello.tp
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse --format json examples/hello.tp
cat examples/hello.tp | cargo run -p tupa-cli -- parse --stdin
cat examples/hello.tp | cargo run -p tupa-cli -- lex --stdin
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check --format json examples/hello.tp
cat examples/hello.tp | cargo run -p tupa-cli -- check --stdin
cargo run -p tupa-cli -- codegen examples/hello.tp
cargo run -p tupa-cli -- codegen --format json examples/hello.tp
cargo run -p tupa-cli -- version
cargo run -p tupa-cli -- about
```

---

## Diagnostics example

Errors include code and line/column:

```text
error[E2001]: type mismatch: expected I64, got Bool
  --> examples/invalid_type.tp:2:15
   |
 2 |   let x: i64 = true;
   |               ^^^^
```

JSON output is also available via `--format json` for tool integration.

---

## License

- **Compiler**: Apache License 2.0
- **Runtime**: MIT License
- **Specification**: CC-BY-SA 4.0

---

## Contributors

Coming soon.

---

## Security

See the policy in [docs/SECURITY.md](docs/SECURITY.md).

---

## Sponsors

Coming soon.

---

## Support matrix

| System | Status |
| ------ | ------ |
| Linux | ✅ |
| macOS | ✅ |
| Windows (WSL) | ✅ |

---

## Community

- [GitHub Issues](https://github.com/marciopaiva/tupalang/issues): bugs and improvements
- [Twitter @tupalang](https://twitter.com/tupalang): updates and demos

---

> **🌩️ Tupã: where ancestral wisdom meets future engineering**  
> *[github.com/marciopaiva/tupalang](https://github.com/marciopaiva/tupalang)*
