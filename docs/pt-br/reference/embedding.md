# Embedding de Tupã em Rust

## Propósito

Descrever a superfície suportada de embedding para `v0.8.2`.

## Crates públicas suportadas

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

Estas crates são a superfície estável de embedding para esta release.

## Extension API

Projetos podem definir funções de passo customizadas via o trait `TupaExtension`:

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MeusHelpers;
impl TupaExtension for MeusHelpers {
    fn name(&self) -> &str { "meu_projeto" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("meu::helper", |input| {
            // lógica de negócio
            Ok(serde_json::json!({ "status": "ok" }))
        });
    }
}

// Durante a inicialização
MeusHelpers.register(&runtime);
```text

## Plugin System

Carregamento dinâmico de plugins (`tupa-plugin`):

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./plugins/meu_plugin.so")?;

// Em um passo do pipeline:
// pm.call("meu_step", json!(input))?
```text

Plugins são bibliotecas compartilhadas que exportam `_tupa_plugin_name` e `_tupa_plugin_register`.

## Hot Reload

Habilite o feature `hot-reload` para observar mudanças em arquivos:

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// Notifica mudanças automaticamente
```text

## Exemplo mínimo

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;

fn main() -> anyhow::Result<()> {
    let src = "fn main() { print(1) }";
    let ast = parse(src)?;
    let _typed = typecheck(&ast)?;
    Ok(())
}
```text

## Notas de compatibilidade

- Siga SemVer conforme [Versionamento](versioning.md).
- Evite depender de crates internas não listadas acima se você precisa de estabilidade de API.
