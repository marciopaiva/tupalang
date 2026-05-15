# Python Plugin Example

Uses `tupa-pyffi` to expose Python step functions to the Tupã engine.

## Setup

```bash
pip install tupa-pyffi
```

## Plugin file

`my_steps.py`:

```python
import tupa

@tupa.step
def sentiment(input):
    text = input.get("text", "").lower()
    if any(w in text for w in ("good", "great", "excellent")):
        return {"sentiment": "positive"}
    if any(w in text for w in ("bad", "terrible", "awful")):
        return {"sentiment": "negative"}
    return {"sentiment": "neutral"}
```

## Load from Rust

```rust
use tupa_pyffi::PythonPlugin;

let plugin = PythonPlugin::new("my_steps.py")?;
let result = plugin.call("sentiment", json!({ "text": "great trade" }))?;
// -> {"sentiment": "positive"}
```
