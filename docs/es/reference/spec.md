# Especificación del Lenguaje Tupã v0.1

> **Fuerza ancestral, código moderno**
> Lenguaje brasileño para sistemas críticos e IA en evolución

![Specification Status](https://img.shields.io/badge/status-draft-orange)
![License](https://img.shields.io/badge/license-CC--BY--SA%204.0-ff69b4)

## Propósito

Este documento define la especificación formal del lenguaje Tupã, incluyendo gramática, reglas de tipos y semántica.

## Índice

- [1. Filosofía y objetivos de diseño](#1-filosofía-y-objetivos-de-diseño)
- [2. Estructura léxica](#2-estructura-léxica)
- [3. Sistema de tipos](#3-sistema-de-tipos)
- [4. Expresiones](#4-expresiones)
- [5. Instrucciones](#5-instrucciones)
- [6. Semántica numérica (Normativa)](#6-semántica-numérica-normativa)
- [6. Concurrencia](#6-concurrencia)
- [7. Módulos y FFI](#7-módulos-y-ffi)
- [8. Gramática EBNF completa (Normativa)](#8-gramática-ebnf-completa-normativa)
- [9. Semántica y notas de implementación](#9-semántica-y-notas-de-implementación)
- [10. Conversiones de tipos (Normativa)](#10-conversiones-de-tipos-normativa)
- [10. Ejemplos validados](#10-ejemplos-validados)
- [11. Diagnósticos (Normativa)](#11-diagnósticos-normativa)
- [12. Política de versiones](#12-política-de-versiones)
- [13. Referencias e influencias](#13-referencias-e-influencias)

---

## 1. Filosofía y objetivos de diseño

### 1.1 Principios fundamentales

1. **Rendimiento predecible**: cero asignaciones ocultas; el costo de ejecución es visible en el código fuente
2. **Diferenciabilidad nativa**: cualquier expresión pura es automáticamente diferenciable mediante el operador `∇`
3. **Alineación mediante tipos**: restricciones éticas verificadas en tiempo de compilación, no en tiempo de ejecución
4. **Dispersión declarativa**: la densidad de datos es parte del tipo, no una optimización posterior

### 1.2 Público objetivo

- Investigadores de IA que necesitan rendimiento y seguridad formal
- Ingenieros de sistemas críticos (fintech, salud, infraestructura)
- Desarrolladores que valoran la productividad sin sacrificar el control

### 1.3 Objetivos excluidos

- Reemplazar Python para scripts rápidos
- Ser 100% compatible con la sintaxis de Rust/Python
- Soportar programación imperativa no estructurada

### 1.4 Convenciones del documento

- **Normativo**: las secciones con gramática EBNF, reglas de tipos y semántica son obligatorias.
- **Informativo**: los ejemplos, notas y comentarios sirven como orientación.

### 1.5 Alcance del MVP (núcleo)

- Lexer + parser para funciones, `let`, `if`, `match`, llamadas y literales.
- Verificador de tipos para tipos primitivos y tuplas simples.
- Semántica de `∇` limitada a funciones puras.
- Generación de código para expresiones aritméticas básicas.

---

## 2. Estructura léxica

### 2.1 Codificación de caracteres

- UTF-8 obligatorio
- Los identificadores admiten letras Unicode (`\p{L}`) + `_`
- Las palabras clave son solo ASCII (sensibles a mayúsculas)

### 2.2 Comentarios

```tupa
// Comentario de una línea

/* Comentario
   multilínea */
```

### 2.3 Identificadores

```ebnf
identifier = letter { letter | digit | "_" } ;
letter     = "a".."z" | "A".."Z" | "\u{0080}".."\u{10FFFF}" ;
digit      = "0".."9" ;
```

**Normalización Unicode (Normativa)**:

- Los identificadores se comparan tras la normalización NFC.
- El compilador debe rechazar identificadores que cambien tras la normalización (para evitar confusión visual).

**Ejemplos válidos**: `x`, `_temp`, `ação`, `π_value`
**Ejemplos inválidos**: `1var`, `@name`, `fn` (palabra clave)

### 2.4 Palabras clave

```text
fn let if else match while for in return async spawn await
pipeline step
true false null i64 f64 f32 f16 bool string tensor option result
safe unsafe extern import export
```

### 2.5 Literales

```ebnf
integer_literal = digit { digit } ;
float_literal   = digit { digit } "." digit { digit } [ ("e" | "E") ["+" | "-"] digit { digit } ] ;
string_literal  = '"' { unicode_char | escape_sequence } '"' ;
escape_sequence = "\\" ("n" | "t" | '"' | "\\" | "u{" hex_digit {1,6} "}") ;
hex_digit       = digit | "a".."f" | "A".."F" ;
tensor_literal  = "[" expression { "," expression } "]" ;
```

**Ejemplos**:

```tupa
42          // integer_literal
3.14        // float_literal
1.5e-3      // notación científica
"Olá 🌩️"   // cadena con Unicode
"newline\n" // secuencia de escape
[1, 2, 3]   // tensor_literal
```

---

## 3. Sistema de tipos

### 3.1 Tipos primitivos

| Tipo | Descripción | Tamaño | Ejemplo |
| --- | --- | --- | --- |
| `i64` | Entero con signo | 64 bits | `42` |
| `f64` | IEEE 754 doble | 64 bits | `3.14` |
| `f32` | IEEE 754 flotante | 32 bits | `1.0f32` |
| `f16` | Media precisión | 16 bits | `0.5f16` |
| `bool` | Booleano | 1 bit | `true` |
| `string` | UTF-8 inmutable | dinámico | `"Tupã"` |

### 3.2 Tipos compuestos

#### 3.2.1 Tuplas

```ebnf
tuple_type = "(" type { "," type } [","] ")" ;
```

```tupa
let pair: (i64, string) = (42, "answer")
let first = pair.0  // 42
```

#### 3.2.2 Tipos de función (Normativa)

```ebnf
func_type = "fn" "(" [ type { "," type } ] ")" "->" type ;
```

```tupa
let f: fn(i64, i64) -> i64 = add
let g: fn() -> bool = is_ready
// Función anónima (lambda)
let inc: fn(i64) -> i64 = |x| x + 1
// Función como valor
let apply: fn(fn(i64)->i64, i64) -> i64 = |f, x| f(x)
let r = apply(inc, 10) // r = 11
// Función con print y concatenación de cadenas
fn hello(name: string) {
    print("Hola, " + name)
}
hello("Tupã")
```

**Comparación:**

| Tupã | Python | Rust |
| --- | --- | --- |
| `let inc: fn(i64)->i64 = \|x\| x+1` | `inc = lambda x: x+1` | `let inc = \|x: i64\| x+1;` |
| `print("Hola, " + name)` | `print("Hola, " + name)` | `println!("Hola, {}", name);` |

Ver más ejemplos en [Guía de ejemplos](../guides/examples_guide.md) y [examples/README.md](../../examples/README.md).

#### 3.2.3 Tipos enum (genéricos)

```ebnf
enum_decl = "enum" identifier [ "<" identifier { "," identifier } ">" ] "{" enum_variant { "," enum_variant } [ "," ] "}" ;
enum_variant = identifier [ "(" type { "," type } [ "," ] ")" ] ;
enum_type = identifier [ "<" type { "," type } ">" ] ;
```

```tupa
enum Result<T, E> {
    Ok,
    Err
}

fn use_result(r: Result<Safe<f64, !nan>, string>) {
    print("ok")
}
```

#### 3.2.4 Option / Result (manejo de errores)

```ebnf
option_type = "Option" "<" type ">" ;
result_type = "Result" "<" type "," type ">" ;
```

```tupa
fn divide(a: f64, b: f64): Result<f64, string> {
    if b == 0.0 {
        return Err("División por cero")
    }
    return Ok(a / b)
}
```

#### 3.2.5 Tensores (IA de primer nivel)

```ebnf
tensor_type = "Tensor" "<"
                type ","
                "shape" "=" "[" dimension { "," dimension } "]"
                [ "," "density" "=" float_literal ]
              ">" ;
dimension   = integer_literal | "..." ;  // "..." = dimensión dinámica
```

```tupa
// Tensor denso 28x28 (MNIST)
let image: Tensor<f32, shape=[28, 28]> = load("digit.tp")

// Tensor 90% disperso (recomendado para LLMs)
let weights: Tensor<f16, shape=[4096, 4096], density=0.1> = load("llama3.tp")
```

#### 3.2.6 Tipos de alineación (restricciones éticas)

```ebnf
safe_type = "Safe" "<" type "," constraint_list ">" ;
constraint_list = "!" identifier { "," "!" identifier } ;
```

```tupa
// Texto que no puede contener discurso de odio
let summary: Safe<string, !hate_speech> = summarize(article)

// Número que no puede ser NaN/Inf (crítico para entrenamiento estable)
let loss: Safe<f64, !nan, !inf> = compute_loss(predictions, targets)
```

Ejemplo con propagación de enum:

```tupa
enum Reason {
    Misinformation
}

enum LLMResponse<T> {
    Safe(T),
    Flagged(T, Reason),
    Blocked(Reason)
}

fn classify(text: string): LLMResponse<Safe<string, !misinformation>> {
    if is_misinformation(text) {
        return Flagged(text, Misinformation())
    }
    return Safe(text)
}
```

Ejemplo con coincidencia de patrones:

```tupa
fn handle(response: LLMResponse<Safe<string, !misinformation>>) {
    match response {
        Safe(text) => publish(text),
        Flagged(text, reason) => review(text, reason),
        Blocked(reason) => reject(reason),
    }
}
```

> **Nota**: Las restricciones se verifican mediante:
>
> - Pruebas formales (para propiedades matemáticas)
> - Puntuaciones RLHF (para contenido generado por LLM)
> - Guardia de tiempo de ejecución (si el tiempo de compilación no puede probar)

**Semántica**:

- Si el compilador **prueba** la restricción, el tipo `Safe<T, !c>` es válido.
- Si **no puede probar**, es un error en tiempo de compilación (con una sugerencia de corrección).
- Se puede usar `unsafe { ... }` para asumir responsabilidad explícita.

**Implementación actual (compilador)**:

- `!nan` e `!inf` solo se aceptan con base `f64`.
- `!hate_speech` y `!misinformation` solo se aceptan con base `string`.
- La prueba solo se realiza con literales `f64` y expresiones constantes.
- Para restricciones de `string`, el compilador solo acepta valores ya probados `Safe<string, ...>`.
- Si la prueba no es posible, el compilador reporta un error de restricción no probada.

##### 3.2.6.1 Resolución de restricciones (Normativa)

Para cada restricción `!c` en `Safe<T, !c>`:

| Restricción | Requisito del solver | Alternativa |
| --- | --- | --- |
| `!nan` | Análisis de intervalo prueba `x ∈ [-∞, +∞] \ {NaN}` | `@assume(!nan)` con advertencia |
| `!inf` | Límites estáticos prueban `abs(x) < 1.7976931348623157e308` | `@assume(!inf)` con advertencia |
| `!hate_speech` | Puntuación RLHF ≥ 0.95 en el dataset definido | ❌ No permitido |
| `!misinformation` | Puntuación RLHF ≥ 0.95 en el dataset definido | ❌ No permitido |

---

### 3.3 Tipos de arreglo (Normativa)

```ebnf
array_type = "[" type ";" integer_literal "]" ;  // tamaño fijo
slice_type = "[" type "]" ;                      // tamaño dinámico
```

```tupa
let fixed: [i64; 5] = [1, 2, 3, 4, 5]
let dynamic: [i64] = vec![1, 2, 3]
```

**Semántica (Normativa)**:

- `[T; N]` se asigna en la pila cuando es posible.
- `[T]` se asigna en el heap y es mutable solo si se referencia con `mut`.
- Los literales `[a, b, c]` infieren `[T; N]` cuando `N` es conocido.

---

## 4. Expresiones

### 4.0 Precedencia de operadores (mayor → menor)

| Precedencia | Operadores |
| --- | --- |
| 1 | `()` `.` llamada a función |
| 2 | `∇` unario |
| 3 | `!` `-` unario |
| 4 | `**` |
| 5 | `*` `/` |
| 6 | `+` `-` |
| 7 | `<` `<=` `>` `>=` |
| 8 | `==` `!=` |
| 9 | `&&` |
| 10 | `\|\|` |

### 4.1 Reglas de evaluación (Normativa)

- `if` evalúa solo la rama seleccionada.
- `a && b` usa cortocircuito: `b` se evalúa solo si `a` es `true`.
- `a || b` usa cortocircuito: `b` se evalúa solo si `a` es `false`.
- `match` evalúa solo el cuerpo del primer patrón coincidente.

### 4.1 Gramática completa

```ebnf
expression        = assignment
                  | conditional
                  | match_expr
                  | binary_expr
                  | unary_expr
                  | primary_expr ;

assignment        = identifier "=" expression ;

conditional       = "if" expression block [ "else" ( block | conditional ) ] ;

match_expr        = "match" expression "{" { match_arm } "}" ;
match_arm         = pattern ["if" guard] "=>" expression [","] ;
guard             = expression ;
pattern           = "_"
                  | literal
                  | identifier
                  | tuple_pattern
                  | constructor_pattern ;
tuple_pattern     = "(" pattern { "," pattern } [","] ")" ;
constructor_pattern = identifier "(" pattern { "," pattern } [","] ")" ;

binary_expr       = unary_expr { binary_op unary_expr } ;
binary_op         = "||" | "&&" | "==" | "!=" | "<" | "<=" | ">" | ">="
                  | "+" | "-" | "*" | "/" | "**" ;  // ** = exponenciación

unary_expr        = [ unary_op ] primary_expr ;
unary_op          = "!" | "-" | "∇" ;  // ∇ = operador gradiente

primary_expr      = literal
                  | identifier
                  | "(" expression ")"
                  | identifier "(" [ argument_list ] ")"
                  | identifier "." field_access
                  | "await" expression ;

argument_list     = expression { "," expression } ;
field_access      = identifier | integer_literal ;
literal           = integer_literal | float_literal | string_literal | "true" | "false" | "null" ;
```

### 4.2 Expresiones principales

#### 4.2.1 Operador gradiente (`∇`)

```tupa
// Función pura → derivada simbólica generada por el compilador
fn square(x: f64): f64 { x * x }

let grad_at_3 = ∇square(3.0)  // → 6.0 (derivada: 2*x)

// Derivada parcial para múltiples parámetros
fn mse(pred: f64, target: f64): f64 {
    let diff = pred - target
    return diff * diff
}

let (d_pred, d_target) = ∇mse(0.8, 1.0)  // → (-0.4, 0.4)
```

**Tipo de retorno**:

- Para `f: (T1, ..., Tn) -> R`, `∇f(args)` retorna `(dT1, ..., dTn)`.
- Para `n = 1`, el retorno es un escalar `dT1`.
- El valor de `f(args)` se obtiene llamando `f(args)` por separado.

##### Pureza formal (Normativa)

Una función `f` es **pura** si y solo si:

1. No llama a funciones con el atributo `@side_effects(...)`.
2. No accede ni modifica variables mutables no locales (`static mut`, globales).
3. No realiza operaciones de E/S (`print`, `file.read`, `http.get`).
4. No contiene no-determinismo (`rand()`, `time.now()`, `thread_id()`).
5. Todas las funciones llamadas por `f` son puras (recursión de pureza).

> **Regla de pureza**: `∇` solo funciona en expresiones *puras* (sin efectos secundarios). El compilador rechaza:
>
> ```tupa
> fn impure(x: f64): f64 {
>     print(x)  // ¡efecto secundario!
>     return x * 2
> }
> let g = ∇impure(3.0)  // ❌ Error: la función no es pura
> ```

#### 4.2.2 Coincidencia de patrones

```tupa
match http_status {
    200 => "OK",
    404 => "No encontrado",
    code if code >= 500 => f"Error del servidor {code}",
    _ => "Desconocido"
}
```

#### 4.2.3 Interpolación de cadenas

```tupa
let name = "Tupã"
print(f"Hola, {name}!")  // → "Hola, Tupã!"
```

---

## 5. Instrucciones

### 5.1 Gramática

```ebnf
statement         = declaration
                  | expression ";"
                  | block
                  | control_flow ;

declaration       = "let" [ "mut" ] identifier [ ":" type ] "=" expression ";"
                  | function_decl
                  | enum_decl ;

function_decl     = [ attribute_list ] "fn" identifier
                    "(" [ parameter_list ] ")"
                    [ ":" type ]
                    block ;

enum_decl         = "enum" identifier [ "<" identifier { "," identifier } ">" ]
                    "{" enum_variant { "," enum_variant } [ "," ] "}" ;
enum_variant      = identifier [ "(" type { "," type } [ "," ] ")" ] ;

attribute_list    = "@" identifier [ "(" attribute_args ")" ]
                    { "@" identifier [ "(" attribute_args ")" ] } ;
attribute_args    = identifier "=" literal { "," identifier "=" literal } ;

parameter_list    = parameter { "," parameter } ;
parameter         = identifier ":" type ;

block             = "{" { statement } "}" ;

control_flow      = "return" [ expression ] ";"
                  | "while" expression block
                  | "for" identifier "in" range_expr block ;

range_expr        = expression ".." expression ;  // fin exclusivo
```

### 5.2 Ligadura de variables

```tupa
// Inferencia de tipos
let x = 42          // x: i64
let pi = 3.14       // pi: f64

// Tipo explícito (recomendado para APIs públicas)
let name: string = "Tupã"

// Mutabilidad explícita (por defecto es inmutable)
let mut counter = 0
counter = counter + 1  // permitido
```

### 5.3 Funciones

```tupa
// Función pura (por defecto) → automáticamente diferenciable
fn relu(x: f64): f64 {
    if x > 0.0 { x } else { 0.0 }
}

// Función con efectos secundarios explícitos
@side_effects(io)
fn log(message: string) {
    print(f"[LOG] {message}")
}

// Función asíncrona
async fn fetch_user(id: i64): Result<User, string> {
    let resp = await http.get(f"/api/users/{id}")
    return parse_user(resp)
}
```

### 5.4 Flujo de control

```tupa
// if como expresión
let status = if temp > 100 { "crítico" } else { "normal" }

// bucle while
let mut i = 0
while i < 10 {
    print(i)
    i = i + 1
}

// bucle for con rango
for i in 0..10 {
    print(i)  // 0, 1, 2, ..., 9 (fin exclusivo)
}
```

### 5.5 Ámbito y sombreado (Normativa)

- La resolución de nombres es léxica, del bloque más interno al más externo.
- El sombreado está permitido (estilo Rust).
- Redeclarar el mismo nombre en el mismo ámbito es un error.

Ejemplo:

```tupa
let x = 10
fn foo() {
    let x = 20
    print(x)  // 20
}
```

---

## 6. Semántica numérica (Normativa)

### 6.1 Desbordamiento de enteros

- El desbordamiento en `i64` lanza un error en tiempo de ejecución (panic).
- Se deben usar `wrap_add`, `wrap_sub`, `wrap_mul` para desbordamiento intencional.

Ejemplo:

```tupa
let x: i64 = 9223372036854775807
let y = x.wrap_add(1)
```

---

## 6. Concurrencia

### 6.1 Lanzamiento de tareas

```ebnf
spawn_stmt = "spawn" expression ";" ;
```

```tupa
spawn async fn worker(id: i64) {
    let data = await db.query(id)
    process(data)
}

// Spawn anónimo
spawn async {
    let result = await heavy_computation()
    send_to_main(result)
}
```

### 6.2 Canales

```tupa
// Creación de canal tipado
let (tx, rx): (Channel<i64>, Channel<i64>) = channel()

// Enviar
await tx.send(42)

// Recibir (bloqueante)
let value = await rx.recv()  // value: i64

// Recibir con tiempo de espera
match await rx.recv_timeout(1000) {  // 1000ms
    Some(v) => print(f"Recibido: {v}"),
    None => print("¡Tiempo de espera agotado!")
}
```

> **Garantía**: Los canales son *basados en propiedad*, haciendo imposibles las condiciones de carrera a través del sistema de tipos.

---

## 7. Módulos y FFI

### 7.1 Módulos

```tupa
// math.tp
export fn square(x: f64): f64 { x * x }

// main.tp
import "math" as math

let result = math.square(5.0)
```

### 7.2 Interfaz de función foránea (C)

```tupa
extern "C" {
    fn malloc(size: i64): *void
    fn free(ptr: *void)
}

fn main() {
    let ptr = unsafe { malloc(1024) }
    // ... uso ...
    unsafe { free(ptr) }
}
```

**ABI mínimo (Normativa)**:

- Tipos requeridos: `usize`, `*const T`, `*mut T`.
- Enteros C: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`.
- Punteros opacos: `*void`.
- `usize` tiene el mismo tamaño que el puntero de datos de la plataforma.
- Los punteros no pueden desreferenciarse fuera de `unsafe`.

> **Regla**: `unsafe` requiere un bloque explícito, lo que facilita la auditoría.

---

## 8. Gramática EBNF completa (Normativa)

```ebnf
(* ===== LÉXICO ===== *)
letter          = "a".."z" | "A".."Z" | "\u{0080}".."\u{10FFFF}" ;
digit           = "0".."9" ;
hex_digit       = digit | "a".."f" | "A".."F" ;
identifier      = letter { letter | digit | "_" } ;
integer_literal = digit { digit } ;
float_literal   = digit { digit } "." digit { digit }
                  [ ("e" | "E") ["+" | "-"] digit { digit } ] ;
string_literal  = '"' { ( "\u{0000}".."\u{0021}" | "\u{0023}".."\u{005B}" | "\u{005D}".."\u{10FFFF}" )
                      | escape_sequence } '"' ;
escape_sequence = "\\" ( "n" | "t" | '"' | "\\" | "u{" hex_digit {1,6} "}" ) ;

(* ===== TIPOS ===== *)
type            = primitive_type
                | tuple_type
                | enum_type
                | option_type
                | result_type
                | tensor_type
                | safe_type
                | identifier ;

primitive_type  = "i64" | "f64" | "f32" | "f16" | "bool" | "string" ;
tuple_type      = "(" type { "," type } [","] ")" ;
enum_type       = identifier [ "<" type { "," type } ">" ] ;
option_type     = "Option" "<" type ">" ;
result_type     = "Result" "<" type "," type ">" ;
tensor_type     = "Tensor" "<"
                    type ","
                    "shape" "=" "[" dimension { "," dimension } "]"
                    [ "," "density" "=" float_literal ]
                  ">" ;
dimension       = integer_literal | "..." ;
safe_type       = "Safe" "<" type "," constraint_list ">" ;
constraint_list = "!" identifier { "," "!" identifier } ;

(* ===== EXPRESIONES ===== *)
expression      = assignment
                | conditional
                | match_expr
                | binary_expr
                | unary_expr
                | primary_expr ;

assignment      = identifier "=" expression ;

conditional     = "if" expression block [ "else" ( block | conditional ) ] ;

match_expr      = "match" expression "{" { match_arm } "}" ;
match_arm       = pattern [ "if" expression ] "=>" expression [ "," ] ;
pattern         = "_"
                | literal
                | identifier
                | tuple_pattern
                | constructor_pattern ;
tuple_pattern   = "(" pattern { "," pattern } [ "," ] ")" ;
constructor_pattern = identifier "(" pattern { "," pattern } [ "," ] ")" ;

binary_expr     = unary_expr { binary_op unary_expr } ;
binary_op       = "||" | "&&" | "==" | "!=" | "<" | "<=" | ">" | ">="
                | "+" | "-" | "*" | "/" | "**" ;

unary_expr      = [ unary_op ] primary_expr ;
unary_op        = "!" | "-" | "∇" ;

primary_expr    = literal
                | identifier
                | "(" expression ")"
                | identifier "(" [ argument_list ] ")"
                | identifier "." ( identifier | integer_literal )
                | "await" expression ;

literal         = integer_literal | float_literal | string_literal | "true" | "false" | "null" ;
tensor_literal  = "[" expression { "," expression } "]" ;
argument_list   = expression { "," expression } ;

(* ===== INSTRUCCIONES ===== *)
statement       = declaration
                | expression ";"
                | block
                | control_flow ;

declaration     = "let" [ "mut" ] identifier [ ":" type ] "=" expression ";"
                | function_decl
                | enum_decl
                | pipeline_decl ;

function_decl   = [ attribute_list ] "fn" identifier
                  "(" [ parameter_list ] ")"
                  [ ":" type ]
                  block ;

enum_decl       = "enum" identifier [ "<" identifier { "," identifier } ">" ]
                  "{" enum_variant { "," enum_variant } [ "," ] "}" ;
enum_variant    = identifier [ "(" type { "," type } [ "," ] ")" ] ;

attribute_list  = "@" identifier [ "(" attribute_args ")" ]
                  { "@" identifier [ "(" attribute_args ")" ] } ;
attribute_args  = identifier "=" literal { "," identifier "=" literal } ;

parameter_list  = parameter { "," parameter } ;
parameter       = identifier ":" type ;

block           = "{" { statement } "}" ;

control_flow    = "return" [ expression ] ";"
                | "while" expression block
                | "for" identifier "in" range_expr block ;

range_expr      = expression ".." expression ;

(* ===== NIVEL SUPERIOR ===== *)
program         = { import_decl | export_decl | declaration } ;
import_decl     = "import" string_literal [ "as" identifier ] ";" ;
export_decl     = "export" ( function_decl | "let" identifier ) ;

(* ===== PIPELINES ===== *)
pipeline_decl   = "pipeline" identifier [ "@" attribute_list ] "{" pipeline_body "}" ;
pipeline_body   = "input" ":" type ","
                [ "constraints" ":" "[" identifier { "," identifier } "]" "," ]
                "steps" ":" "[" step_list "]" [ "," ]
                [ "validation" ":" block ] ;
step_list       = step_decl { "," step_decl } ;
step_decl       = "step" "(" string_literal ")" "{" expression "}" ;
```

---

## 9. Semántica y notas de implementación

### 9.1 Pipeline del compilador

```text
Fuente (.tp)
  ↓ [Lexer: nom]
Tokens
  ↓ [Parser: descenso recursivo]
AST
  ↓ [Verificador de tipos: Hindley-Milner + solver de restricciones]
AST tipado
  ↓ [Codegen: inkwell → LLVM IR]
LLVM IR
  ↓ [Optimizador LLVM (-O3)]
Binario nativo (ELF/Mach-O/PE)
```

### 9.2 Estrategia de compilación del gradiente

Para `∇f(x)` donde `f` es pura:

1. El parser marca la función como `#[pure]` (implícito mediante análisis de efectos)
2. El verificador de tipos verifica la ausencia de efectos secundarios
3. Codegen emite **dos caminos** en LLVM IR:
   - Paso hacia adelante: código original
   - Paso hacia atrás: derivadas simbólicas mediante diferenciación automática
4. El runtime selecciona el camino según el uso de `∇`

**Ejemplo de LLVM IR generado** para `fn square(x: f64): f64 { x * x }`:

```llvm
; Paso hacia adelante
define double @square(double %x) {
  %mul = fmul double %x, %x
  ret double %mul
}

; Paso hacia atrás (generado automáticamente)
define { double, double } @square_grad(double %x) {
  %mul = fmul double %x, %x        ; hacia adelante
  %grad = fmul double 2.0, %x      ; derivada: 2*x
  %ret = insertvalue { double, double } undef, double %mul, 0
  %ret2 = insertvalue { double, double } %ret, double %grad, 1
  ret { double, double } %ret2
}
```

### 9.3 Verificación de tipos de alineación

Para `Safe<T, !constraint>`:

- El compilador consulta el **solver de restricciones** (plugin):
  - Para `!nan`: análisis de intervalo estático + propagación de restricciones
  - Para `!hate_speech`: integración offline con un scorer RLHF
- Si el solver no puede probar la seguridad → error en tiempo de compilación con sugerencia
- Alternativa explícita: `unsafe { ... }` con auditoría obligatoria

### 9.4 Modelo de memoria

- **Asignación en pila** preferida para valores pequeños (< 4KB)
- **Asignación en arena** para ASTs y estructuras temporales (sin sobrecarga GC)
- **GC de rastreo opcional** solo para ciclos de referencia (habilitado con `@gc`)
- **Sin asignaciones ocultas**: todas las asignaciones requieren una llamada explícita a `alloc()`

### 9.5 Diagnósticos (Normativa)

- Los errores deben incluir: código, mensaje, ubicación y sugerencia.
- Formato mínimo: `E####: mensaje (archivo:línea:columna)`.
- Ejemplo: `E3002: no se puede probar la restricción '!nan' en tiempo de compilación (main.tp:12:5)`.

**Códigos recomendados**:

- `E1001`: error léxico
- `E2001`: error de tipo
- `E3001`: restricción inválida
- `E3002`: restricción no probada
- `E4001`: uso inválido de `unsafe`

---

## 10. Conversiones de tipos (Normativa)

- Las conversiones implícitas están prohibidas entre tipos numéricos.
- Las conversiones explícitas usan `as` (por ejemplo, `i64 as f64`).
- Convertir `bool` a numérico está prohibido.

---

## 10. Ejemplos validados

### 10.1 Hola Mundo

```tupa
fn main() {
    print("🌩️ ¡Hola, Tupã!")
}
```

### 10.2 Inferencia MNIST (Tensor disperso)

```tupa
fn softmax(x: Tensor<f32, shape=[10]>): Tensor<f32, shape=[10]> {
    let max = x.max()
    let exps = x.map(|v| (v - max).exp())
    return exps / exps.sum()
}

fn predict(image: Tensor<f32, shape=[28, 28]>): i64 {
    let weights: Tensor<f16, shape=[784, 10], density=0.15> = load("weights.tp")
    let flattened = image.flatten()
    let logits = matmul(flattened, weights)
    let probs = softmax(logits)
    return probs.argmax()
}
```

### 10.3 Resumen con garantía de alineación

```tupa
fn summarize(article: string): Safe<string, !misinformation, !hate_speech> {
    // El compilador requiere prueba de seguridad mediante:
    // 1. Puntuación RLHF > 0.95 en el dataset de validación
    // 2. Verificación formal de no generar contenido prohibido
    return llm.generate(f"Resume objetivamente: {article}")
}

fn main() {
    let article = load_article("news.tp")
    let summary = summarize(article)  // ✅ Compila solo si se prueba la seguridad
    publish(summary)  // Nunca publica contenido peligroso
}
```

### 10.4 Detección de fraude diferenciable (neurosimbólico)

```tupa
@differentiable
fn risk_score(tx: Transaction): f64 {
    let neural = fraud_net.infer(tx.features)  // Tensor<f16, density=0.1>
    let symbolic = if tx.country == "BR" && tx.amount > 1000.0 {
        0.8
    } else {
        0.2
    }
    return 0.7 * neural + 0.3 * symbolic
}

// Entrenamiento mediante descenso de gradiente
fn train_step(batch: [Transaction], targets: [f64], lr: f64) {
    let (loss, grad) = ∇compute_loss(batch, targets)
    update_weights(grad, lr)
}
```

---

## 11. Diagnósticos (Normativa)

### 11.1 Formato de error

El compilador **debe** reportar errores con:

- Código de error (`E####`)
- Mensaje corto
- Span con línea/columna (basado en 1)
- Fragmento de código con resaltado

Ejemplo:

```text
error[E0003]: se esperaba ';' después de la expresión
  --> examples/hello.tp:3:18
   |
 3 |  let age = 28
   |                 ^
```

### 11.2 Formato de advertencia

Las advertencias siguen el mismo formato, con el prefijo `warning[W####]`.

**Nota (informativa)**: Las herramientas pueden ofrecer salida JSON equivalente con `code`, `message`, `label`, `span`, `line` y `col` para integración con editores y automatización.

### 11.3 Semántica del span

- El span **debe** apuntar al token que causa el error cuando sea posible.
- Para errores EOF, el span **debe** apuntar al final del archivo.

### 11.4 Diagnósticos de tipos (Normativa)

El compilador **debe** emitir errores de tipo con código y, cuando sea posible, con span:

```text
error[E2001]: incompatibilidad de tipos: se esperaba I64, se obtuvo Bool
  --> examples/invalid_type.tp:2:15
   |
 2 |  let x: i64 = true;
   |               ^^^^
```

Para aridad incorrecta:

```text
error[E2002]: incompatibilidad de aridad: se esperaba 2, se obtuvo 1
  --> examples/invalid_call.tp:6:10
   |
 6 |  let y = add(1);
   |          ^^^^^^
```

---

## 12. Política de versiones

- **Mayor** (v1 → v2): Cambios incompatibles en gramática o sistema de tipos
- **Menor** (v0.1 → v0.2): Características compatibles con versiones anteriores
- **Parche** (v0.1.0 → v0.1.1): Correcciones de errores sin cambios en la spec

> **Compromiso**: API estable a partir de v1.0.

---

## 13. Referencias e influencias

| Lenguaje/Proyecto | Influencia en Tupã |
| --- | --- |
| **Rust** | Modelo de propiedad, coincidencia de patrones, seguridad sin GC |
| **Zig** | Cero asignaciones ocultas, simplicidad radical |
| **Mojo** | Diferenciabilidad nativa, rendimiento de Python |
| **Swift** | Tipado gradual, interoperabilidad con C |
| **Lean** | Verificación formal integrada en el lenguaje |
| **JAX** | Transformaciones funcionales (`grad`, `jit`) como primitivas |

---

*Especificación mantenida por la comunidad Tupã • Licencia: CC-BY-SA 4.0*
*Versión: 0.1-draft*
