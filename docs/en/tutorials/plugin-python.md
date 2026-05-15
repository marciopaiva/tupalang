# Writing Tupã Plugins in Python

Extend Tupã pipelines with Python step functions using `tupa-pyffi`.

**Status:** `tupa-pyffi` is in **alpha** (v0.9.x). API may change.

---

## Prerequisites

- Python 3.8+
- `tupa-pyffi` package (install from crates.io or local build)
- Rust project with `tupa-plugin` enabled

---

## Install tupa-pyffi

From PyPI (when published):

```bash
pip install tupa-pyffi
```

Or from source:

```bash
cd crates/tupa-pyffi
pip install -e .
```

---

## Write a Python Step

Create `plugins/my_plugin.py`:

```python
import tupa

@tupa.step
def double(input):
    """Double a numeric input."""
    return input * 2

@tupa.step
def sentiment_score(text: str) -> float:
    """Simple sentiment analysis (example)."""
    positive = ["good", "great", "excellent"]
    score = sum(1 for word in positive if word in text.lower())
    return float(score) / len(positive) if positive else 0.0
```

Key points:

- Decorate functions with `@tupa.step`
- Input comes as a Python object (dict, list, str, int, float, bool, None)
- Return any JSON-serializable Python object
- Errors raise `PythonException` (propagates to Rust as `PluginError`)

---

## Load the Python Plugin in Rust

```rust
use tupa_pyffi::PythonPlugin;
use tupa_plugin::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pm = PluginManager::new();

    // Initialize Python plugin subsystem
    let py_plugin = PythonPlugin::new("plugins/my_plugin.py")?;
    py_plugin.register_to(&mut pm)?;

    // Now "double" step is available
    let result = pm.call("double", serde_json::json!(21))?;
    println!("double(21) = {}", result); // 42.0

    Ok(())
}
```

Within a pipeline:

```rust
pipeline! {
    name: WithPython,
    input: f64,
    steps: [
        step("double") {
            // Call Python plugin
            let ctx = tupa_plugin::context();
            ctx.call("double", Value::from(input))?
        }
    ],
    constraints: []
}
```

---

## Python Plugin API Reference

### `@tupa.step`

Decorator marks a Python function as a Tupã step.

```python
@tupa.step
def my_step(input):
    return process(input)
```

### Error Handling

Raise exceptions to fail the step:

```python
@tupa.step
def strict_step(input):
    if input < 0:
        raise ValueError("input must be non-negative")
    return input * 2
```

The exception propagates to Rust as `EngineError::StepFailed`.

### Access to Pipeline Context

Future: `@tupa.step(context=True)` provides access to metrics, other plugins:

```python
@tupa.step(context=True)
def step_with_context(input, ctx):
    value = ctx.read_metric("previous_step_output")
    return value + input
```

---

## Limitations

- **GIL contention:** Only one Python thread runs at a time. Batch calls if possible.
- **Serialization overhead:** Input/Output are JSON-serialized `Value` objects.
- **Type safety:** Python is dynamic. Type errors surface at runtime.
- **Performance:** ~10–100x slower than native Rust steps. Use for glue code, not hot loops.

---

## Debugging

Enable Python logging:

```python
import logging
logging.basicConfig(level=logging.DEBUG)
```

Inspect plugin registration:

```rust
let functions = pm.list_functions();
println!("Available: {:?}", functions);
```

Common errors:

- `ModuleNotFoundError` — Python cannot find your plugin file; set `PYTHONPATH`
- `SymbolNotFound` — function name mismatch between Rust call and Python `@tupa.step` name
- `PickleError` — (future) serialization issues with complex objects

---

## Best Practices

1. **Keep Python steps simple:** Math, I/O, ML inference (via PyTorch/Burn)
2. **Hot paths in Rust:** Write performance-critical steps in Rust; call Python for model inference
3. **Validate inputs early:** Check types at the start of the function
4. **Graceful degradation:** Catch exceptions and return error messages
5. **Unit test Python code independently** with `pytest`

---

## Complete Example

**Python plugin** (`plugins/sentiment.py`):

```python
import tupa

@tupa.step
def analyze_sentiment(text: str) -> float:
    if "good" in text.lower():
        return 0.8
    elif "bad" in text.lower():
        return 0.2
    return 0.5
```

**Rust pipeline**:

```rust
pipeline! {
    name: SentimentCheck,
    input: String,
    steps: [
        step("sentiment") {
            let ctx = tupa_plugin::context();
            ctx.call("analyze_sentiment", Value::from(input))?
        }
    ],
    constraints: [
        metric("sentiment").ge(0.5)  // require positive sentiment
    ]
}
```

Run: `cargo tupa run --input '{"text":"good product"}'`

---

## Next Steps

- See `crates/tupa-pyffi/` for API docs and examples
- Contribute improvements to `tupa-pyffi` (open source)
- Combine Rust + Python: Rust for orchestration, Python for ML models
- Read [Rust Plugin Tutorial](./plugin-rust.md) for pure Rust approach
