# Tutoriales paso a paso

## Propósito

Guiar a usuarios de distintos niveles por tareas comunes con el Rust DSL de Tupã
(macro `pipeline!`). El lenguaje `.tp` independiente fue eliminado en 0.9.0.

---

## 1. Hola, Mundo

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;

pipeline! {
    name: Hello,
    input: (),
    steps: [
        step("hello") { println!("¡Hola, Tupã!") }
    ],
    constraints: []
}

fn main() {
    let engine = Executor::new();
    engine.run(&Hello::new(), &()).unwrap();
}
```

---

## 2. Funciones y closures

```rust
let inc = |x: i64| x + 1;
println!("{}", inc(41)); // 42
```

---

## 3. Trabajando con strings

```rust
let name = "Tupã";
println!("Bienvenido, {name}");
```

---

## 4. Tipos Safe

```rust
use tupa_core::Safe;

struct NonNan;
let x = Safe::<f64, NonNan>::new(3.14);
println!("{}", x.get());
```

---

## 5. Proyecto de ejemplo: suma de vectores

```rust
fn sum(v: &[i64]) -> i64 {
    v.iter().sum()
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4])); // 10
}
```

Para un pipeline completo de extremo a extremo, vea
[`examples/simple_pipeline.rs`](../../../examples/simple_pipeline.rs) y los
ejemplos del crate `tupa-engine`.
