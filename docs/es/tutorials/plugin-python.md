# Escribiendo Plugins Tupã en Python

Extienda pipelines con funciones de step Python usando `tupa-pyffi`.

**Estado:** `tupa-pyffi` está en **alpha** (v0.9.x). API puede cambiar.

---

## Prerrequisitos

- Python 3.8+
- `tupa-pyffi` (pip o source)
- Proyecto Rust con `tupa-plugin`

---

## Instalar

```bash
pip install tupa-pyffi
```

O desde source:

```bash
cd crates/tupa-pyffi
pip install -e .
```

---

## Plugin Python

`plugins/mi_plugin.py`:

```python
import tupa

@tupa.step
def doblar(valor):
    """Doble un número."""
    return valor * 2

@tupa.step
def analisis_sentimiento(texto: str) -> float:
    if "bueno" in texto.lower():
        return 0.8
    elif "malo" in texto.lower():
        return 0.2
    return 0.5
```

Puntos:

- Decore con `@tupa.step`
- Recibe objetos Python (dict, list, str, int, float, bool, None)
- Retorne cualquier objeto JSON-serializable
- Excepciones propagan a Rust como `PluginError`

---

---

## Limitaciones

- **GIL:** Solo un thread Python a la vez
- **Serialización:** Overhead de JSON en cada llamada
- **Tipado:** Dinámico — errores solo en runtime
- **Performance:** 10–100x más lento que Rust. Use para ML/I/O, no loops calientes

---

## Mejores Prácticas

1. Steps Python simples — ML, I/O, glue code
2. Paths críticos en Rust
3. Valide entradas early
4. Capture excepciones y retorne errores claros
5. Teste com `pytest` independientemente

---

## Depuración

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

Errores comunes:

- `ModuleNotFoundError` — `PYTHONPATH` incorrecto
- `SymbolNotFound` — nombre función no coincide con `@tupa.step`
- `PickleError` — objetos complejos no serializables

---

## Próximos Pasos

- Crate `tupa-pyffi` docs
- [Plugin Rust](./plugin-rust.md) para enfoque nativo
- Combine Rust (orquestración) + Python (ML)
