# Tupã Language Specification v0.1

> **Força ancestral, código moderno**  
> Linguagem brasileira para sistemas críticos e IA evolutiva

[![Specification Status](https://img.shields.io/badge/status-draft-orange)](#)
[![License](https://img.shields.io/badge/license-CC--BY--SA%204.0-ff69b4)](#)

---

## 1. Philosophy & Design Goals

### 1.1 Core Principles
1. **Performance previsível**: Zero alocações ocultas; custo de execução visível no código-fonte
2. **Diferenciabilidade nativa**: Qualquer expressão pura é automaticamente derivável via operador `∇`
3. **Alignment via tipos**: Restrições éticas verificadas em *compile-time*, não runtime
4. **Esparsidade declarativa**: Densidade de dados é parte do tipo, não otimização pós-processo

### 1.2 Target Audience
- Pesquisadores de IA que precisam de performance + segurança formal
- Engenheiros de sistemas críticos (fintech, saúde, infraestrutura)
- Devs que valorizam produtividade sem sacrificar controle

### 1.3 Non-Goals
- Substituir Python para scripts rápidos
- Ser 100% compatível com Rust/Python syntax
- Suportar programação imperativa não estruturada

### 1.4 Document Conventions
- **Normative**: seções com gramática EBNF, regras de tipos e semântica são obrigatórias.
- **Informative**: exemplos, notas e comentários servem como guia.

### 1.5 MVP Scope (Core)
- Lexer + parser para funções, `let`, `if`, `match`, chamadas e literais.
- Type checker para tipos primitivos e tuplas simples.
- Semântica de `∇` limitada a funções puras.
- Geração de código para expressões aritméticas básicas.

---

## 2. Lexical Structure

### 2.1 Character Encoding
- UTF-8 obrigatório
- Identificadores suportam Unicode letters (`\p{L}`) + `_`
- Keywords são ASCII-only (case-sensitive)

### 2.2 Comments
```tupa
// Comentário de linha única

/* Comentário
   multilinha */
```

### 2.3 Identifiers
```ebnf
identifier = letter { letter | digit | "_" } ;
letter     = "a".."z" | "A".."Z" | "\u{0080}".."\u{10FFFF}" ;
digit      = "0".."9" ;
```

**Normalização Unicode (Normative)**:
- Identificadores são comparados após normalização NFC.
- O compilador deve rejeitar identificadores que mudem após normalização (para evitar confusão visual).

**Exemplos válidos**: `x`, `_temp`, `ação`, `π_value`  
**Exemplos inválidos**: `1var`, `@name`, `fn` (keyword)

### 2.4 Keywords
```
fn let if else match while for in return async spawn await
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

**Exemplos**:
```tupa
42          // integer_literal
3.14        // float_literal
1.5e-3      // scientific notation
"Olá 🌩️"   // string com Unicode
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
```

#### 3.2.3 Option / Result (error handling)
```ebnf
option_type = "Option" "<" type ">" ;
result_type = "Result" "<" type "," type ">" ;
```
```tupa
fn divide(a: f64, b: f64) -> Result<f64, string> {
	if b == 0.0 {
		return Err("Divisão por zero")
	}
	return Ok(a / b)
}
```

#### 3.2.4 Tensors (IA first-class)
```ebnf
tensor_type = "Tensor" "<" 
				type "," 
				"shape" "=" "[" dimension { "," dimension } "]" 
				[ "," "density" "=" float_literal ] 
			  ">" ;
dimension   = integer_literal | "..." ;  // "..." = dynamic dimension
```
```tupa
// Tensor denso 28x28 (MNIST)
let image: Tensor<f32, shape=[28, 28]> = load("digit.tp")

// Tensor esparso 90% (recomendado para LLMs)
let weights: Tensor<f16, shape=[4096, 4096], density=0.1> = load("llama3.tp")
```

#### 3.2.5 Alignment Types (ethical constraints)
```ebnf
safe_type = "Safe" "<" type "," constraint_list ">" ;
constraint_list = "!" identifier { "," "!" identifier } ;
```
```tupa
// Texto que não pode conter discurso de ódio
let summary: Safe<string, !hate_speech> = summarize(article)

// Número que não pode ser NaN/Inf (crítico para treinamento estável)
let loss: Safe<f64, !nan, !inf> = compute_loss(predictions, targets)
```

> **Nota**: Constraints são verificadas via:
> - Provas formais (para propriedades matemáticas)
> - RLHF scores (para conteúdo gerado por LLM)
> - Runtime guards fallback (se compile-time não puder provar)

**Semântica**:
- Se o compilador **provar** a constraint, o tipo `Safe<T, !c>` é válido.
- Se **não puder provar**, é erro de compilação (com sugestão de correção).
- `unsafe { ... }` pode ser usado para assumir responsabilidade explícita.

##### 3.2.5.1 Constraint Resolution (Normative)

Para cada constraint `!c` em `Safe<T, !c>`:

| Constraint | Solver Requirement | Fallback |
|------------|--------------------|----------|
| `!nan` | Interval analysis prova `x ∈ [-∞, +∞] \ {NaN}` | `@assume(!nan)` com warning |
| `!inf` | Limites estáticos provam `abs(x) < f64::MAX` | `@assume(!inf)` com warning |
| `!hate_speech` | RLHF scorer ≥ 0.95 no dataset definido | ❌ Não permitido |

---

### 3.3 Array Types (Normative)

```ebnf
array_type = "[" type ";" integer_literal "]" ;  // tamanho fixo
slice_type = "[" type "]" ;                      // tamanho dinâmico
```

```tupa
let fixed: [i64; 5] = [1, 2, 3, 4, 5]
let dynamic: [i64] = vec![1, 2, 3]
```

**Semântica (Normative)**:
- `[T; N]` é alocado na stack quando possível.
- `[T]` é alocado no heap e é mutável apenas se referenciado por `mut`.
- Literais `[a, b, c]` inferem `[T; N]` quando `N` é conhecido.

---

## 4. Expressions

### 4.0 Operator Precedence (highest → lowest)
| Precedência | Operadores |
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
- `if` avalia apenas o branch selecionado.
- `a && b` usa short-circuit: `b` só é avaliado se `a` for `true`.
- `a || b` usa short-circuit: `b` só é avaliado se `a` for `false`.
- `match` avalia o corpo somente do primeiro padrão correspondente.

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
// Função pura → derivada simbólica gerada pelo compilador
fn square(x: f64) -> f64 { x * x }

let grad_at_3 = ∇square(3.0)  // → 6.0 (derivada: 2*x)

// Derivada parcial para múltiplos parâmetros
fn mse(pred: f64, target: f64) -> f64 {
	let diff = pred - target
	return diff * diff
}

let (d_pred, d_target) = ∇mse(0.8, 1.0)  // → (-0.4, 0.4)
```

**Tipo de retorno**:
- Para `f: (T1, ..., Tn) -> R`, `∇f(args)` retorna `(dT1, ..., dTn)`.
- Para `n = 1`, o retorno é escalar `dT1`.
- O valor de `f(args)` pode ser obtido chamando `f(args)` separadamente.

##### Pureza Formal (Normative)

Uma função `f` é **pura** sse:

1. Não contém chamadas a funções com atributo `@side_effects(...)`.
2. Não acessa nem modifica variáveis mutáveis não-locais (`static mut`, globals).
3. Não contém operações de I/O (`print`, `file.read`, `http.get`).
4. Não contém non-determinismo (`rand()`, `time.now()`, `thread_id()`).
5. Todas as funções chamadas por `f` são puras (recursão de pureza).

> **Regra de pureza**: `∇` só funciona em expressões *puras* (sem side effects). Compilador rejeita:
> ```tupa
> fn impure(x: f64) -> f64 {
>     print(x)  // side effect!
>     return x * 2
> }
> let g = ∇impure(3.0)  // ❌ Erro: função não é pura
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
print(f"Olá, {name}!")  // → "Olá, Tupã!"
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
				  | function_decl ;

function_decl     = [ attribute_list ] "fn" identifier 
					"(" [ parameter_list ] ")" 
					[ ":" type ] 
					block ;

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
// Inferência de tipo
let x = 42          // x: i64
let pi = 3.14       // pi: f64

// Tipo explícito (recomendado para APIs públicas)
let name: string = "Tupã"

// Mutabilidade explícita (default é imutável)
let mut counter = 0
counter = counter + 1  // permitido
```

### 5.3 Functions
```tupa
// Função pura (default) → automaticamente diferenciável
fn relu(x: f64) -> f64 {
	if x > 0.0 { x } else { 0.0 }
}

// Função com side effects explícitos
@side_effects(io)
fn log(message: string) {
	print(f"[LOG] {message}")
}

// Função assíncrona
async fn fetch_user(id: i64) -> Result<User, string> {
	let resp = await http.get(f"/api/users/{id}")
	return parse_user(resp)
}
```

### 5.4 Control Flow
```tupa
// if como expressão
let status = if temp > 100 { "crítico" } else { "normal" }

// while loop
let mut i = 0
while i < 10 {
	print(i)
	i = i + 1
}

// for loop com range
for i in 0..10 {
	print(i)  // 0, 1, 2, ..., 9 (exclusivo no final)
}
```

### 5.5 Escopo e Shadowing (Normative)

- Resolução de nomes é léxica, do bloco mais interno para o mais externo.
- Shadowing é permitido (estilo Rust).
- Redeclaração do mesmo nome no mesmo escopo é erro.

Exemplo:
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
- Overflow em `i64` gera erro em runtime (panic).
- `wrap_add`, `wrap_sub`, `wrap_mul` devem ser usados para overflow intencional.

Exemplo:
```tupa
let x: i64 = i64::MAX
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

// Spawn anônimo
spawn async {
	let result = await heavy_computation()
	send_to_main(result)
}
```

### 6.2 Channels
```tupa
// Criação de channel tipado
let (tx, rx): (Channel<i64>, Channel<i64>) = channel()

// Envio
await tx.send(42)

// Recebimento (blocking)
let value = await rx.recv()  // value: i64

// Recebimento com timeout
match await rx.recv_timeout(1000) {  // 1000ms
	Some(v) => print(f"Recebido: {v}"),
	None => print("Timeout!")
}
```

> **Garantia**: Canais são *ownership-based*, o que torna impossível data races pelo sistema de tipos.

---

## 7. Modules & FFI

### 7.1 Modules
```tupa
// math.tp
export fn square(x: f64) -> f64 { x * x }

// main.tp
import "math" as math

let result = math.square(5.0)
```

### 7.2 Foreign Function Interface (C)
```tupa
extern "C" {
	fn malloc(size: i64) -> *void
	fn free(ptr: *void)
}

fn main() {
	let ptr = unsafe { malloc(1024) }
	// ... uso ...
	unsafe { free(ptr) }
}
```

**ABI mínima (Normative)**:
- Tipos obrigatórios: `usize`, `*const T`, `*mut T`.
- Inteiros C: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`.
- Ponteiros opacos: `*void`.
- `usize` tem o mesmo tamanho do ponteiro de dados da plataforma.
- Ponteiros não podem ser desreferenciados fora de `unsafe`.

> **Regra**: `unsafe` requer bloco explícito, o que facilita auditoria.

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
				| option_type
				| result_type
				| tensor_type
				| safe_type
				| identifier ;

primitive_type  = "i64" | "f64" | "f32" | "f16" | "bool" | "string" ;
tuple_type      = "(" type { "," type } [","] ")" ;
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
				| function_decl ;

function_decl   = [ attribute_list ] "fn" identifier 
				  "(" [ parameter_list ] ")" 
				  [ ":" type ] 
				  block ;

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
Para `∇f(x)` onde `f` é pura:
1. Parser marca função como `#[pure]` (implícito via análise de efeitos)
2. Type checker verifica ausência de side effects (I/O, mutação global, non-determinismo)
3. Codegen gera **dois caminhos** no LLVM IR:
   - Forward pass: código original
   - Backward pass: derivadas simbólicas via regras de diferenciação automática (chain rule, product rule)
4. Runtime seleciona caminho baseado no uso de `∇`

Nota: Para funções puras pequenas (<100 ops), usar diferenciação simbólica sem tape. Para funções grandes, permitir fallback para tape-based com arena allocator.

**Exemplo LLVM IR gerado** para `fn square(x: f64) -> f64 { x * x }`:
```llvm
; Forward pass
define double @square(double %x) {
  %mul = fmul double %x, %x
  ret double %mul
}

; Backward pass (gerado automaticamente)
define { double, double } @square_grad(double %x) {
  %mul = fmul double %x, %x        ; forward
  %grad = fmul double 2.0, %x      ; derivative: 2*x
  %ret = insertvalue { double, double } undef, double %mul, 0
  %ret2 = insertvalue { double, double } %ret, double %grad, 1
  ret { double, double } %ret2
}
```

### 9.3 Alignment Type Verification
Para `Safe<T, !constraint>`:
- Compilador consulta **constraint solver** (plugin):
  - Para `!nan`: análise de intervalo estática + propagação de restrições
  - Para `!hate_speech`: integração offline com RLHF scorer (threshold configurável)
- Se solver não pode provar safety → erro de compilação com sugestão de correção
- Fallback explícito: `unsafe { ... }` com auditoria obrigatória via `@audit(required=true)`

### 9.4 Memory Model
- **Stack allocation** preferencial para valores pequenos (< 4KB)
- **Arena allocation** para ASTs e estruturas temporárias (zero GC overhead)
- **Optional tracing GC** apenas para ciclos de referência (ativado via `@gc` attribute)
- **Nenhuma alocação oculta**: todas as alocações requerem chamada explícita a `alloc()`

### 9.5 Diagnostics (Normative)
- Erros devem incluir: código, mensagem, local e sugestão.
- Formato mínimo: `E####: mensagem (arquivo:linha:coluna)`.
- Exemplo: `E2001: constraint !nan não provada (main.tp:12:5)`.

**Códigos recomendados**:
- `E1001`: erro léxico
- `E2001`: erro de tipos
- `E2002`: constraint não provada
- `E3001`: erro de borrow/mutabilidade
- `E4001`: uso de `unsafe` inválido

Exemplo:
`E2001: tipos incompatíveis em atribuição (main.tp:8:12)`

Exemplo visual:
```
error[E2001]: tipos incompatíveis
	--> main.tp:8:12
	 |
 8 | let x: i64 = "texto"
	 |            ^^^^^^^^
```

---

## 10. Type Conversions (Normative)

- Conversões implícitas são proibidas entre tipos numéricos.
- Conversões explícitas usam `as` (ex.: `i64 as f64`).
- Conversão de `bool` para numérico é proibida.

---

## 10. Validated Examples

### 10.1 Hello World
```tupa
fn main() {
	print("🌩️ Olá, Tupã!")
}
```

### 10.2 MNIST Inference (Sparse Tensor)
```tupa
fn softmax(x: Tensor<f32, shape=[10]>) -> Tensor<f32, shape=[10]> {
	let max = x.max()
	let exps = x.map(|v| (v - max).exp())
	return exps / exps.sum()
}

fn predict(image: Tensor<f32, shape=[28, 28]>) -> i64 {
	let weights: Tensor<f16, shape=[784, 10], density=0.15> = load("weights.tp")
	let flattened = image.flatten()
	let logits = matmul(flattened, weights)
	let probs = softmax(logits)
	return probs.argmax()
}
```

### 10.3 Alignment-Guaranteed Summarization
```tupa
fn summarize(article: string) -> Safe<string, !misinformation, !hate_speech> {
	// Compilador exige prova de safety via:
	// 1. RLHF score > 0.95 no dataset de validação
	// 2. Verificação formal de não geração de conteúdo proibido
	return llm.generate(f"Resuma objetivamente: {article}")
}

fn main() {
	let article = load_article("news.tp")
	let summary = summarize(article)  // ✅ Compila só se safety provada
	publish(summary)  // Nunca publica conteúdo perigoso
}
```

### 10.4 Differentiable Fraud Detection (Neurosymbolic)
```tupa
@differentiable
fn risk_score(tx: Transaction) -> f64 {
	let neural = fraud_net.infer(tx.features)  // Tensor<f16, density=0.1>
	let symbolic = if tx.country == "BR" && tx.amount > 1000.0 { 
		0.8 
	} else { 
		0.2 
	}
	return 0.7 * neural + 0.3 * symbolic
}

// Treinamento via gradiente descendente
fn train_step(batch: [Transaction], targets: [f64], lr: f64) {
	let (loss, grad) = ∇compute_loss(batch, targets)
	update_weights(grad, lr)
}
```

---

## 11. Diagnostics (Normative)

### 11.1 Error Format

O compilador **deve** reportar erros com:

- Código do erro (`E####`)
- Mensagem curta
- Span com linha/coluna (1-based)
- Trecho de código com destaque

Exemplo:

```
error[E0003]: expected ';' after expression
	--> examples/hello.tp:3:18
	 |
 3 | 	let idade = 28
	 |                 ^
```

### 11.2 Warning Format

Warnings seguem o mesmo formato, com prefixo `warning[W####]`.

**Nota (informativa)**: Ferramentas podem oferecer saída JSON equivalente contendo `code`, `message`, `label`, `span`, `line` e `col` para integração com editores e automações.

### 11.3 Span Semantics

- O span **deve** apontar para o token causador do erro quando possível.
- Para erros de EOF, o span **deve** apontar para o fim do arquivo.

### 11.4 Type Diagnostics (Normative)

O compilador **deve** emitir erros de tipo com um código e, quando possível, com span:

```
error[E2001]: type mismatch: expected I64, got Bool
	--> examples/invalid_type.tp:2:15
	 |
 2 | 	let x: i64 = true;
	 |               ^^^^
```

Para aridade incorreta:

```
error[E2002]: arity mismatch: expected 2, got 1
	--> examples/invalid_call.tp:6:10
	 |
 6 | 	let y = add(1);
	 |          ^^^^^^
```

---

## 12. Versioning Policy

- **Major** (v1 → v2): Breaking changes na gramática ou sistema de tipos
- **Minor** (v0.1 → v0.2): Novas features compatíveis (ex: novo atributo)
- **Patch** (v0.1.0 → v0.1.1): Correções de bugs sem mudanças na spec

> **Compromisso**: API estável a partir de v1.0.

---

## 13. References & Influences

| Linguagem/Projeto | Influência em Tupã |
|-------------------|-------------------|
| **Rust** | Ownership model, pattern matching, safety sem GC |
| **Zig** | Zero alocações ocultas, simplicidade radical |
| **Mojo** | Diferenciabilidade nativa, performance Python |
| **Swift** | Tipagem gradual, interoperabilidade C |
| **Lean** | Verificação formal integrada à linguagem |
| **JAX** | Transformações funcionais (`grad`, `jit`) como primitivas |

---

*Especificação mantida pela comunidade Tupã • Licença: CC-BY-SA 4.0*  
*Versão: 0.1-draft*
