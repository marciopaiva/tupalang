# Entorno de Desarrollo (WSL Fedora 43)

## Propósito

Describir el entorno necesario para compilar y probar el proyecto en WSL Fedora 43.

## Requisitos previos

- WSL with Fedora 43
- `sudo` access
- Conexión a internet

## Instalación (paso a paso)

### 1) Actualizar repositorios

```bash
sudo dnf -y update
```

### 2) Dependencias de build

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

### 3) Rust (toolchain estable)

Instala vía rustup (recomendado):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Luego carga el entorno de Rust:

```bash
source "$HOME/.cargo/env"
```

### 4) Verificación

```bash
rustc --version
cargo --version
```

## Ejecutar pruebas localmente

```bash
cargo test -p tupa-lexer -p tupa-parser
cargo test -p tupa-typecheck
cargo test -p tupa-cli

# suite completa
cargo test
```
