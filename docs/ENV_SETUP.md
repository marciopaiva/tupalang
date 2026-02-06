# Guia de ambiente por OS

## Objetivo

Fornecer passos resumidos de setup por sistema operacional.

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

Siga o guia em [DEV_ENV](DEV_ENV.md).

## Verificação

```bash
rustc --version
cargo --version
```
