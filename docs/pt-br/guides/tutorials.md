# Tutoriais passo a passo

## Propósito

Guiar usuários de diferentes níveis por tarefas comuns com o Rust DSL do Tupã
(macro `pipeline!`). A linguagem `.tp` standalone foi removida na 0.9.0.

---

## 1. Olá, Mundo

```rust
use tupa_core::pipeline;
use tupa_engine::Executor;

pipeline! {
    name: Hello,
    input: (),
    steps: [
        step("hello") { println!("Olá, Tupã!") }
    ],
    constraints: []
}

fn main() {
    let engine = Executor::new();
    engine.run(&Hello::new(), &()).unwrap();
}
```

---

## 2. Funções e closures

```rust
let inc = |x: i64| x + 1;
println!("{}", inc(41)); // 42
```

---

## 3. Trabalhando com strings

```rust
let name = "Tupã";
println!("Bem-vindo, {name}");
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

## 5. Projeto de exemplo: soma de vetores

```rust
fn sum(v: &[i64]) -> i64 {
    v.iter().sum()
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4])); // 10
}
```

Para um pipeline completo de ponta a ponta, veja
[`examples/simple_pipeline.rs`](../../../examples/simple_pipeline.rs) e os
exemplos do crate `tupa-engine`.
