# Guia de Configuração do Ambiente

## Propósito

Fornecer passos curtos de configuração por sistema operacional.

## Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y git curl build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```text

## macOS

```bash
xcode-select --install
brew install git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```text

## Windows (WSL)

Siga o guia em [Ambiente de desenvolvimento](dev_env.md).

## Verificação

```bash
rustc --version
cargo --version
```text
