
# Changelog

## Propósito

Registrar mudanças relevantes por versão.

## 0.8.1 (2026-03-21)

- Tema do release: suporte a estratégias de produção para sistemas reais de política.
- Referência de planejamento:
  - `docs/pt-br/releases/rfc_v0.8.1_trading_strategy_support.md`

### Escopo Entregue

- Suporte de linguagem e runtime para sistemas de estratégia de produção.
- Melhorias para modelagem declarativa de estratégia:
  - outputs estruturados por step
  - `reason` de primeira classe
  - suporte a score ponderado
  - padrão de input tipado para configuração com records aninhados
  - suporte declarativo a políticas temporais
- Slices de type system e runtime entregues:
  - record types
  - record literals
  - acesso tipado a campos
  - validação de schema no runtime para inputs e outputs estruturados
- Builtins temporais entregues:
  - `confirm(...)`
  - `cooldown(...)`

### Engenharia e CI Entregues

- RFC adicionada em inglês, PT-BR e espanhol para preservar a paridade de docs.
- Paridade de docs mantida durante o ciclo de planejamento e implementação.
- CI local containerizado adicionado para reduzir drift entre host e GitHub Actions.
- Docs e exemplos de trading expandidos com:
  - exemplo de pipeline guiado por configuração
  - exemplo de política temporal
- A integração com o ViperTrade foi usada como prova funcional dos slices da `0.8.1`.

### Snapshot de Validação do Workspace

- Status de preparação de release: implementação mergeada na `main`, aguardando tag e binários públicos.
- Status de validação:
  - docs parity verde
  - markdownlint verde
  - CI verde para as mudanças de linguagem e runtime mergeadas
  - CI local do ViperTrade verde contra a `main` atual

### Débito Técnico

- O acesso tipado à configuração foi resolvido pragmaticamente por meio de `input` estruturado, não por sintaxe dedicada.
- A política temporal continua declarativa na camada de policy; o estado do host continua fora do runtime da linguagem.
- A automação de release está pronta, mas a tag pública `v0.8.1` e os binários ainda não foram publicados.

## 0.8.0-rc.5 (2026-03-07)

- Correções de compatibilidade do parser para adoção dos pipelines do ViperTrade:
  - tolera declarações `type` em nível superior
  - tolera declarações `extern fn ...;` em nível superior
  - aceita nomes de step sem aspas (`step(name)`) em pipelines
- Melhoria da documentação de publicação de crates:
  - adicionado `README.md` em todos os crates publicáveis
  - adicionado `readme = "README.md"` em todos os manifests de crates

## 0.8.0 (2026-03-05)

- Tema do release: integração Python controlada e auditável para pipelines de produção.
- Princípio guia: "Integrar sem perder governança - toda chamada Python é rastreada, validada e auditável."

### Escopo Entregue

- Interoperabilidade Python (`tupa-pyffi`) para invocação segura de passos `py:module.func`.
- Resiliência de runtime com circuit breaker e suporte a async/await.
- Fluxo de backtesting com avaliação de PnL/risco e logging de auditoria estruturado.
- Melhorias de validação para shapes de tensores, atributos de pipeline e robustez de parser/typechecker.

### Engenharia e CI Entregues

- CI agora exige convenção de título de PR (`type(scope): subject`) e convenção de mensagem de commit.
- Rotulagem automática de PR por tipo de mudança (`feat`, `fix`, `docs`, `refactor`, `test`, `ci`, `chore`, `breaking`).
- Release Drafter habilitado com categorização automática.
- Proteção de branch em `main` reforçada:
  - checks obrigatórios (`pr-title-convention`, `commit-message-convention`, `lint`, `test`)
  - exigência estrita de branch atualizada
  - resolução de conversas obrigatória
  - revisão de CODEOWNERS e 1 aprovação obrigatórias
  - descarte de reviews obsoletas habilitado
- CODEOWNERS adicionado para arquivos críticos de governança e workflows.
- Governança de backport implementada:
  - validação de labels `backport-X.Y`
  - criação automática de issue de acompanhamento para PRs mergeadas com label de backport
- Operação de release documentada em `release_guide.md` e `release_cut_checklist.md`.
- Validação local padronizada com `scripts/ci-local.sh` (código + lint de docs/links).

### Snapshot de Validação do Workspace

- Checagem local completa executada em 2026-03-05: `./scripts/ci-local.sh`.
- Resultado: pass (`fmt`, `clippy`, `test`, `markdownlint`, `lychee`).
- Estado do working tree durante a validação: limpo na `main`.

### Débito Técnico

- A validação de convenção de commit ainda depende do contexto de PR; pushes diretos para branches protegidas devem permanecer bloqueados por política.
- Os quality gates de docs são fortes no CI, e a paridade multilíngue de estrutura e versão mais recente já está automatizada; a paridade semântica completa do conteúdo traduzido ainda é manual.
- O workflow de backport cria issues de acompanhamento, mas a automação de cherry-pick de backport ainda não foi implementada.
- As metas de performance estão documentadas, mas não existe dashboard de tendência no CI com histórico de latência e throughput.

## 0.7.0 (2026-02-20)

- Release: engine híbrido com governança nativa de pipelines
- CLI: `tupa run` com `--plan`, `--plan-only`, `--output`
- Runtime: relatório JSON com métricas e restrições (pass/fail), hash de auditoria
- Determinismo: `@deterministic(seed=...)` analisado e seed propagada para o PRNG
- Codegen: `ExecutionPlan` JSON com `steps`, `constraints`, `metrics`, `metric_plans`
- Validação: entrada JSON validada contra `TypeSchema` antes da execução

