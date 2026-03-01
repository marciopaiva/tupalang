# Guía de configuración del entorno

## Propósito

Proporcionar pasos breves de configuración por sistema operativo.

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

Sigue la guía en [Entorno de desarrollo](dev_env.md).

## Verificación

```bash
rustc --version
cargo --version
```
