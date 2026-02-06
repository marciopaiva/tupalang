# Ambiente de Desenvolvimento (WSL Fedora 43)

Este documento descreve o ambiente necessário para compilar e testar o projeto no WSL Fedora 43.

## Pré-requisitos

- WSL com Fedora 43
- Acesso a `sudo`
- Conexão com a internet

## Instalação (passo a passo)

### 1) Atualize os repositórios

```bash
sudo dnf -y update
```

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
```

### 3) Rust (toolchain estável)

Instale via rustup (recomendado):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Depois, carregue o ambiente do Rust:

```bash
source "$HOME/.cargo/env"
```

### 4) Verificação

```bash
rustc --version
cargo --version
```

## Execução local dos testes

```bash
cargo test -p tupa-lexer -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli

# suite completa
cargo test
```