### Adicionado

- Backend híbrido:
  - ExecutionPlan JSON para pipelines
  - CLI `tupa codegen --format=llvm` emite `.ll` e `.plan.json`
  - Runtime de pipeline (`tupa-runtime`) e comando `tupa run`
- Validador de pipeline:
  - `@deterministic` rejeita `Random`/`Time` (E2005)
  - Restrições com métricas indefinidas (E2006)
- Sem breaking changes

### Desempenho

- Tempo de compilação (exemplo médio): alvo < 200ms
- Status: não benchmarkado explicitamente no CI; acompanhado como meta de produto
- Como medir localmente:
  - Faça build do CLI: `cargo build --quiet`
  - Comandos de benchmark (exemplo):
    - `tupa codegen --format=llvm examples/pipeline/minimal.tp`
    - `tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json`
  - Opcional: use `hyperfine` para benchmark:
    - `hyperfine --warmup 3 'tupa codegen --format=llvm examples/pipeline/minimal.tp' 'tupa run --pipeline=FraudDetection --input examples/pipeline/inputs/tx.json'`
  - Condições: Linux, Rust stable (>=1.75), builds release quando aplicável
- Hardware e condições:
  - Linux x86_64, Rust stable, máquina local de dev, cold run
- Referência de teste (imprime tempo):
  - `cargo test -p tupa-cli perf -- --nocapture`
  - Observado localmente: `codegen fraud_complete ~= 1ms`, `run fraud_complete ~= 3ms` (fora do CI, ilustrativo)

## 0.6.0 (2026-02-13)

- Inferência de construtor de enum com genéricos e restrições Safe em variants.
- Padrões de match agora suportam destructuring de construtor com padrões de tupla.
- Uso de binding em guard de match validado no typechecker.
- Diagnósticos de match não exaustivo agora apontam para spans do scrutinee.
- Adicionados testes para restrições de construtor de enum e destructuring/guards de match.
- Protótipo do motor de auditoria com hash determinístico para AST e entradas.
- Comando `tupa audit` no CLI com saída JSON para hashes.
- CLI de auditoria agora usa SHA3-256 e flag `--input`.
- Adicionado suporte a anotações `@safety` no parsing.
- Exemplo de auditoria `fraud_pipeline.tp` alinhado às restrições Safe atuais.
- Aviso `private_interfaces` do typechecker resolvido para `Ty::Enum`.

## 0.5.0 (2026-02-12)

- Conclusão das restrições do typechecker e correções de validação.
- Restrições Safe<string, ...>: diagnósticos para !hate_speech e !misinformation.
- Melhoria de clareza de diagnósticos e revisão de consistência.
- Cobertura de testes expandida com casos negativos.
- Adicionados exemplos de misinformation e goldens para Safe<string, ...>.
- Docs atualizadas com exemplos safe e referências de diagnósticos.
- Docs alinhadas com posicionamento do README e atualizações do roadmap.
- Docs incluem um exemplo rascunho de orquestração de pipeline.
- Plano de release alinhado com o roadmap de governança de pipelines.
- Diagnósticos de match agora apontam para spans de padrão inválido; adicionada cobertura de testes negativos.
- Anotações Safe agora validam restrições base; adicionados exemplos de parâmetros/retorno inválidos.
- Casos negativos de lex/parse e saídas de erro JSON adicionados aos goldens.
- Script de atualização de goldens agora cobre todos os exemplos negativos.

## 0.4.0 (2026-02-11)

- Melhorias no codegen de closures e correções de captura de ambiente.
- Melhorias de restrições no typechecker e melhor inferência de lambdas.
- Atualizações de fluxo do CLI para o pipeline typecheck/codegen.
- SPEC e erros comuns atualizados para o novo comportamento.
- Limpeza de documentação: inglês canônico, índices consolidados e entrada PT-BR.

## 0.3.0 (2026-02-07)

- Suporte a closures com captura real de variáveis (estruturas de ambiente, alocação em heap).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipo Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correções de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos de docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correções de TODOs residuais no codegen para maior robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (ainda sem captura de ambiente).
- Correções de golden tests para casos de erro (mensagens do cargo removidas).

## 0.2.0 (2026-02-06)

- Suporte a closures com captura real de variáveis (estruturas de ambiente, alocação em heap).
- Melhorias na inferência de tipos para lambdas com parâmetros Unknown.
- Suporte a compatibilidade de tipo Func com parâmetros Unknown em chamadas de função.
- Melhorias de qualidade de código: Clippy e rustfmt no CI, correções de warnings.
- Suporte básico a traits (parsing, typechecking, codegen).
- Suporte básico a enums (parsing, typechecking, codegen).
- Testes unitários adicionados ao codegen.
- Exemplo de enum adicionado à documentação.
- Índice/SUMMARY centralizado e links internos de docs.
- Sincronização de CHANGELOG, VERSIONING e RELEASE_GUIDE.
- Detecção de captura de variáveis em lambdas (closures em desenvolvimento).
- Correções de TODOs residuais no codegen para maior robustez.
- Implementação de inferência de tipos para parâmetros de lambda.
- Suporte básico a closures no codegen (ainda sem captura de ambiente).
- Correções de golden tests para casos de erro (mensagens do cargo removidas).

## 0.1.0

- Specification v0.1 publicada.
- Lexer, parser, typechecker e CLI básicos.
