# Ambiente de Desenvolvimento (WSL Fedora 43)

## Propósito

Descrever o ambiente necessário para compilar e testar o projeto no WSL Fedora 43.

## Pré-requisitos

- WSL with Fedora 43
- `sudo` access
- Conexão com a internet

## Instalação (passo a passo)

### 1) Atualizar repositórios

```bash
sudo dnf -y update
```text

### 2) Dependências de build

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

### 3) Rust (toolchain estável)

Instale via rustup (recomendado):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```text

Em seguida, carregue o ambiente Rust:

```bash
source "$HOME/.cargo/env"
```text

### 4) Verificação

```bash
rustc --version
cargo --version
```text

## Rodando testes localmente

```bash
cargo test -p tupa-core -p tupa-core-macros
cargo test -p tupa-engine
cargo test -p cargo-tupa

# suite completa
cargo test
```text
