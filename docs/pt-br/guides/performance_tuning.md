# Otimização de Desempenho

Guia para otimizar a execução de pipelines Tupã.

## Execução Paralela

### Habilitar Runtime Tokio

A execução paralela de passos (`Executor::run_parallel`) requer um runtime Tokio. Certifique-se de que seu binário use `#[tokio::main]` ou crie manualmente um runtime:

```rust
#[tokio::main]
async fn main() {
    let plan = MyPipeline::new();
    let engine = Executor::new();
    let result = engine.run_parallel(&plan, &input).await?;
}
```

### Anotações de Dependências

Anotações precisas de `produces` e `requires` permitem paralelismo máximo. Especificar dependências em excesso serializa a execução.

```rust
pipeline! {
    steps: [
        step("fetch")   { fetch_data(input) }  produces ["raw"],
        step("parse")   { parse(&raw) }       requires ["raw"] produces ["parsed"],
        step("validate"){ validate(&parsed) } requires ["parsed"],
        // métricas independentes podem rodar em paralelo com parse
        step("log_count") { count_logs(input) }
    ]
}
### Grau de Paralelismo

Os threads de trabalho padrão do Tokio são iguais ao número de núcleos da CPU. Sobrescreva via `tokio::runtime::Builder` se necessário:

```rust
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()
    .build()?;
```

## Eficiência de Memória

### Evitar Clonagens Desnecessárias

Funções de passo que recebem `&Input` e retornam valores owned já são ótimas. Evite clonar estruturas grandes dentro dos passos; use referências quando possível.

```rust
fn process(data: &LargeStruct) -> Metric {
    // empresta, não clona
    data.compute_metric()
}
```

### Reutilizar Dados de Entrada

Se múltiplos passos precisam dos mesmos dados derivados, compute uma vez em um passo inicial e produza uma métrica para passos downstream.

## Overhead de FFI de Plugin

Chamadas dinâmicas de plugin (`PluginManager::call`) incorrem em custo de transição FFI (serialização/deserialização + chamada C). Para cenários de alto throughput:

- Operações em lote: projete plugins que aceitem arrays de entradas e retornem arrays de saídas.
- Mantenha a lógica do plugin leve; delegue trabalho pesado para passos Rust-native quando possível.
- Perfile com `cargo bench --bench plugin_bench` para medir overhead.

Overhead esperado: ~0,5–2µs por chamada em x86_64 (varia com tamanho da entrada). Se for significativo, considira funções de passo in-process ao invés de FFI.

## Benchmarking

Use benchmarks `criterion` para medir desempenho:

```bash
# Benchmarks de engine
cargo bench --bench engine_bench -p tupa-engine

# Benchmarks de FFI de plugin
cargo bench --bench plugin_bench -p tupa-plugin
```

Métricas-chave a monitorar:

- Throughput sequencial de passos (passos/seg)
- Speedup paralelo vs sequencial (ideal: quase-linear para passos independentes)
- Overhead de verificação de constraints (por constraint)
- Latência de chamada de plugin (µs)

## Otimização de Constraints

Constraints são avaliadas após todos os passos completarem. Para pipelines com muitos passos e constraints:

- Coloque constraints diretamente em métricas produzidas por passos (evite recomputar valores derivados).
- Use operadores de comparação simples (`ge`, `le`, `eq`, `ne`, `gt`, `lt`) — são otimizados.
- Evite cálculos custosos dentro de expressões de constraint; compute uma vez em um passo e reference a métrica.

## Otimização Guiada por Perfil (PGO)

Para pipelines em produção crítica:

```bash
# 1. Build com instrumentação
cargo build --release -p tupa-engine --profile=pgo-instrument

# 2. Executar workload representativo para coletar perfis
./target/release/my_pipeline < input.json

# 3. Build com dados PGO
cargo build --release -p tupa-engine --profile=pgo-opt
```

## Configuração de Canais (Avançado)

O engine usa canais MPSC sem limite para notificações de conclusão de passo. Para contagens de passos extremamente altas (1000+), considere:

- Ajustar semântica de canais Tokio (atualmente sem limite, sem backpressure).
- Agrupar escritas de métricas em passos que produzem muitos valores.

## Problemas Comuns

| Sintoma | Causa Provável | Solução |
|---------|----------------|---------|
| Execução paralela não mais rápida que sequencial | Dependências super-especificadas (falsas) | Audite `requires`/`produces`; mantenha minimal |
| Alto uso de memória | Passos retêm alocações grandes após execução | Libere valores pesados ao fim do passo; use alocações com escopo |
| Chamadas de plugin lentas | FFI frequente de chamadas pequenas | Agrupe chamadas ou mova lógica para passos nativos |

## Leitura Adicional

- Docs de runtime Tokio: <https://docs.rs/tokio/latest/tokio/runtime/>
- Livro do Criterion: <https://bheisler.github.io/criterion.rs/book/>
- Rust Performance Book: <https://nnethercote.github.io/perf-book/>
