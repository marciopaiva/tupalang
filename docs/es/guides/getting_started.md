# Guía de inicio rápido

## Propósito

Proporcionar el camino mínimo para compilar el proyecto y ejecutar el primer ejemplo con Rust DSL.

## Prerrequisitos

- Rust estable (vía rustup)
- Git

## Pasos

### 1) Clona el repositorio

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

### 2) Ejecuta el ejemplo básico

```bash
cargo run --example minimal
```

### 3) Verifica el pipeline

```bash
cargo tupa check        # si tienes el CLI instalado globalmente
# o simplemente compila:
cargo build
```

### 4) Ejecuta pruebas

```bash
cargo test --workspace --locked
```

## Primer pipeline en tu proyecto

```bash
cargo new my-strategy --lib
cd my-strategy
```

Añade a `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.9"
tupa-engine = "0.9"
```

Crea `src/lib.rs`:

```rust
use tupa_core::pipeline;

pipeline! {
    name: HolaMundo,
    input: (),
    steps: [
        step("hola") { println!("Hola, Tupã!") }
    ],
    constraints: []
}
```

Compila y ejecuta:

```bash
cargo run
```

---

## Próximos pasos

- Explora [Ejemplos](../../examples/README.md)
- Lee la [Guía de Pipeline](pipeline_guide.md)
- Consulta la [Documentación de API](https://docs.rs/tupa-core)
- Revisa [TRANSITION.md](../TRANSITION.md) si vienes de `.tp`
- Únete a [Comunidad](https://github.com/marciopaiva/tupalang/discussions)
