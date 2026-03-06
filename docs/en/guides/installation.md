# Installation Guide

## Recommended Path (Standalone Binary)

Download a release artifact and place it on your `PATH`.

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

Download `tupa-windows-x86_64.exe` from Releases and add it to your `PATH`.

## Verify Installation

```bash
tupa --help
```

## Rust Developer Path (Cargo)

```bash
cargo install tupa-cli
```

If installed via Cargo, the executable is usually `tupa-cli`.
