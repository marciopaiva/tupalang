# Motor de Auditoría

## Propósito

Describir el hash de auditoría determinístico usado para generar la huella de ejecuciones.

## Entradas

El hash de auditoría combina:

- AST normalizada (orden estable de campos, sin spans)
- Entradas JSON canónicas (claves de objeto ordenadas)
- Cadena de versión del compilador

## Salida

La salida del hash es una cadena hex SHA3-256. El CLI devuelve:

- `hash`: hash de ejecución (AST + entradas + versión)
- `ast_fingerprint`: hash solo de AST (AST + versión)
- `compiler_version`: versión del compilador usada para el hash

## Ejemplo

Fuente:

```tupa
fn main() {
  let x = 1;
  print(x);
}
```

Entradas:

```json
[
  1,
  "ok",
  {
    "b": 2,
    "a": 1
  }
]
```

CLI:

```bash
cargo run -p tupa-cli -- audit examples/audit_hello.tp --input examples/audit_inputs.json
cargo run -p tupa-cli -- audit --format json examples/audit_hello.tp --input examples/audit_inputs.json
```

Criterios de aceptación:

```bash
tupa audit examples/pipeline.tp --input=data.json
```

## API de biblioteca

```rust
use serde_json::Value;
use tupa_audit::hash_execution;
use tupa_parser::parse_program;

let program = parse_program("fn main() { let x = 1; }").unwrap();
let inputs = vec![Value::from(1)];
let hash = hash_execution(&program, &inputs);
println!("{hash}");
```

## Determinismo

Dada la misma fuente, versión del compilador y entradas, el hash es estable entre máquinas.
