# Guía de Instalación

## Ruta recomendada (binario standalone)

Descarga un artifact de release y colócalo en tu `PATH`.

### Linux x86_64

```bash
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-linux-x86_64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```

### macOS arm64

```bash
curl -L https://github.com/marciopaiva/tupalang/releases/latest/download/tupa-macos-aarch64 -o /usr/local/bin/tupa
chmod +x /usr/local/bin/tupa
```

### Windows x86_64

Descarga `tupa-windows-x86_64.exe` en Releases y agrégalo a tu `PATH`.

## Verificar instalación

```bash
tupa --help
```

## Ruta para desarrolladores Rust (Cargo)

```bash
cargo install tupa-cli
```

Si instalas por Cargo, el ejecutable normalmente es `tupa-cli`.
