# Motor de Auditoria

## Propósito

Descrever o hash de auditoria determinístico usado para gerar a impressão das execuções.

## Entradas

O hash de auditoria combina:

- AST normalizada (ordem estável de campos, sem spans)
- Entradas JSON canônicas (chaves de objeto ordenadas)
- String de versão do compilador

## Saída

O hash de saída é uma string hex SHA3-256. O CLI retorna:

- `hash`: hash da execução (AST + entradas + versão)
- `ast_fingerprint`: hash apenas da AST (AST + versão)
- `compiler_version`: versão do compilador usada no hash

## Exemplo

Fonte:

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

Critérios de aceitação:

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

Dada a mesma fonte, versão do compilador e entradas, o hash é estável entre máquinas.
