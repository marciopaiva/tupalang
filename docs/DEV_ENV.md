# Development Environment (WSL Fedora 43)

## Purpose

Describe the environment needed to build and test the project on WSL Fedora 43.

## Prerequisites

- WSL with Fedora 43
- `sudo` access
- Internet connection

## Installation (step by step)

### 1) Update repositories

```bash
sudo dnf -y update
```

### 2) Build dependencies

```bash
sudo dnf -y install \
  git \
  curl \
  gcc \
  gcc-c++ \
  make \
  openssl-devel \
  pkgconf-pkg-config
```

### 3) Rust (stable toolchain)

Install via rustup (recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then load the Rust environment:

```bash
source "$HOME/.cargo/env"
```

### 4) Verification

```bash
rustc --version
cargo --version
```

## Running tests locally

```bash
cargo test -p tupa-lexer -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli

# full suite
cargo test
```
