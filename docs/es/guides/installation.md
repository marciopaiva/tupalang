# Guía de Instalación

## Ruta recomendada (binario standalone)

Descarga un artifact de release y colócalo en tu `PATH`.

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

Descarga `tupa-windows-x86_64.exe` en Releases y agrégalo a tu `PATH`.

## Verificar instalación

```bash
tupa --help
```text

## Ruta para desarrolladores Rust (Cargo)

```bash
cargo install tupa-cli
```text

Si instalas por Cargo, el ejecutable normalmente es `tupa-cli`.
