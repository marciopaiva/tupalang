
# Changelog

## Propósito

Registrar mudanças relevantes por versão.

## 0.8.0 (Não lançado)

- Tema do release: integração Python controlada e auditável para pipelines de produção.
- Princípio guia: "Integrar sem perder governança — toda chamada Python é rastreada, validada e auditável."
- Escopo: orquestração PyTorch/TensorFlow via adaptadores auditados.
- Foco: rastreamento de execução, hooks de validação e esquema de log de auditoria para chamadas Python.

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
  - Observado localmente: `codegen fraud_complete ≈ 1ms`, `run fraud_complete ≈ 3ms` (fora do CI, ilustrativo)

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
