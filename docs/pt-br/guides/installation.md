# Guia de Instalação

## Para Projetos Rust (Recomendado)

Adicione os crates do Tupã ao seu `Cargo.toml`:

```toml
[dependencies]
tupa-core = "0.9"
tupa-engine = "0.9"
```

Execute:

```bash
cargo build
```

Isso é tudo — **não há toolchain separado para instalar**. Os crates integram-se diretamente à sua build Rust.

**Versão mínima do Rust:** 1.83

---

## Verificação

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

Compile:

```bash
cargo check
```

Deve compilar sem erros.

---

## CLI (`cargo-tupa`)

O comando `cargo tupa` oferece subcomandos para trabalhar com pipelines Tupã:

```bash
cargo install cargo-tupa   # opcional, para instalação global

# Em qualquer projeto Tupã:
cargo tupa check           # verifica erros de tipo
cargo tupa run             # executa pipeline com entrada JSON
cargo tupa fmt             # formata blocos pipeline!
cargo tupa lint            # detecta problemas comuns
cargo tupa discover        # detecta binário automaticamente
```

Nota: `cargo tupa` é **opcional** — seu projeto compila sem ele. É apenas uma ferramenta de conveniência.

---

## Matriz de Versões

Sempre use versões major compatíveis (SemVer):

| tupa-core | tupa-engine | Rust MSRV | Notas |
|---|---|---|---|
| 0.9.x | 0.9.x | 1.83 | Atual (Rust-DSL only) |
| 0.8.x | 0.8.x | 1.75 | Legacy (compilador `.tp` standalone) — EOL |

A série 0.9.x é a era Rust-DSL. As versões 0.8.x e anteriores não são mais suportadas.
