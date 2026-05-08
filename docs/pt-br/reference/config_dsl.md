# Config DSL — Referência

## Propósito

O Config DSL fornece uma sintaxe declarativa para definir blocos de configuração tipados que servem como pré-condições para steps de pipeline. Blocos de config são nós AST de primeira classe, permitindo validação estática das entradas do pipeline antes da execução.

## Sintaxe

```tupa
config Nome {
    tipo campo_nome: Tipo
    tipo outro_campo: OutroTipo
    ...
}
```

- `config` — palavra-chave que introduz uma declaração de configuração.
- `Nome` — identificador para o tipo de configuração (usado depois como `Nome`).
- Dentro das chaves: uma ou mais declarações `tipo`, cada uma com um nome de campo e um tipo.
- Os campos são imutáveis dentro do step; devem ser fornecidos pelo chamador.

## Semântica

- Um bloco `config` declara um novo tipo semelhante a um registro que só pode ser instanciado fora do pipeline (pelo runtime ou camada de orquestração).
- Tipos de configuração são **de entrada apenas**: não podem ser criados dentro do código Tupã; são populados a partir de dados externos (ex.: JSON) quando o pipeline inicia.
- Os campos são fortemente tipados e participam do type checker. Todos os usos dos campos de config são validados.
- Blocos de config podem aparecer no nível superior de um módulo (junto com `fn`, `step`, etc.).
- Múltiplas declarações `config` são permitidas; cada uma define um tipo distinto.

## Exemplo: Config Simples

```tupa
config ParametrosEstrategia {
    type limiar: f64
    type posicao_maxima: i64
}

step avaliar {
    input: ParametrosEstrategia
    // usar campos: limiar, posicao_maxima
    let risco = ...
    // ...
}
```

O step `avaliar` declara que requer `ParametrosEstrategia` como entrada. Em runtime, o chamador fornece um objeto JSON correspondente a `{ "limiar": 0.7, "posicao_maxima": 1000 }`. O type checker garante que `limiar` e `posicao_maxima` são usados com os tipos corretos dentro do step.

## Exemplo: Pipeline Orientado por Config (config_driven_strategy.tp)

Exemplo realista combinando pipeline e config:

```tupa
config ConfigTrading {
    type capital_inicial: f64
    type risco_maximo: f64
    type posicao_maxima: i64
}

step inicializar {
    output: { capital: f64 } = config.capital_inicial
}

step verificar_risco {
    input: { capital: f64 }
    let exposicao = capital * 0.1
    guard exposicao <= config.risco_maximo
}

step executar {
    input: { capital: f64 }
    let tamanho = min(config.posicao_maxima, capital as i64 / 2)
    // lógica de trading...
}
```

Neste pipeline:

- O `ConfigTrading` é fornecido pelo runner externo (ex.: `tupa run --config trading_config.json`).
- Cada step pode acessar `config.campo` para ler valores de configuração.
- O `guard` em `verificar_risco` usa um campo de config para impor uma política.
- O type checker valida que `config.capital_inicial`, `config.risco_maximo` e `config.posicao_maxima` existem e têm os tipos declarados.

## Boas Práticas

- Use nomes **CamelCase** para tipos de configuração.
- Mantenha configs pequenos e focados: agrupe parâmetros relacionados.
- Documente cada campo com comentários para clareza.
- Trate configs como parte do contrato público do pipeline; versione-os junto com o código.

## Relação com Outros Recursos

- **Schema Registry**: Para evolução avançada, tipos de config podem ser registrados e versionados via `SchemaRegistry` ao implantar pipelines em múltiplos serviços.
- **Plugins**: Plugins podem expor step functions que aceitam parâmetros de configuração, aumentando a reutilização.
- **Hot Reload**: Combinado com hot reload, alterar um arquivo de config pode disparar o recarregamento do pipeline com novos parâmetros sem reiniciar o runtime.
