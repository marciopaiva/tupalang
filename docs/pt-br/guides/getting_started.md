# Guia de Início Rápido

## Propósito

Fornecer o caminho mínimo para compilar o projeto e executar o primeiro exemplo.

## Pré-requisitos

- Rust estável (via rustup)
- Git

## Passos

1) Clone o repositório:

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```text

1) Execute o exemplo básico:

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse examples/lambda_basic.tp
```text

1) Execute o verificador de tipos:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check examples/lambda_basic.tp
```text

1) Saída JSON (opcional):

```bash
cargo run -p tupa-cli -- parse --format json examples/hello.tp
```text

1) Execute os testes goldens (recomendado para validar o pipeline completo):

```bash
cargo test -p tupa-cli --test cli_golden
```text

## Próximos passos

- Leia a [SPEC](spec.md)
- Explore [Exemplos](../examples/README.md)
- Configure o ambiente em [Ambiente de desenvolvimento](dev_env.md)
