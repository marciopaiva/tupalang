# Especificação da Linguagem Tupã v0.1

> **Força ancestral, código moderno**  
> Linguagem brasileira para sistemas críticos e IA em evolução

![Specification Status](https://img.shields.io/badge/status-draft-orange)
![License](https://img.shields.io/badge/license-CC--BY--SA%204.0-ff69b4)

## Propósito

Definir a especificação formal da linguagem Tupã, incluindo gramática, regras de tipo e semântica.

## Índice

- [1. Filosofia e objetivos de design](#1-filosofia-e-objetivos-de-design)
- [2. Estrutura lexical](#2-estrutura-lexical)
- [3. Sistema de tipos](#3-sistema-de-tipos)
- [4. Expressões](#4-expressões)
- [5. Instruções](#5-instruções)
- [6. Semântica numérica (Normativa)](#6-semântica-numérica-normativa)
- [6. Concorrência](#6-concorrência)
- [7. Módulos e FFI](#7-módulos-e-ffi)
- [8. Gramática EBNF completa (Normativa)](#8-gramática-ebnf-completa-normativa)
- [9. Semântica e notas de implementação](#9-semântica-e-notas-de-implementação)
- [10. Conversões de tipos (Normativa)](#10-conversões-de-tipos-normativa)
- [10. Exemplos validados](#10-exemplos-validados)
- [11. Diagnósticos (Normativa)](#11-diagnósticos-normativa)
- [12. Política de versionamento](#12-política-de-versionamento)
- [13. Referências e influências](#13-referências-e-influências)

---

## 1. Filosofia e objetivos de design

### 1.1 Princípios centrais

1. **Desempenho previsível**: zero alocações ocultas; custo de execução visível no código-fonte
2. **Diferenciabilidade nativa**: qualquer expressão pura é automaticamente diferenciável via operador `∇`
3. **Alinhamento via tipos**: restrições éticas verificadas em tempo de compilação, não em tempo de execução
4. **Esparsidade declarativa**: densidade de dados faz parte do tipo, não de uma otimização pós-processamento

### 1.2 Público-alvo

- Pesquisadores de IA que precisam de desempenho e segurança formal
- Engenheiros de sistemas críticos (fintech, saúde, infraestrutura)
- Desenvolvedores que valorizam produtividade sem sacrificar controle

### 1.3 Não objetivos

- Substituir Python para scripts rápidos
- Ser 100% compatível com a sintaxe de Rust/Python
- Suportar programação imperativa não estruturada

### 1.4 Convenções do documento

- **Normativo**: seções com gramática EBNF, regras de tipo e semântica são obrigatórias.
- **Informativo**: exemplos, notas e comentários servem como orientação.

### 1.5 Escopo do MVP (núcleo)

- Lexer + parser para funções, `let`, `if`, `match`, chamadas e literais.
- Verificador de tipos para tipos primitivos e tuplas simples.
- Semântica do `∇` limitada a funções puras.
- Geração de código para expressões aritméticas básicas.

---

## 2. Estrutura lexical

### 2.1 Codificação de caracteres

- UTF-8 obrigatório
- Identificadores suportam letras Unicode (`\p{L}`) + `_`
- Palavras-chave são apenas ASCII (case-sensitive)

### 2.2 Comentários

```tupa
// Comentário de linha única

/* Comentário
   de múltiplas linhas */
```

### 2.3 Identificadores

```ebnf
identifier = letter { letter | digit | "_" } ;
letter     = "a".."z" | "A".."Z" | "\u{0080}".."\u{10FFFF}" ;
digit      = "0".."9" ;
```

**Normalização Unicode (Normativo)**:

- Identificadores são comparados após normalização NFC.
- O compilador deve rejeitar identificadores que mudam após a normalização (para evitar confusão visual).

**Exemplos válidos**: `x`, `_temp`, `ação`, `π_value`  
**Exemplos inválidos**: `1var`, `@name`, `fn` (palavra-chave)

### 2.4 Palavras-chave

```text
fn let if else match while for in return async spawn await
pipeline step
true false null i64 f64 f32 f16 bool string tensor option result
safe unsafe extern import export
```

### 2.5 Literais

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
1.5e-3      // notação científica
"Olá 🌩️"   // string com Unicode
"newline\n" // sequência de escape
[1, 2, 3]   // tensor_literal
```

---

## 3. Sistema de tipos

### 3.1 Tipos primitivos

| Tipo | Descrição | Tamanho | Exemplo |
| --- | --- | --- | --- |
| `i64` | Inteiro com sinal | 64-bit | `42` |
| `f64` | IEEE 754 double | 64-bit | `3.14` |
| `f32` | IEEE 754 float | 32-bit | `1.0f32` |
| `f16` | Meia precisão | 16-bit | `0.5f16` |
| `bool` | Booleano | 1-bit | `true` |
| `string` | UTF-8 imutável | dinâmico | `"Tupã"` |

### 3.2 Tipos compostos

#### 3.2.1 Tuplas

```ebnf
tuple_type = "(" type { "," type } [","] ")" ;
```

```tupa
let pair: (i64, string) = (42, "answer")
let first = pair.0  // 42
```

#### 3.2.2 Tipos de função (Normativo)

```ebnf
func_type = "fn" "(" [ type { "," type } ] ")" "->" type ;
```

```tupa
let f: fn(i64, i64) -> i64 = add
let g: fn() -> bool = is_ready
// Função anônima (lambda)
let inc: fn(i64) -> i64 = |x| x + 1
// Função como valor
let apply: fn(fn(i64)->i64, i64) -> i64 = |f, x| f(x)
let r = apply(inc, 10) // r = 11
// Função com print e concatenação de string
fn hello(name: string) {
  print("Olá, " + name)
}
hello("Tupã")
```

**Comparação:**

| Tupã | Python | Rust |
| --- | --- | --- |
| `let inc: fn(i64)->i64 = \|x\| x+1` | `inc = lambda x: x+1` | `let inc = \|x: i64\| x+1;` |
| `print("Olá, " + name)` | `print("Olá, " + name)` | `println!("Olá, {}", name);` |

Veja mais exemplos no [Guia de exemplos](../guides/examples_guide.md) e no [examples/README.md](../../examples/README.md).

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

#### 3.2.4 Option / Result (tratamento de erros)

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

#### 3.2.5 Tensores (IA de primeira classe)

```ebnf
tensor_type = "Tensor" "<" 
              type "," 
              "shape" "=" "[" dimension { "," dimension } "]" 
              [ "," "density" "=" float_literal ] 
            ">" ;
dimension   = integer_literal | "..." ;  // "..." = dimensão dinâmica
```

```tupa
// Tensor denso 28x28 (MNIST)
let image: Tensor<f32, shape=[28, 28]> = load("digit.tp")

// Tensor esparso 90% (recomendado para LLMs)
let weights: Tensor<f16, shape=[4096, 4096], density=0.1> = load("llama3.tp")
```

#### 3.2.6 Tipos de alinhamento (restrições éticas)

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

Exemplo com propagação via enum:

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

Exemplo com correspondência de padrões:

```tupa
fn handle(response: LLMResponse<Safe<string, !misinformation>>) {
  match response {
    Safe(text) => publish(text),
    Flagged(text, reason) => review(text, reason),
    Blocked(reason) => reject(reason),
  }
}
```

> **Nota**: Restrições são verificadas via:
>
> - Provas formais (para propriedades matemáticas)
> - Scores RLHF (para conteúdo gerado por LLMs)
> - Alternativa de guarda em tempo de execução (se não for possível provar em tempo de compilação)

**Semântica**:

- Se o compilador **provar** a restrição, o tipo `Safe<T, !c>` é válido.
- Se **não conseguir provar**, é erro em tempo de compilação (com sugestão de correção).
- `unsafe { ... }` pode ser usado para assumir responsabilidade explícita.

**Implementação atual (compilador)**:

- `!nan` e `!inf` são aceitos apenas para base `f64`.
- `!hate_speech` e `!misinformation` são aceitos apenas para base `string`.
- A prova é feita apenas com literais `f64` e expressões constantes (por exemplo, `1.0`, `-1.0`, `1.0 + 2.0`, `1.0 / 0.0`).
- Para restrições de `string`, o compilador só aceita valores já comprovados como `Safe<string, ...>` (propagação de variáveis/retornos).
- Se a prova não for possível, o compilador reporta um erro de restrição não comprovada.

##### 3.2.6.1 Resolução de restrições (Normativa)

Para cada restrição `!c` em `Safe<T, !c>`:

| Restrição | Requisito do solucionador | Alternativa |
| --- | --- | --- |
| `!nan` | Análise de intervalos prova `x ∈ [-∞, +∞] \ {NaN}` | `@assume(!nan)` com aviso |
| `!inf` | Limites estáticos provam `abs(x) < 1.7976931348623157e308` | `@assume(!inf)` com aviso |
| `!hate_speech` | Scorer RLHF ≥ 0.95 no dataset definido | ❌ Não permitido |
| `!misinformation` | Scorer RLHF ≥ 0.95 no dataset definido | ❌ Não permitido |

---

### 3.3 Tipos de array (Normativo)

```ebnf
array_type = "[" type ";" integer_literal "]" ;  // tamanho fixo
slice_type = "[" type "]" ;                      // tamanho dinâmico
```

```tupa
let fixed: [i64; 5] = [1, 2, 3, 4, 5]
let dynamic: [i64] = vec![1, 2, 3]
```

**Semântica (Normativa)**:

- `[T; N]` é alocado na pilha quando possível.
- `[T]` é alocado no heap e é mutável apenas se referenciado por `mut`.
- Literais `[a, b, c]` inferem `[T; N]` quando `N` é conhecido.

---

## 4. Expressões

### 4.0 Precedência de operadores (maior → menor)

| Precedência | Operadores |
| --- | --- |
| 1 | `()` `.` chamada de função |
| 2 | `∇` unário |
| 3 | `!` `-` unário |
| 4 | `**` |
| 5 | `*` `/` |
| 6 | `+` `-` |
| 7 | `<` `<=` `>` `>=` |
| 8 | `==` `!=` |
| 9 | `&&` |
| 10 | `\|\|` |

### 4.1 Regras de avaliação (Normativa)

- `if` avalia apenas o ramo selecionado.
- `a && b` usa curto-circuito: `b` é avaliado apenas se `a` for `true`.
- `a || b` usa curto-circuito: `b` é avaliado apenas se `a` for `false`.
- `match` avalia apenas o corpo do primeiro padrão compatível.

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
                  | "+" | "-" | "*" | "/" | "**" ;  // ** = exponenciação

unary_expr        = [ unary_op ] primary_expr ;
unary_op          = "!" | "-" | "∇" ;  // ∇ = operador de gradiente

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

### 4.2 Expressões-chave

#### 4.2.1 Operador de gradiente (`∇`)

```tupa
// Função pura → derivada simbólica gerada pelo compilador
fn square(x: f64): f64 { x * x }

let grad_at_3 = ∇square(3.0)  // → 6.0 (derivada: 2*x)

// Derivada parcial para múltiplos parâmetros
fn mse(pred: f64, target: f64): f64 {
  let diff = pred - target
  return diff * diff
}

let (d_pred, d_target) = ∇mse(0.8, 1.0)  // → (-0.4, 0.4)
```

**Tipo de retorno**:

- Para `f: (T1, ..., Tn) -> R`, `∇f(args)` retorna `(dT1, ..., dTn)`.
- Para `n = 1`, o retorno é um escalar `dT1`.
- O valor de `f(args)` pode ser obtido chamando `f(args)` separadamente.

##### Pureza formal (Normativa)

Uma função `f` é **pura** se e somente se:

1. Não chama funções com o atributo `@side_effects(...)`.
2. Não acessa nem modifica variáveis mutáveis não locais (`static mut`, globais).
3. Não realiza operações de E/S (`print`, `file.read`, `http.get`).
4. Não contém não determinismo (`rand()`, `time.now()`, `thread_id()`).
5. Todas as funções chamadas por `f` são puras (recursão de pureza).

> **Regra de pureza**: `∇` só funciona em expressões *puras* (sem efeitos colaterais). O compilador rejeita:
>
> ```tupa
> fn impure(x: f64): f64 {
>     print(x)  // efeito colateral!
>     return x * 2
> }
> let g = ∇impure(3.0)  // ❌ Erro: a função não é pura
> ```

#### 4.2.2 Correspondência de padrões

```tupa
match http_status {
  200 => "OK",
  404 => "Não encontrado",
  code if code >= 500 => f"Erro do servidor {code}",
  _ => "Desconhecido"
}
```

#### 4.2.3 Interpolação de strings

```tupa
let name = "Tupã"
print(f"Olá, {name}!")  // → "Olá, Tupã!"
```

---

## 5. Instruções

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

range_expr        = expression ".." expression ;  // fim exclusivo
```

### 5.2 Vinculação de variáveis

```tupa
// Inferência de tipos
let x = 42          // x: i64
let pi = 3.14       // pi: f64

// Tipo explícito (recomendado para APIs públicas)
let name: string = "Tupã"

// Mutabilidade explícita (padrão é imutável)
let mut counter = 0
counter = counter + 1  // permitido
```

### 5.3 Funções

```tupa
// Função pura (padrão) → automaticamente diferenciável
fn relu(x: f64): f64 {
  if x > 0.0 { x } else { 0.0 }
}

// Função com efeitos colaterais explícitos
@side_effects(io)
fn log(message: string) {
  print(f"[LOG] {message}")
}

// Função assíncrona
async fn fetch_user(id: i64): Result<User, string> {
  let resp = await http.get(f"/api/users/{id}")
  return parse_user(resp)
}
```

### 5.4 Controle de fluxo

```tupa
// if como expressão
let status = if temp > 100 { "crítico" } else { "normal" }

// loop while
let mut i = 0
while i < 10 {
  print(i)
  i = i + 1
}

// loop for com range
for i in 0..10 {
  print(i)  // 0, 1, 2, ..., 9 (fim exclusivo)
}
```

### 5.5 Escopo e sombreado (Normativa)

- A resolução de nomes é léxica, do bloco mais interno ao mais externo.
- O sombreado é permitido (estilo Rust).
- Redeclarar o mesmo nome no mesmo escopo é erro.

Exemplo:

```tupa
let x = 10
fn foo() {
  let x = 20
  print(x)  // 20
}
```

---

## 6. Semântica numérica (Normativa)

### 6.1 Overflow de inteiro

- Overflow em `i64` gera erro em tempo de execução (panic).
- `wrap_add`, `wrap_sub`, `wrap_mul` devem ser usados para overflow intencional.

Exemplo:

```tupa
let x: i64 = 9223372036854775807
let y = x.wrap_add(1)
```

---

## 6. Concorrência

### 6.1 Criação de tarefas

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

### 6.2 Canais

```tupa
// Criação de canal tipado
let (tx, rx): (Channel<i64>, Channel<i64>) = channel()

// Envio
await tx.send(42)

// Recebimento (bloqueante)
let value = await rx.recv()  // valor: i64

// Recebimento com timeout
match await rx.recv_timeout(1000) {  // 1000ms
  Some(v) => print(f"Recebido: {v}"),
  None => print("Timeout!")
}
```

> **Garantia**: Canais são *baseados em ownership*, tornando corridas de dados impossíveis via sistema de tipos.

---

## 7. Módulos e FFI

### 7.1 Módulos

```tupa
// math.tp
export fn square(x: f64): f64 { x * x }

// main.tp
import "math" as math

let result = math.square(5.0)
```

### 7.2 Interface de função estrangeira (C)

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

- Tipos obrigatórios: `usize`, `*const T`, `*mut T`.
- Inteiros C: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`.
- Ponteiros opacos: `*void`.
- `usize` tem o mesmo tamanho do ponteiro de dados da plataforma.
- Ponteiros não podem ser desreferenciados fora de `unsafe`.

> **Regra**: `unsafe` exige um bloco explícito, o que ajuda na auditoria.

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

(* ===== EXPRESSÕES ===== *)
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

(* ===== INSTRUÇÕES ===== *)
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

(* ===== NÍVEL SUPERIOR ===== *)
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

## 9. Semântica e notas de implementação

### 9.1 Pipeline do compilador

```text
Fonte (.tp) 
  ↓ [Lexer: nom]
Tokens 
  ↓ [Parser: descida recursiva]
AST 
  ↓ [Verificador de tipos: Hindley-Milner + solucionador de restrições]
AST tipada 
  ↓ [Geração de código: inkwell → LLVM IR]
LLVM IR 
  ↓ [Otimizador LLVM (-O3)]
Binário nativo (ELF/Mach-O/PE)
```

### 9.2 Estratégia de compilação de gradientes

Para `∇f(x)` quando `f` é pura:

1. O parser marca a função como `#[pure]` (implícito via análise de efeitos)
2. O typechecker verifica ausência de efeitos colaterais (E/S, mutação global, não determinismo)
3. A geração de código emite **dois caminhos** em LLVM IR:
   - Passo direto: código original
   - Passo reverso: derivadas simbólicas via regras de diferenciação automática (regra da cadeia, regra do produto)
4. O runtime seleciona o caminho com base no uso de `∇`

Nota: Para funções puras pequenas (<100 ops), usar diferenciação simbólica sem fita. Para funções maiores, permitir fallback para diferenciação baseada em fita com um alocador em arena.

**Exemplo de LLVM IR gerado** para `fn square(x: f64): f64 { x * x }`:

```llvm
; Passo direto
define double @square(double %x) {
  %mul = fmul double %x, %x
  ret double %mul
}

; Passo reverso (gerado automaticamente)
define { double, double } @square_grad(double %x) {
  %mul = fmul double %x, %x        ; direto
  %grad = fmul double 2.0, %x      ; derivada: 2*x
  %ret = insertvalue { double, double } undef, double %mul, 0
  %ret2 = insertvalue { double, double } %ret, double %grad, 1
  ret { double, double } %ret2
}
```

### 9.3 Verificação de tipos de alinhamento

Para `Safe<T, !constraint>`:

- O compilador consulta o **solucionador de restrições** (plugin):
  - Para `!nan`: análise de intervalos estática + propagação de restrições
  - Para `!hate_speech`: integração offline com um scorer de RLHF (limiar configurável)
- Se o solucionador não consegue provar segurança → erro em tempo de compilação com sugestão de correção
- Alternativa explícita: `unsafe { ... }` com auditoria obrigatória via `@audit(required=true)`

### 9.4 Modelo de memória

- **Alocação na pilha** preferida para valores pequenos (< 4KB)
- **Alocação em arena** para ASTs e estruturas temporárias (zero overhead de GC)
- **GC de rastreamento opcional** apenas para ciclos de referência (habilitado via atributo `@gc`)
- **Sem alocações ocultas**: todas as alocações exigem uma chamada explícita a `alloc()`

### 9.5 Diagnósticos (Normativa)

- Erros devem incluir: código, mensagem, localização e sugestão.
- Formato mínimo: `E####: mensagem (arquivo:linha:coluna)`.
- Exemplo: `E3002: não foi possível provar a restrição '!nan' em tempo de compilação (main.tp:12:5)`.

**Códigos recomendados**:

- `E1001`: erro léxico
- `E2001`: erro de tipo
- `E3001`: restrição inválida
- `E3002`: restrição não comprovada
- `E4001`: uso inválido de `unsafe`

Exemplo:
`E2001: tipos incompatíveis em atribuição (main.tp:8:12)`

Exemplo visual:

```text
error[E2001]: tipos incompatíveis
  --> main.tp:8:12
   |
8 | let x: i64 = "text"
   |            ^^^^^^^^
```

---

## 10. Conversões de tipos (Normativa)

- Conversões implícitas são proibidas entre tipos numéricos.
- Conversões explícitas usam `as` (por exemplo, `i64 as f64`).
- Converter `bool` para numérico é proibido.

---

## 10. Exemplos validados

### 10.1 Olá Mundo

```tupa
fn main() {
  print("🌩️ Olá, Tupã!")
}
```

### 10.2 Inferência MNIST (Tensor esparso)

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

### 10.3 Resumo com alinhamento garantido

```tupa
fn summarize(article: string): Safe<string, !misinformation, !hate_speech> {
  // O compilador exige uma prova de segurança via:
  // 1. Score RLHF > 0.95 no dataset de validação
  // 2. Verificação formal de não gerar conteúdo proibido
  return llm.generate(f"Resumir objetivamente: {article}")
}

fn main() {
  let article = load_article("news.tp")
  let summary = summarize(article)  // ✅ Compila apenas se a segurança for comprovada
  publish(summary)  // Nunca publica conteúdo perigoso
}
```

### 10.4 Detecção de fraude diferenciável (Neurosimbólica)

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

// Treinamento via gradiente descendente
fn train_step(batch: [Transaction], targets: [f64], lr: f64) {
  let (loss, grad) = ∇compute_loss(batch, targets)
  update_weights(grad, lr)
}
```

---

## 11. Diagnósticos (Normativa)

### 11.1 Formato de erro

O compilador **deve** reportar erros com:

- Código de erro (`E####`)
- Mensagem curta
- Span com linha/coluna (base 1)
- Trecho de código com destaque

Exemplo:

```text
error[E0003]: esperado ';' após a expressão
  --> examples/hello.tp:3:18
   |
 3 |  let age = 28
   |                 ^
```

### 11.2 Formato de aviso

Avisos seguem o mesmo formato, com o prefixo `warning[W####]`.

**Nota (informativa)**: Ferramentas podem oferecer saída JSON equivalente contendo `code`, `message`, `label`, `span`, `line` e `col` para integração com editores e automação.

### 11.3 Semântica de span

- O span **deve** apontar para o token que causa o erro quando possível.
- Para erros de EOF, o span **deve** apontar para o final do arquivo.

### 11.4 Diagnósticos de tipo (Normativa)

O compilador **deve** emitir erros de tipo com um código e, quando possível, com um span:

```text
error[E2001]: incompatibilidade de tipos: esperado I64, obtido Bool
  --> examples/invalid_type.tp:2:15
   |
 2 |  let x: i64 = true;
   |               ^^^^
```

Para aridade incorreta:

```text
error[E2002]: aridade incompatível: esperado 2, obtido 1
  --> examples/invalid_call.tp:6:10
   |
 6 |  let y = add(1);
   |          ^^^^^^
```

---

## 12. Política de versionamento

- **Major** (v1 → v2): mudanças incompatíveis na gramática ou no sistema de tipos
- **Minor** (v0.1 → v0.2): recursos compatíveis com versões anteriores (por exemplo, novos atributos)
- **Patch** (v0.1.0 → v0.1.1): correções de bugs sem mudanças na spec

> **Compromisso**: API estável a partir da v1.0.

---

## 13. Referências e influências

| Linguagem/Projeto | Influência em Tupã |
| ----------------- | ----------------- |
| **Rust** | Modelo de ownership, correspondência de padrões, segurança sem GC |
| **Zig** | Zero alocações ocultas, simplicidade radical |
| **Mojo** | Diferenciabilidade nativa, desempenho de Python |
| **Swift** | Tipagem gradual, interoperabilidade com C |
| **Lean** | Verificação formal integrada à linguagem |
| **JAX** | Transformações funcionais (`grad`, `jit`) como primitivas |

---

*Especificação mantida pela comunidade Tupã • Licença: CC-BY-SA 4.0*  
*Versão: 0.1-draft*
