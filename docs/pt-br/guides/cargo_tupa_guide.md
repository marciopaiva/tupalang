# cargo-tupa

Subcomando Cargo para desenvolvimento de políticas Tupã.

## Instalação

```bash
cargo install cargo-tupa
```text

## Comandos

### `cargo tupa check`

Valida a expansão do macro `pipeline!` e a checagem de tipos. Roda `cargo check` e filtra mensagens relevantes do Tupã.

```bash
cargo tupa check          # check current package
cargo tupa check -v       # verbose
cargo tupa check --manifest-path path/to/Cargo.toml
```text

### `cargo tupa discover`

Descobre e imprime o nome do alvo binário do pacote Cargo atual.

```bash
cargo tupa discover
cargo tupa discover --manifest-path path/to/Cargo.toml
```

O comando escaneia `Cargo.toml` procurando uma seção `[[bin]]` com um campo `name`. Se nenhuma for encontrada, ele usa `src/main.rs` e o `package.name` como nome do binário.

### `cargo tupa expand`

Expande o macro pipeline! para código Rust gerado

```bash
cargo tupa expand --pretty
cargo tupa expand --file src/pipeline.rs
```

### `cargo tupa bench`

Benchmark do pipeline executando múltiplas vezes e agregando métricas de timing.

```bash
cargo tupa bench                    # roda com iterações padrão
cargo tupa bench --iterations 100   # número customizado de iterações
cargo tupa bench --metrics          # inclui métricas de step na saída
```

**Opções:**

- `--iterations <N>` — número de iterações do benchmark (padrão: 10)
- `--metrics` — inclui métricas por-step da execução do pipeline

A saída inclui tempo total, duração média por step, e estimativas de throughput.

### `cargo tupa watch`

Observa arquivos fonte por mudanças e re-executa o pipeline automaticamente.

```bash
cargo tupa watch                    # observa src/**/*.rs por mudanças
cargo tupa watch --debounce 500     # delay customizado em ms
```

**Opções:**

- `--debounce <MS>` — delay antes de re-executar após mudanças (padrão: 300ms)

### `cargo tupa run`

Executa o pipeline definido no pacote atual com entrada JSON opcional.

```bash
# Use entrada padrão (se o pipeline suporta input unitário)
cargo tupa run

# Forneça entrada JSON via variável de ambiente
TUPA_INPUT='{"amount":1000.0,"risk_score":0.5}' cargo tupa run

# Habilita execução paralela de steps
TUPA_INPUT='{"x":42}' TUPA_PARALLEL=1 cargo tupa run

# Com um arquivo
cargo tupa run --input data.json

# Exportar métricas de execução dos steps como JSON
cargo tupa run --metrics-output metrics.json
```text

O `src/main.rs` deve ler `TUPA_INPUT` (ou usar o padrão) e chamar `Executor::run` ou `Executor::run_parallel`.

### `cargo tupa fmt`

Formata código de pipeline Rust-DSL (blocos `pipeline!`) em arquivos fonte.

```bash
cargo tupa fmt                # formata todo código de pipeline em src/
cargo tupa fmt --dry-run       # mostra o que seria alterado
cargo tupa fmt --check         # falha se algum arquivo precisa de formatação
```

> **Nota:** A toolchain `.tp` legada foi removida na v0.9.0. Este comando opera exclusivamente em código Rust DSL.

### `cargo tupa lint`

Executa análise estática em definições de pipeline Rust-DSL (macros `pipeline!`).

```bash
cargo tupa lint                # lint do pacote atual
cargo tupa lint --json         # output legível por máquina
cargo tupa lint --deny warnings # trata warnings como erros
```

> **Nota:** A toolchain `.tp` legada foi removida na v0.9.0. Este comando analisa apenas código Rust DSL.

### `cargo tupa test`

Alias para `cargo test --examples`, conveniente para executar exemplos de pipeline e testes de integração.

```bash
cargo tupa test                # executa todos os testes
cargo tupa test --example credit_decision  # teste exemplo específico
cargo tupa test -- --nocapture  # passa argumentos para cargo test
```

### `cargo tupa plugin new`

Gera um scaffold de plugin para funções de step customizadas.

```bash
cargo tupa plugin new my_plugin.rs  # cria template my_plugin.rs
```

Isso cria um template `my_plugin.rs` exportando:

- `_tupa_plugin_name()`: retorna o nome do plugin
- `_tupa_plugin_register(ctx)`: registra funções de step
- Função de step de exemplo `my_step(input: Value) -> Value`

Compile como cdylib:

```bash
cargo build --crate-type=cdylib --release
# target/release/libmy_plugin.so (ou .dll/.dylib)
```

Use no pipeline:

```rust
use tupa_plugin::PluginManager;

let mut pm = PluginManager::new();
pm.load_plugin("./target/release/libmy_plugin.so")?;

fn use_plugin(pm: &PluginManager, input: &MyInput) -> Result<Value, String> {
    pm.call("my_step", serde_json::to_value(input)?).map_err(|e| e.to_string())
}
```

## Template de Projeto

Gere um novo projeto a partir do template:

```bash
cargo generate --git https://github.com/marciopaiva/tupalang#crates/tupa-template
# ou local:
cargo generate --path crates/tupa-template
```text

O template inclui um pipeline de exemplo, Cargo.toml com dependências, e um `main.rs` que integra com `cargo-tupa run`.

## Como Funciona

- `check`: Delega para `cargo check` e filtra erros de macro Tupã.
- `run`: Compila e executa seu binário com `TUPA_INPUT` definido; o binário chama `Executor::run` ou `Executor::run_parallel`.
- `test`: Alias para `cargo test --examples`, executa testes dos exemplos.
- `fmt`: Formata código Rust-DSL (macros `pipeline!`) com regras básicas de indentação.
- `lint`: Realiza análise estática em definições de pipeline Rust-DSL (detecta steps duplicados, requires/produces indefinidos, nomes/inputs ausentes).
- `plugin new`: Gera template de plugin (`_tupa_plugin_name`, `_tupa_plugin_register`, step function de exemplo).
- `discover`: Imprime o nome do alvo binário a partir de `[[bin]]` ou `src/main.rs`.
- `expand`: Expande macro `pipeline!` para código Rust gerado (use `--pretty` para saída indentada).
- `bench`: Benchmark do pipeline executando múltiplas iterações e reportando métricas de timing agregadas.
- `watch`: Observa arquivos fonte e re-executa o pipeline em mudanças com debounce configurável.

## Notas

- Workspace-aware: use `--manifest-path` para apontar para um Cargo.toml específico.
- Execução paralela: defina `TUPA_PARALLEL=1` ou use a flag `--parallel` (quando implementada).
- Saída é impressa em stdout; erros em stderr.
