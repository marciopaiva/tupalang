# Escrevendo Plugins Tupã em Rust

Plugins permitem estender pipelines Tupã com funções de step customizadas em Rust, carregadas em runtime sem recompilar o binário principal.

## Arquitetura do Plugin

Um plugin é uma biblioteca dinâmica (`cdylib`) que exporta dois símbolos obrigatórios:

- `_tupa_plugin_name()` — retorna o nome do plugin como string C
- `_tupa_plugin_register(registry)` — registra funções de step no engine

O sistema usa ABI C para carregamento independente de linguagem.

---

## Criar Projeto de Plugin

```bash
cargo new meu_plugin_tupa --lib
cd meu_plugin_tupa
```

`Cargo.toml`:

```toml
[lib]
name = "meu_plugin_tupa"
crate-type = ["cdylib"]

[dependencies]
tupa-plugin = "0.9"
serde_json = "1.0"
```

---

## Implementar o Plugin

`src/lib.rs`:

```rust
use tupa_plugin::{PluginRegistry, PluginError};
use serde_json::Value;

#[no_mangle]
pub extern "C" fn _tupa_plugin_name() -> *const u8 {
    b"meu_plugin\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn _tupa_plugin_register(registry: &mut PluginRegistry) {
    registry
        .register("dobro", step_dobro)
        .expect("falha ao registrar 'dobro'");
}

pub fn step_dobro(input: Value) -> Result<Value, PluginError> {
    let num = input.as_f64().ok_or(PluginError::TypeError("esperado f64".into()))?;
    Ok(Value::from(num * 2.0))
}
```

### Assinatura de Step Function

Todas as funções de step devem ter:

```rust
pub fn nome_step(input: Value) -> Result<Value, PluginError>
```

- `Value` é `serde_json::Value` — dados dinâmicos tipo JSON
- `Ok(Value)` sucesso, `Err(PluginError)` falha
- Use `PluginError::type_error()` para erros de tipo
- Use `PluginError::runtime()` para outros erros

---

## Build

```bash
cargo build --release --crate-type cdylib
```

Saída:

- Linux: `target/release/libmeu_plugin_tupa.so`
- macOS: `target/release/libmeu_plugin_tupa.dylib`
- Windows: `target/release/meu_plugin_tupa.dll`

---

## Carregar no Seu Projeto

```rust
use tupa_plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pm = PluginManager::new();
    pm.load_plugin("./target/release/libmeu_plugin_tupa.so")?;

    let result = pm.call("dobro", serde_json::json!(21.0))?;
    println!("Resultado: {}", result); // 42.0

    Ok(())
}
```

Dentro de `pipeline!`:

```rust
pipeline! {
    name: ComPlugin,
    input: f64,
    steps: [
        step("dobro") {
            let ctx = tupa_plugin::context();
            ctx.call("dobro", Value::from(input))?
        }
    ],
    constraints: []
}
```

---

## Exemplo Completo

Veja `crates/tupa-plugin/tests/plugin_src/` para exemplos funcionais.

---

## Depuração

Verifique símbolos exportados:

```bash
nm target/release/libmeu_plugin_tupa.so | grep _tupa_plugin
```

Erros comuns:

- `PluginNotFound` — caminho incorreto
- `SymbolNotFound` — falta `#[no_mangle]` ou nome errado
- Versão incompatível — recompile com mesma versão `tupa-plugin`

---

## Boas Práticas

- Mantenha steps puros (sem side effects)
- Valide tipos no início da função
- Retorne mensagens de erro descritivas
- Teste funções independentemente com `#[test]`
- Use Rust para hot paths; Python só para I/O/ML (via `tupa-pyffi`)

---

## Plugins Python

Veja [Plugin Python](../tutorials/plugin-python.md) (usando `tupa-pyffi`).

---

## Próximos Passos

- Docs da crate `tupa-plugin`: <https://docs.rs/tupa-plugin>
- [Pipeline Guide](../guides/pipeline_guide.md)
- Contribua seu plugin ao ecossistema!
