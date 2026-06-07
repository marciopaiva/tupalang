# Guia de Migração: De `.tp` para Rust-DSL

**Status:** A toolchain `.tp` legada foi **removida** na v0.9.0. O desenvolvimento ativo usa apenas Rust DSL (`pipeline!` macro).

O compilador standalone `.tp` e seus crates (`tupa-cli`, `tupa-parser`, `tupa-typecheck`, etc.) foram permanentemente removidos do workspace. Todo novo desenvolvimento usa a macro `pipeline!` em arquivos Rust.

Este guia ajuda a migrar pipelines `.tp` existentes para Rust DSL.

---

## Por Que Migrar?

- ✅ Sem toolchain separada — use apenas `cargo` e `rustc`
- ✅ Suporte completo a IDEs (rust-analyzer funciona imediatamente)
- ✅ Mensagens de erro melhores (diagnósticos do rustc com spans)
- ✅ Acesso ao ecossistema Rust (crates, macros, traits)
- ✅ Iteração mais rápida — sem debug entre linguagens
- ✅ Pronto para produção — ViperTrade usa Rust DSL exclusivamente

---

## Checklist de Migração

Para cada arquivo `.tp` no projeto:

- [ ] Criar módulo `.rs` com macro `pipeline!`
- [ ] Converter funções de step para Rust
- [ ] Converter definição do pipeline para sintaxe Rust DSL
- [ ] Atualizar `Cargo.toml` com `tupa-core` e `tupa-engine`
- [ ] Remover o arquivo `.tp`
- [ ] Executar `cargo tupa check` para validar
- [ ] Executar `cargo tupa run` para testar

---

## Passo-a-Passo

### 1. Identificar Arquivos Legados

```bash
find . -name "*.tp"
```

### 2. Criar Esqueleto do Módulo Rust

Para `estrategias/risk_limits.tp`, crie `estrategias/risk_limits.rs`:

```rust
use tupa_core::{pipeline, step, constraint, metric};

pipeline! {
    name: RiskLimits,
    input: Trade,
    steps: [
        // TODO: converter cada step
    ],
    constraints: [
        // TODO: converter cada constraint
    ]
}
```

### 3. Converter Funções de Step

Copie o corpo de cada função `.tp` para Rust. Ajustes necessários:

| `.tp` | Rust DSL |
|-------|----------|
| `fn score(s: Signal): i64` | `fn score(s: &Signal) -> i64` |
| `fn validate(t: Trade): bool` | `fn validate(t: &Trade) -> bool` |

**Mudanças importantes:**

- Tipos de retorno: `: T` → `-> T`
- Parâmetros devem ser referências (`&T`) para evitar clones
- Última expressão é retornada (como Rust)

Exemplo:

```tupa
// .tp
fn compute_risk(trade: Trade): f64 {
    trade.size * trade.price / 1_000_000.0
}
```

↓

```rust
// .rs
fn compute_risk(trade: &Trade) -> f64 {
    trade.size * trade.price / 1_000_000.0
}
```

↓

```rust
// .rs
fn compute_risk(trade: &Trade) -> f64 {
    trade.size * trade.price / 1_000_000.0
}
```

### 4. Converter Definição do Pipeline

A sintaxe é quase idêntica:

```tupa
pipeline PreTradeCheck {
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0)
    ]
}
```

↓

```rust
pipeline! {
    name: PreTradeCheck,
    input: Trade,
    steps: [
        step("risk") { compute_risk(input) }
    ],
    constraints: [
        metric("max_position").le(10_000_000.0)
    ]
}
```

### 5. Atualizar Cargo.toml

```toml
[dependencies]
tupa-core = "0.10"
tupa-engine = "0.10"
```

### 6. Criar Binário Principal (se não existir)

```rust
// src/main.rs
use sua_crate::{PreTradeCheck, Trade};
use tupa_engine::Executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pipeline = PreTradeCheck::new();
    let engine = Executor::new();
    let input = Trade { /* ... */ };
    let result = engine.run(pipeline, &input)?;
    println!("Passou: {}", result.passed);
    Ok(())
}
```

### 7. Validar

```bash
cargo tupa check
cargo tupa run
cargo test
```

### 8. Remover `.tp`

```bash
rm estrategias/risk_limits.tp
```

---

## Diferenças de Sintaxe

| Construct | `.tp` | Rust DSL |
|-----------|-------|----------|
| Pipeline declaration | `pipeline Nome {` | `pipeline! { name: Nome,` |
| Input type | `input: T` | `input: T,` (igual) |
| Step | `step("x") { expr }` | `step("x") { expr }` (igual) |
| Constraint | `metric("x").ge(v)` | `metric("x").ge(v)` (igual) |
| Return type | `fn f(): T` | `fn f() -> T` |

**Sem diferenças semânticas** — a DSL dentro de `pipeline!` foi projetada para ser idêntica ao `.tp`. A mudança principal é que agora é verificada por `rustc`.

---

## Problemas Comuns

### "cannot find macro `pipeline`"

**Causa:** Falta `use tupa_core::pipeline;` ou `tupa-core` não está no `Cargo.toml`.

**Solução:** Adicionar dependência e `use`.

---

### Step function not found

**Causa:** Função não está em escopo ou não é `pub` em outro módulo.

**Solução:** Declare funções como `pub` se cruzarem módulos, ou reorganize as definições.

---

### "cannot prove constraint at compile time"

**Causa:** O antigo verificador provava mais constraints; o Rust DSL é mais conservador.

**Solução:** A constraint ainda será verificada em runtime. É um warning, não erro. Simplifique a expressão se quiser prova em compile-time.

---

### Funções padrão do `.tp` ausentes (`abs`, `max`, `min`)

**Solução:** Use equivalents da std (`f64::abs`, `f64::max`, etc.) ou defina suas próprias helpers.

---

### Plugins Python não funcionam

**Causa:** Sistema de plugins `.tp` foi removido.

**Solução:** Reescreva plugins como Rust plugins (`tupa-plugin`) ou use `tupa-pyffi`. Veja [Plugin Tutorial](../tutorials/plugin-rust.md).

---

## Validação

1. `cargo tupa check` — sem erros
2. Unit tests para funções de step
3. Integration test do pipeline completo
4. Comparar outputs com pipeline `.tp` legado (se disponível)

---

## Ajuda

- Issues: [GitHub Issues](https://github.com/marciopaiva/tupalang/issues)
- [Pipeline Guide](../guides/pipeline_guide.md)
- Exemplos: `crates/tupa-engine/examples/`
