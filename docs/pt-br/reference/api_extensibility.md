# API do Compilador e Extensibilidade

## Propósito

Explicar como usar a API do compilador do Tupã, estender funcionalidades e fazer embedding de Tupã em sistemas Rust.

## Superfície estável de embedding (`v0.8.2`)

A superfície estável de embedding para esta release é:

- `tupa-parser`
- `tupa-typecheck`
- `tupa-runtime`
- `tupa-codegen`

Para exemplos mínimos, veja [Embedding](embedding.md).

## Uso como biblioteca

Cada crate pode ser usada como biblioteca Rust independente:

```rust
use tupa_parser::parse;
use tupa_typecheck::typecheck;
use tupa_codegen::codegen;

let ast = parse("fn main() { print(42) }")?;
let typed = typecheck(&ast)?;
let ir = codegen(&typed)?;
```

## Pontos de extensão

### Built-in Functions

TupaLang fornece helpers embutidos acessíveis via namespace `tupa::`:

- `tupa::weighted(score, weight, reason)` — score ponderado com reason
- `tupa::warn(reason)` — aprovação com aviso
- `tupa::pass(reason)` — aprovação pura com motivo
- `tupa::confirm(observed, consecutive, required, reason)` — política de confirmação consecutiva
- `tupa::cooldown(active, remaining_seconds, reason)` — bloqueio por cooldown temporal

Essas funções são registradas em `Runtime::new()` e podem ser chamadas de qualquer step do pipeline.

### Custom Extensions

Implemente o trait `TupaExtension` (`tupa-runtime/src/extensions.rs`):

```rust
use tupa_runtime::{Runtime, TupaExtension};

pub struct MeusHelpers;
impl TupaExtension for MeusHelpers {
    fn name(&self) -> &str { "meu_projeto" }
    fn register(&self, runtime: &Runtime) {
        runtime.register_step("meu::helper", |input| {
            // lógica customizada
            Ok(input)
        });
    }
}
```

Chame `MeusHelpers.register(&runtime)` durante a inicialização.

### Plugin System

Carregamento dinâmico de plugins (`tupa-plugin` crate):

```rust
use tupa_plugin::PluginManager;

let mut manager = PluginManager::new();
manager.load_plugin("./plugins/meu_plugin.so")?;
manager.register_all(&runtime);
```

Plugins são bibliotecas compartilhadas que exportam `_tupa_plugin_name` e `_tupa_plugin_register`.

### Schema Registry

Schemas versionados com suporte a migrações (`tupa-codegen/src/schema_registry.rs`):

```rust
use tupa_codegen::schema_registry::{SchemaRegistry, SchemaVersion};

let mut registry = SchemaRegistry::new();
registry.register_schema(
    "TradingConfig",
    "0.1.0",
    schema,
    migrations,
)?;
```

Schemas evoluem entre versões de pipeline com warnings de depreciação.

### Hot Reload

Observação de arquivos para hot reload (`tupa-runtime/src/hot_reload.rs`):

```rust
let (tx, rx) = runtime.watch_and_reload("./strategies")?;
// Receptor notifica mudanças; call reload_pipeline() para aplicar
```

Habilitado com feature flag:

```bash
cargo add tupa-runtime --features hot-reload
```

## Exemplo: Adicionar um Backend WASM

1. Criar uma nova crate `tupa-backend-wasm`.
2. Implementar o trait `CodegenBackend`.
3. Integrar no CLI.

## Links úteis

- [Embedding](embedding.md)
- [Codegen](codegen.md)
- [Contribuição](../../CONTRIBUTING.md)
