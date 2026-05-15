# Escrevendo Plugins Tupã em Python

Estenda pipelines com funções de step em Python usando `tupa-pyffi`.

**Status:** `tupa-pyffi` está em **alpha** (v0.9.x). API pode mudar.

---

## Pré-requisitos

- Python 3.8+
- `tupa-pyffi` (instalar via pip ou source)
- Projeto Rust com `tupa-plugin`

---

## Instalar

```bash
pip install tupa-pyffi
```

Ou do source:

```bash
cd crates/tupa-pyffi
pip install -e .
```

---

## Plugin Python

`plugins/meu_plugin.py`:

```python
import tupa

@tupa.step
def dobrar(valor):
    """Dobra um número."""
    return valor * 2

@tupa.step
def analise_sentimento(texto: str) -> float:
    if "bom" in texto.lower():
        return 0.8
    elif "ruim" in texto.lower():
        return 0.2
    return 0.5
```

Pontos:

- Decore com `@tupa.step`
- Recebe objetos Python (dict, list, str, int, float, bool, None)
- Retorne qualquer objeto JSON-serializável
- Exceções propagam para Rust como `PluginError`

---

## Carregar no Rust

```rust
use tupa_pyffi::PythonPlugin;
use tupa_plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pm = PluginManager::new();
    let py_plugin = PythonPlugin::new("plugins/meu_plugin.py")?;
    py_plugin.register_to(&mut pm)?;

    let result = pm.call("dobrar", serde_json::json!(21))?;
    println!("Resultado: {}", result); // 42.0

    Ok(())
}
```

---

## Limitações

- **GIL:** Apenas uma thread PythonExecuta por vez
- **Serialização:** Overhead de JSON em cada chamada
- **Tipagem:** Dinâmica — erros só em runtime
- **Performance:** 10–100x mais lento que Rust. Use para ML/I/O, não loops quentes

---

## Melhores Práticas

1. Steps Python simples — ML, I/O, glue code
2. Paths críticos em Rust
3. Valide entradas early
4. Capture exceções e retorne erros claros
5. Teste com `pytest` separadamente

---

## Depuração

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

Erros comuns:

- `ModuleNotFoundError` — `PYTHONPATH` incorreto
- `SymbolNotFound` — nome da função não bate com `@tupa.step`
- `PickleError` — objetos complexos não serializáveis

---

## Próximos Passos

- Crate `tupa-pyffi` docs
- [Plugin Rust](../tutorials/plugin-rust.md) para abordagem nativa
- Combine Rust (orquestração) + Python (ML)
