# Guia de Início Rápido

## Propósito

Fornecer o caminho mínimo para compilar o projeto e executar o primeiro exemplo com Rust DSL.

## Pré-requisitos

- Rust estável (via rustup)
- Git

## Passos

### 1) Clone o repositório

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

### 2) Execute o exemplo básico

```bash
cargo run --example minimal
```

### 3) Verifique o pipeline

```bash
cargo tupa check        # com CLI instalado globalmente
# ou simplesmente:
cargo build
```

### 4) Execute os testes

```bash
cargo test --workspace --locked
```

## Primeiro pipeline no seu projeto

```bash
cargo new my-strategy --lib
cd my-strategy
```

Adicione ao `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
```

Crie `src/lib.rs`:

```rust
use tupa_core::pipeline;

pipeline! {
    name: OlaMundo,
    input: (),
    steps: [
        step("ola") { println!("Olá, Tupã!") }
    ],
    constraints: []
}
```

Compile e execute:

```bash
cargo run
```

---

## Próximos passos

- Leia a [SPEC](../reference/spec.md)
- Explore [Exemplos](../../examples/README.md)
- Configure o ambiente em [Ambiente de desenvolvimento](dev_env.md)
- Participe da [Comunidade](https://github.com/marciopaiva/tupalang/discussions)
