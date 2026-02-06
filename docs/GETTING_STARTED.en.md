# Getting Started

## Requirements
- Rust (latest stable)
- Cargo

## Quick Start

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
cargo test
```

## Running the CLI

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- check examples/hello.tp
```

## Example

```tupa
fn add(x: int, y: int): int {
  return x + y
}

print(add(2, 3))
```

## More
- See [../README.en.md](../README.en.md)
- See [docs/FAQ.en.md](FAQ.en.md)
