# Guía de Instalación

## Para Proyectos Rust (Recomendado)

Añade los crates de Tupã a tu `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
```

Ejecuta:

```bash
cargo build
```

Listo — **no hay toolchain separado para instalar**. Los crates se integran directamente en tu build de Rust.

**Versión mínima de Rust:** 1.83

---

## Verificar

Crea `src/lib.rs`:

```rust
use tupa_core::pipeline;

pipeline! {
    name: Hola,
    input: (),
    steps: [
        step("hola") { println!("¡Hola, Tupã!") }
    ],
    constraints: []
}
```

Compila:

```bash
cargo check
```

Debería compilar sin errores.

---

## CLI (`cargo-tupa`)

El comando `cargo tupa` proporciona subcomandos para trabajar con pipelines Tupã:

```bash
cargo install cargo-tupa   # opcional, para instalar globalmente

# En cualquier proyecto Tupã:
cargo tupa check           # verifica tipos
cargo tupa run             # ejecuta pipeline con entrada JSON
cargo tupa fmt             # formatea bloques pipeline!
cargo tupa lint            # detecta problemas comunes
cargo tupa discover        # detecta binario automáticamente
```

Nota: `cargo tupa` es **opcional** — tu proyecto compila sin él. Es solo una ayuda de desarrollo.

---

## Matriz de Versiones

Siempre usa versiones mayores compatibles (SemVer):

| tupa-core | tupa-engine | Rust MSRV | Notas |
|---|---|---|---|
| 0.9.x | 0.9.x | 1.83 | Actual (Rust-DSL only) |
| 0.8.x | 0.8.x | 1.75 | Legacy (compilador `.tp` standalone) — EOL |

La serie 0.9.x es la era Rust-DSL. Las versiones legacy 0.8.x y anteriores ya no se soportan.
