# Tupã Language Specification v0.1

> **Ancestral strength, modern code**  
> Brazilian language for critical systems and evolving AI

[![Specification Status](https://img.shields.io/badge/status-draft-orange)](#)
[![License](https://img.shields.io/badge/license-CC--BY--SA%204.0-ff69b4)](#)

## Purpose

Define the formal specification of the Tupã language, including grammar, type rules, and semantics.

## Index

- [1. Philosophy & Design Goals](#1-philosophy--design-goals)
- [2. Lexical Structure](#2-lexical-structure)
- [3. Type System](#3-type-system)
- [4. Expressions](#4-expressions)
- [5. Statements](#5-statements)
- [6. Numeric Semantics (Normative)](#6-numeric-semantics-normative)
- [6. Concurrency](#6-concurrency)
- [7. Modules & FFI](#7-modules--ffi)
- [8. Complete EBNF Grammar (Normative)](#8-complete-ebnf-grammar-normative)
- [9. Semantics & Implementation Notes](#9-semantics--implementation-notes)
- [10. Type Conversions (Normative)](#10-type-conversions-normative)
- [10. Validated Examples](#10-validated-examples)
- [11. Diagnostics (Normative)](#11-diagnostics-normative)
- [12. Versioning Policy](#12-versioning-policy)
- [13. References & Influences](#13-references--influences)

---

## 1. Philosophy & Design Goals

### 1.1 Core Principles
1. **Predictable performance**: Zero hidden allocations; execution cost visible in source code
2. **Native differentiability**: Any pure expression is automatically differentiable via the `∇` operator
3. **Alignment via types**: Ethical constraints checked at compile time, not runtime
4. **Declarative sparsity**: Data density is part of the type, not a post-processing optimization

### 1.2 Target Audience
- AI researchers who need performance and formal safety
- Critical systems engineers (fintech, healthcare, infrastructure)
- Developers who value productivity without sacrificing control

### 1.3 Non-Goals
- Replace Python for quick scripts
- Be 100% compatible with Rust/Python syntax
- Support unstructured imperative programming

### 1.4 Document Conventions
- **Normative**: sections with EBNF grammar, type rules, and semantics are mandatory.
- **Informative**: examples, notes, and comments serve as guidance.

### 1.5 MVP Scope (Core)
- Lexer + parser for functions, `let`, `if`, `match`, calls, and literals.
- Type checker for primitive types and simple tuples.
- `∇` semantics limited to pure functions.
- Code generation for basic arithmetic expressions.

---

## 2. Lexical Structure

### 2.1 Character Encoding
- UTF-8 required
- Identifiers support Unicode letters (`\p{L}`) + `_`
- Keywords are ASCII-only (case-sensitive)

### 2.2 Comments
```tupa
// Single-line comment

/* Multi-line
   comment */
```

### 2.3 Identifiers
```ebnf
identifier = letter { letter | digit | "_" } ;
letter     = "a".."z" | "A".."Z" | "\u{0080}".."\u{10FFFF}" ;
digit      = "0".."9" ;
```

**Unicode Normalization (Normative)**:
- Identifiers are compared after NFC normalization.
- The compiler must reject identifiers that change after normalization (to avoid visual confusion).

**Valid examples**: `x`, `_temp`, `ação`, `π_value`  
**Invalid examples**: `1var`, `@name`, `fn` (keyword)

### 2.4 Keywords
```
fn let if else match while for in return async spawn await
pipeline step
true false null i64 f64 f32 f16 bool string tensor option result
safe unsafe extern import export
```

### 2.5 Literals
```ebnf
integer_literal = digit { digit } ;
float_literal   = digit { digit } "." digit { digit } [ ("e" | "E") ["+" | "-"] digit { digit } ] ;
string_literal  = '"' { unicode_char | escape_sequence } '"' ;
escape_sequence = "\\" ("n" | "t" | '"' | "\\" | "u{" hex_digit {1,6} "}") ;
hex_digit       = digit | "a".."f" | "A".."F" ;
tensor_literal  = "[" expression { "," expression } "]" ;
```

**Examples**:
```tupa
42          // integer_literal
3.14        // float_literal
1.5e-3      // scientific notation
"Olá 🌩️"   // string with Unicode
"newline\n" // escape sequence
[1, 2, 3]   // tensor_literal
```

---

## 3. Type System

### 3.1 Primitive Types
| Type | Description | Size | Example |
|------|-------------|------|---------|
| `i64` | Signed integer | 64-bit | `42` |
| `f64` | IEEE 754 double | 64-bit | `3.14` |
| `f32` | IEEE 754 float | 32-bit | `1.0f32` |
| `f16` | Half-precision | 16-bit | `0.5f16` |
| `bool` | Boolean | 1-bit | `true` |
| `string` | UTF-8 immutable | dynamic | `"Tupã"` |

### 3.2 Composite Types

#### 3.2.1 Tuples
```ebnf
tuple_type = "(" type { "," type } [","] ")" ;
```
```tupa
let pair: (i64, string) = (42, "answer")
let first = pair.0  // 42
```

#### 3.2.2 Function Types (Normative)
```ebnf
func_type = "fn" "(" [ type { "," type } ] ")" "->" type ;
```
```tupa
let f: fn(i64, i64) -> i64 = add
let g: fn() -> bool = is_ready
// Anonymous function (lambda)
let inc: fn(i64) -> i64 = |x| x + 1
// Function as a value
let apply: fn(fn(i64)->i64, i64) -> i64 = |f, x| f(x)
let r = apply(inc, 10) // r = 11
// Function with print and string concatenation
fn hello(name: string) {
	print("Hello, " + name)
}
hello("Tupã")
```

**Comparison:**

| Tupã | Python | Rust |
|------|--------|------|
| `let inc: fn(i64)->i64 = |x| x+1` | `inc = lambda x: x+1` | `let inc = |x: i64| x+1;` |
| `print("Hello, " + name)` | `print("Hello, " + name)` | `println!("Hello, {}", name);` |

See more examples in [FAQ](FAQ.md) and [examples/README.md](../examples/README.md).
```

#### 3.2.3 Enum Types (generics)
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

#### 3.2.4 Option / Result (error handling)
```ebnf
option_type = "Option" "<" type ">" ;
result_type = "Result" "<" type "," type ">" ;
```
```tupa
fn divide(a: f64, b: f64): Result<f64, string> {
	if b == 0.0 {
		return Err("Division by zero")
	}
	return Ok(a / b)
}
```

#### 3.2.5 Tensors (AI first-class)
```ebnf
tensor_type = "Tensor" "<" 
				type "," 
				"shape" "=" "[" dimension { "," dimension } "]" 
				[ "," "density" "=" float_literal ] 
			  ">" ;
dimension   = integer_literal | "..." ;  // "..." = dynamic dimension
```
```tupa
// Dense 28x28 tensor (MNIST)
let image: Tensor<f32, shape=[28, 28]> = load("digit.tp")

// 90% sparse tensor (recommended for LLMs)
let weights: Tensor<f16, shape=[4096, 4096], density=0.1> = load("llama3.tp")
```

#### 3.2.6 Alignment Types (ethical constraints)
```ebnf
safe_type = "Safe" "<" type "," constraint_list ">" ;
constraint_list = "!" identifier { "," "!" identifier } ;
```
```tupa
// Text that cannot contain hate speech
let summary: Safe<string, !hate_speech> = summarize(article)

// Number that cannot be NaN/Inf (critical for stable training)
let loss: Safe<f64, !nan, !inf> = compute_loss(predictions, targets)
```

Example with enum propagation:

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

Example with pattern matching:

```tupa
fn handle(response: LLMResponse<Safe<string, !misinformation>>) {
	match response {
		Safe(text) => publish(text),
		Flagged(text, reason) => review(text, reason),
		Blocked(reason) => reject(reason),
	}
}
```

> **Note**: Constraints are verified via:
> - Formal proofs (for mathematical properties)
> - RLHF scores (for LLM-generated content)
> - Runtime guard fallback (if compile time cannot prove)

**Semantics**:
- If the compiler **proves** the constraint, the `Safe<T, !c>` type is valid.
- If it **cannot prove**, it is a compile-time error (with a correction hint).
- `unsafe { ... }` can be used to assume explicit responsibility.

**Current implementation (compiler)**:
- `!nan` and `!inf` are accepted only for `f64` base.
- `!hate_speech` and `!misinformation` are accepted only for `string` base.
- Proof is done only with `f64` literals and constant expressions (for example, `1.0`, `-1.0`, `1.0 + 2.0`, `1.0 / 0.0`).
- For `string` constraints, the compiler only accepts values already proven `Safe<string, ...>` (propagation from variables/returns).
- If proof is not possible, the compiler reports an unproven constraint error.

##### 3.2.6.1 Constraint Resolution (Normative)

For each constraint `!c` in `Safe<T, !c>`:

| Constraint | Solver Requirement | Fallback |
|------------|--------------------|----------|
| `!nan` | Interval analysis proves `x ∈ [-∞, +∞] \ {NaN}` | `@assume(!nan)` with warning |
| `!inf` | Static bounds prove `abs(x) < 1.7976931348623157e308` | `@assume(!inf)` with warning |
| `!hate_speech` | RLHF scorer ≥ 0.95 on the defined dataset | ❌ Not allowed |
| `!misinformation` | RLHF scorer ≥ 0.95 on the defined dataset | ❌ Not allowed |

---

### 3.3 Array Types (Normative)

```ebnf
array_type = "[" type ";" integer_literal "]" ;  // fixed size
slice_type = "[" type "]" ;                      // dynamic size
```

```tupa
let fixed: [i64; 5] = [1, 2, 3, 4, 5]
let dynamic: [i64] = vec![1, 2, 3]
```

**Semantics (Normative)**:
- `[T; N]` is allocated on the stack when possible.
- `[T]` is allocated on the heap and is mutable only if referenced by `mut`.
- Literals `[a, b, c]` infer `[T; N]` when `N` is known.

---

## 4. Expressions

### 4.0 Operator Precedence (highest → lowest)
| Precedence | Operators |
|------------|------------|
| 1 | `()` `.` function call |
| 2 | `∇` unary | 
| 3 | `!` `-` unary |
| 4 | `**` |
| 5 | `*` `/` |
| 6 | `+` `-` |
| 7 | `<` `<=` `>` `>=` |
| 8 | `==` `!=` |
| 9 | `&&` |
| 10 | `||` |

### 4.1 Evaluation Rules (Normative)
- `if` evaluates only the selected branch.
- `a && b` uses short-circuit: `b` is evaluated only if `a` is `true`.
- `a || b` uses short-circuit: `b` is evaluated only if `a` is `false`.
- `match` evaluates only the body of the first matching pattern.

### 4.1 Full Grammar
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
				  | "+" | "-" | "*" | "/" | "**" ;  // ** = exponentiation

unary_expr        = [ unary_op ] primary_expr ;
unary_op          = "!" | "-" | "∇" ;  // ∇ = gradient operator

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

### 4.2 Key Expressions

#### 4.2.1 Gradient Operator (`∇`)
```tupa
// Pure function → symbolic derivative generated by the compiler
fn square(x: f64): f64 { x * x }

let grad_at_3 = ∇square(3.0)  // → 6.0 (derivative: 2*x)

// Partial derivative for multiple parameters
fn mse(pred: f64, target: f64): f64 {
	let diff = pred - target
	return diff * diff
}

let (d_pred, d_target) = ∇mse(0.8, 1.0)  // → (-0.4, 0.4)
```

**Return type**:
- For `f: (T1, ..., Tn) -> R`, `∇f(args)` returns `(dT1, ..., dTn)`.
- For `n = 1`, the return is a scalar `dT1`.
- The value of `f(args)` can be obtained by calling `f(args)` separately.

##### Formal Purity (Normative)

A function `f` is **pure** iff:

1. It does not call functions with the `@side_effects(...)` attribute.
2. It does not access or modify non-local mutable variables (`static mut`, globals).
3. It does not perform I/O operations (`print`, `file.read`, `http.get`).
4. It does not contain non-determinism (`rand()`, `time.now()`, `thread_id()`).
5. All functions called by `f` are pure (purity recursion).

> **Purity rule**: `∇` only works on *pure* expressions (no side effects). The compiler rejects:
> ```tupa
> fn impure(x: f64): f64 {
>     print(x)  // side effect!
>     return x * 2
> }
> let g = ∇impure(3.0)  // ❌ Error: function is not pure
> ```

#### 4.2.2 Pattern Matching
```tupa
match http_status {
	200 => "OK",
	404 => "Not Found",
	code if code >= 500 => f"Server Error {code}",
	_ => "Unknown"
}
```

#### 4.2.3 String Interpolation
```tupa
let name = "Tupã"
print(f"Hello, {name}!")  // → "Hello, Tupã!"
```

---

## 5. Statements

### 5.1 Grammar
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

range_expr        = expression ".." expression ;  // exclusive end
```

### 5.2 Variable Binding
```tupa
// Type inference
let x = 42          // x: i64
let pi = 3.14       // pi: f64

// Explicit type (recommended for public APIs)
let name: string = "Tupã"

// Explicit mutability (default is immutable)
let mut counter = 0
counter = counter + 1  // allowed
```

### 5.3 Functions
```tupa
// Pure function (default) → automatically differentiable
fn relu(x: f64): f64 {
	if x > 0.0 { x } else { 0.0 }
}

// Function with explicit side effects
@side_effects(io)
fn log(message: string) {
	print(f"[LOG] {message}")
}

// Async function
async fn fetch_user(id: i64): Result<User, string> {
	let resp = await http.get(f"/api/users/{id}")
	return parse_user(resp)
}
```

### 5.4 Control Flow
```tupa
// if as expression
let status = if temp > 100 { "critical" } else { "normal" }

// while loop
let mut i = 0
while i < 10 {
	print(i)
	i = i + 1
}

// for loop with range
for i in 0..10 {
print(i)  // 0, 1, 2, ..., 9 (exclusive end)
}
```

### 5.5 Scope and Shadowing (Normative)

- Name resolution is lexical, from the innermost block to the outermost.
- Shadowing is allowed (Rust style).
- Redeclaring the same name in the same scope is an error.

Example:
```tupa
let x = 10
fn foo() {
	let x = 20
	print(x)  // 20
}
```

---

## 6. Numeric Semantics (Normative)

### 6.1 Integer Overflow
- Overflow in `i64` raises a runtime error (panic).
- `wrap_add`, `wrap_sub`, `wrap_mul` must be used for intentional overflow.

Example:
```tupa
let x: i64 = 9223372036854775807
let y = x.wrap_add(1)
```

---

## 6. Concurrency

### 6.1 Spawning Tasks
```ebnf
spawn_stmt = "spawn" expression ";" ;
```
```tupa
spawn async fn worker(id: i64) {
	let data = await db.query(id)
	process(data)
}

// Anonymous spawn
spawn async {
	let result = await heavy_computation()
	send_to_main(result)
}
```

### 6.2 Channels
```tupa
// Typed channel creation
let (tx, rx): (Channel<i64>, Channel<i64>) = channel()

// Send
await tx.send(42)

// Receive (blocking)
let value = await rx.recv()  // value: i64

// Receive with timeout
match await rx.recv_timeout(1000) {  // 1000ms
	Some(v) => print(f"Received: {v}"),
	None => print("Timeout!")
}
```

> **Guarantee**: Channels are *ownership-based*, making data races impossible through the type system.

---

## 7. Modules & FFI

### 7.1 Modules
```tupa
// math.tp
export fn square(x: f64): f64 { x * x }

// main.tp
import "math" as math

let result = math.square(5.0)
```

### 7.2 Foreign Function Interface (C)
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

**Minimal ABI (Normative)**:
- Required types: `usize`, `*const T`, `*mut T`.
- C integers: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`.
- Opaque pointers: `*void`.
- `usize` has the same size as the platform data pointer.
- Pointers cannot be dereferenced outside `unsafe`.

> **Rule**: `unsafe` requires an explicit block, which helps auditing.

---

## 8. Complete EBNF Grammar (Normative)

```ebnf
(* ===== LEXICAL ===== *)
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

(* ===== TYPES ===== *)
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

(* ===== EXPRESSIONS ===== *)
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

(* ===== STATEMENTS ===== *)
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

(* ===== TOP LEVEL ===== *)
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

## 9. Semantics & Implementation Notes

### 9.1 Compiler Pipeline
```
Source (.tp) 
  ↓ [Lexer: nom]
Tokens 
  ↓ [Parser: recursive descent]
AST 
  ↓ [Type Checker: Hindley-Milner + constraint solver]
Typed AST 
  ↓ [Codegen: inkwell → LLVM IR]
LLVM IR 
  ↓ [LLVM Optimizer (-O3)]
Native Binary (ELF/Mach-O/PE)
```

### 9.2 Gradient Compilation Strategy
For `∇f(x)` where `f` is pure:
1. The parser marks the function as `#[pure]` (implicit via effect analysis)
2. The type checker verifies absence of side effects (I/O, global mutation, non-determinism)
3. Codegen emits **two paths** in LLVM IR:
   - Forward pass: original code
   - Backward pass: symbolic derivatives via automatic differentiation rules (chain rule, product rule)
4. Runtime selects the path based on `∇` usage

Note: For small pure functions (<100 ops), use symbolic differentiation without a tape. For larger functions, allow fallback to tape-based differentiation with an arena allocator.

**Generated LLVM IR example** for `fn square(x: f64): f64 { x * x }`:
```llvm
; Forward pass
define double @square(double %x) {
  %mul = fmul double %x, %x
  ret double %mul
}

; Backward pass (generated automatically)
define { double, double } @square_grad(double %x) {
  %mul = fmul double %x, %x        ; forward
  %grad = fmul double 2.0, %x      ; derivative: 2*x
  %ret = insertvalue { double, double } undef, double %mul, 0
  %ret2 = insertvalue { double, double } %ret, double %grad, 1
  ret { double, double } %ret2
}
```

### 9.3 Alignment Type Verification
For `Safe<T, !constraint>`:
- The compiler consults the **constraint solver** (plugin):
  - For `!nan`: static interval analysis + constraint propagation
  - For `!hate_speech`: offline integration with an RLHF scorer (configurable threshold)
- If the solver cannot prove safety → compile-time error with a correction hint
- Explicit fallback: `unsafe { ... }` with mandatory auditing via `@audit(required=true)`

### 9.4 Memory Model
- **Stack allocation** preferred for small values (< 4KB)
- **Arena allocation** for ASTs and temporary structures (zero GC overhead)
- **Optional tracing GC** only for reference cycles (enabled via `@gc` attribute)
- **No hidden allocations**: all allocations require an explicit `alloc()` call

### 9.5 Diagnostics (Normative)
- Errors must include: code, message, location, and hint.
- Minimum format: `E####: message (file:line:column)`.
- Example: `E3002: cannot prove constraint '!nan' at compile time (main.tp:12:5)`.

**Recommended codes**:
- `E1001`: lexical error
- `E2001`: type error
- `E3001`: invalid constraint
- `E3002`: unproven constraint
- `E4001`: invalid `unsafe` usage

Example:
`E2001: incompatible types in assignment (main.tp:8:12)`

Visual example:
```
error[E2001]: incompatible types
	--> main.tp:8:12
	 |
8 | let x: i64 = "text"
	 |            ^^^^^^^^
```

---

## 10. Type Conversions (Normative)

- Implicit conversions are forbidden between numeric types.
- Explicit conversions use `as` (for example, `i64 as f64`).
- Converting `bool` to numeric is forbidden.

---

## 10. Validated Examples

### 10.1 Hello World
```tupa
fn main() {
print("🌩️ Hello, Tupã!")
}
```

### 10.2 MNIST Inference (Sparse Tensor)
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

### 10.3 Alignment-Guaranteed Summarization
```tupa
fn summarize(article: string): Safe<string, !misinformation, !hate_speech> {
	// Compiler requires a proof of safety via:
	// 1. RLHF score > 0.95 on the validation dataset
	// 2. Formal verification of not generating forbidden content
	return llm.generate(f"Summarize objectively: {article}")
}

fn main() {
	let article = load_article("news.tp")
	let summary = summarize(article)  // ✅ Compiles only if safety is proven
	publish(summary)  // Never publishes dangerous content
}
```

### 10.4 Differentiable Fraud Detection (Neurosymbolic)
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

// Training via gradient descent
fn train_step(batch: [Transaction], targets: [f64], lr: f64) {
	let (loss, grad) = ∇compute_loss(batch, targets)
	update_weights(grad, lr)
}
```

---

## 11. Diagnostics (Normative)

### 11.1 Error Format

The compiler **must** report errors with:

- Error code (`E####`)
- Short message
- Span with line/column (1-based)
- Code snippet with highlight

Example:

```
error[E0003]: expected ';' after expression
	--> examples/hello.tp:3:18
	 |
 3 | 	let age = 28
	 |                 ^
```

### 11.2 Warning Format

Warnings follow the same format, with the `warning[W####]` prefix.

**Note (informative)**: Tools may offer equivalent JSON output containing `code`, `message`, `label`, `span`, `line`, and `col` for editor and automation integration.

### 11.3 Span Semantics

- The span **must** point to the token causing the error when possible.
- For EOF errors, the span **must** point to the end of the file.

### 11.4 Type Diagnostics (Normative)

The compiler **must** emit type errors with a code and, when possible, with a span:

```
error[E2001]: type mismatch: expected I64, got Bool
	--> examples/invalid_type.tp:2:15
	 |
 2 | 	let x: i64 = true;
	 |               ^^^^
```

For incorrect arity:

```
error[E2002]: arity mismatch: expected 2, got 1
	--> examples/invalid_call.tp:6:10
	 |
 6 | 	let y = add(1);
	 |          ^^^^^^
```

---

## 12. Versioning Policy

- **Major** (v1 → v2): Breaking changes in grammar or type system
- **Minor** (v0.1 → v0.2): Backward-compatible features (for example, new attributes)
- **Patch** (v0.1.0 → v0.1.1): Bug fixes without spec changes

> **Commitment**: stable API starting from v1.0.

---

## 13. References & Influences

| Language/Project | Influence on Tupã |
|-------------------|-------------------|
| **Rust** | Ownership model, pattern matching, GC-free safety |
| **Zig** | Zero hidden allocations, radical simplicity |
| **Mojo** | Native differentiability, Python performance |
| **Swift** | Gradual typing, C interoperability |
| **Lean** | Formal verification integrated into the language |
| **JAX** | Functional transformations (`grad`, `jit`) as primitives |

---

*Specification maintained by the Tupã community • License: CC-BY-SA 4.0*  
*Version: 0.1-draft*
