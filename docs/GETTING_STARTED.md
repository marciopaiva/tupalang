# Guia de Início Rápido

## Objetivo

Dar o caminho mínimo para compilar o projeto e executar o primeiro exemplo.

## Pré-requisitos

- Rust estável (via rustup)
- Git

## Passos

1) Clone o repositório:

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

1) Execute o exemplo básico:

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse examples/lambda_basic.tp
```

1) Rode o typechecker:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check examples/lambda_basic.tp
```

1) Saída em JSON (opcional):

```bash
cargo run -p tupa-cli -- parse --format json examples/hello.tp
```

1) Rode os testes golden (recomenda-se para validar toda a pipeline):

```bash
cargo test -p tupa-cli --test cli_golden
```

## Próximos passos

- Leia a [SPEC](SPEC.md)
- Explore os [Exemplos](../examples/README.md)
- Configure o ambiente em [DEV_ENV](DEV_ENV.md)
