# Python FFI Contract (v0.8.0)

## 1. Chamada Controlada

```tupa
@external(python="torch.nn.Linear", effects=[ExternalCall("pytorch")])
fn linear_layer(input: Tensor) -> Tensor {
    // Corpo vazio — implementação delegada para Python
}
```

### Schema Enforcement

- **Inputs Tupã** → serializados para Python via JSON/msgpack
- **Outputs Python** → validados contra tipo Tupã antes de retorno

### Tipos suportados inicialmente

- `i64`, `f64`, `bool`, `string`
- `Tensor` (wrapper mínimo para ndarray/PyTorch tensor)
- `Structs` simples (sem generics)

### Efeitos Rastreados

- Toda chamada Python recebe efeito `ExternalCall("lib_name")`
- Propagado para análise de determinismo:

```tupa
pipeline SafeInference @deterministic {
    steps: [
        step("predict") { linear_layer(input) }  // ❌ Rejeitado: ExternalCall em @deterministic
    ]
}
```

## 2. Estrutura de Crates

```bash
# Novo crate para FFI
cargo new --lib crates/tupa-pyffi
```

```toml
# crates/tupa-pyffi/Cargo.toml
[dependencies]
pyo3 = { version = "0.21", features = ["extension-module"] }
serde = "1.0"
serde_json = "1.0"
tupa-parser = { path = "../tupa-parser" } # Substitui tupa-ast
tupa-typecheck = { path = "../tupa-typecheck" }
```

## 3. Contrato de Build

```toml
# Cargo.toml (root)
[workspace.metadata.tupa]
python-min-version = "3.9"
pytorch-min-version = "2.0"  # Documentado, não enforceado ainda
```

## 4. Fluxo de Execução

```mermaid
graph TD
    Tupa[Tupã Runtime] -->|Serialize Args| PyO3[PyO3 Bridge]
    PyO3 -->|Call| Python[Python Interpreter]
    Python -->|Return Value| PyO3
    PyO3 -->|Validate Type| Validator[Schema Validator]
    Validator -->|Ok| Tupa
    Validator -->|Error| TupaError[Runtime Error]
```
