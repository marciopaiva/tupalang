# Step-by-step Tutorials

## Purpose

Guide users of different levels through common tasks and sample projects with Tupã.

---

## 1. Hello, World

```tupa
print("Hello, Tupã!")
```

Run:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

---

## 2. Functions and Lambdas

```tupa
let inc: fn(int) -> int = |x| x + 1
print(inc(41)) // output: 42
```

---

## 3. Working with Strings

```tupa
let name = "Tupã"
print("Welcome, " + name)
```

---

## 4. Functions with Safe Types

```tupa
fn safe(x: f64): Safe<f64, !nan> {
  return x
}
```

---

## 5. Example Project: Vector Sum

File: `examples/soma_vetor.tp`

```tupa
fn sum(v: [int]) -> int {
  let mut total = 0
  for x in v {
    total = total + x
  }
  return total
}
print(sum([1,2,3,4])) // output: 10
```

---

## 6. Debugging and Diagnostics

- See [docs/COMMON_ERRORS.md](COMMON_ERRORS.md) for error examples.
- Use `cargo test` to run all tests.

---

## 7. Contributing examples

- Add new tutorials to `docs/TUTORIALS.md`.
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.
