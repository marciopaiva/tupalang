# Guia de Instalação

## Caminho recomendado (binário standalone)

Baixe um artefato de release e coloque no seu `PATH`.

### Linux x86_64

```bash
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-linux-x86_64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```text

### macOS arm64

```bash
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-macos-aarch64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```text

### Windows x86_64

Baixe `tupa-windows-x86_64.exe` em Releases e adicione ao seu `PATH`.

## Verificar instalação

```bash
tupa --help
```text

## Caminho para desenvolvedores Rust (Cargo)

```bash
cargo install tupa-cli
```text

Se instalar via Cargo, o executável normalmente é `tupa-cli`.
