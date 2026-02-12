# OS Environment Guide

## Purpose

Provide short setup steps per operating system.

## Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y git curl build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## macOS

```bash
xcode-select --install
brew install git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Windows (WSL)

Follow the guide in [DEV_ENV](DEV_ENV.md).

## Verification

```bash
rustc --version
cargo --version
```
