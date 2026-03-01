# Tutoriales paso a paso

## Propósito

Guiar a usuarios de distintos niveles en tareas comunes y proyectos de ejemplo con Tupã.

---

## 1. Hola, Mundo

```tupa
print("¡Hola, Tupã!")
```

Ejecuta:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
```

---

## 2. Funciones y lambdas

```tupa
let inc: fn(int) -> int = |x| x + 1
print(inc(41)) // salida: 42
```

---

## 3. Trabajando con strings

```tupa
let name = "Tupã"
print("Bienvenido, " + name)
```

---

## 4. Funciones con tipos Safe

```tupa
fn safe(x: f64): Safe<f64, !nan> {
  return x
}

fn safe_text(x: Safe<string, !misinformation>) -> Safe<string, !misinformation> {
  return x
}
```

---

## 5. Proyecto de ejemplo: Suma de vectores

Archivo: `examples/soma_vetor.tp`

```tupa
fn sum(v: [int]) -> int {
  let mut total = 0
  for x in v {
    total = total + x
  }
  return total
}
print(sum([1,2,3,4])) // salida: 10
```

---

## 6. Depuración y diagnósticos

- Consulta [docs/common_errors.md](../reference/common_errors.md) para ejemplos de errores.
- Usa `cargo test` para ejecutar todas las pruebas.

---

## 7. Contribuyendo ejemplos

- Agrega nuevos tutoriales en `docs/en/guides/tutorials.md`.
- Consulta [CONTRIBUTING.md](../../CONTRIBUTING.md) para las guías.
