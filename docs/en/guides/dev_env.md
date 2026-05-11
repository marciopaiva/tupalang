# Development Environment (WSL Fedora 43)

## Purpose

This document describes the environment needed to build and test the project on WSL Fedora 43.

## Prerequisites

- WSL with Fedora 43
- `sudo` access
- Internet connection

## Installation Steps

### 1) Update repositories

```bash
sudo dnf -y update
```text

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
```text

### 3) Rust (stable toolchain)

Install via rustup (recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```text

Then load the Rust environment:

```bash
source "$HOME/.cargo/env"
```text

### 4) Verification

```bash
rustc --version
cargo --version
```text

## Running tests locally

```bash
cargo test -p tupa-lexer -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli

# full suite
cargo test
```text

## Local CI

If your host already has the required tooling, run:

```bash
./scripts/ci-local.sh
```text

If you want a reproducible environment closer to GitHub Actions, use:

```bash
./scripts/ci-local-container.sh
```text

This path avoids relying on host-installed `rustfmt`, `clippy`, `markdownlint`, and `lychee`.
