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
```text

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
```text

### 3) Rust (toolchain estable)

Instala vía rustup (recomendado):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```text

Luego carga el entorno de Rust:

```bash
source "$HOME/.cargo/env"
```text

### 4) Verificación

```bash
rustc --version
cargo --version
```text

## Ejecutar pruebas localmente

```bash
cargo test -p tupa-core -p tupa-core-macros
cargo test -p tupa-engine
cargo test -p cargo-tupa

# suite completa
cargo test
```text
