# tupa-cli

Command-line interface for TupaLang.

## Install

```bash
cargo install --locked tupa-cli
```

## Basic commands

```bash
tupa --help
tupa check path/to/file.tp
tupa run path/to/file.tp
```

## Crate

- Binary name: `tupa`
- Source: [tupalang](https://github.com/marciopaiva/tupalang)

## Applied usage

- Applied reference repository: [ViperTrade](https://github.com/marciopaiva/vipertrade)
- ViperTrade uses `tupa-cli` in local and CI validation flows to typecheck and validate strategy pipelines before runtime deployment.
