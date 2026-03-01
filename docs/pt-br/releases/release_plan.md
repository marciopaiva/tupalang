# Plano de Lançamento (0.4.x → 1.0)

## Propósito

Definir os marcos de lançamento de v0.4.x até v1.0, alinhados ao roadmap e às fases de adoção.

## Referências

- [Roadmap](roadmap.md)
- [Plano de adoção](../governance/adoption_plan.md)
- [Guia de versionamento](../reference/versioning.md)
- [Changelog](changelog.md)

## Base (atual)

- v0.6.0 lançado com genéricos em enums, destructuring/guards em match e protótipo de auditoria.
- Diagnósticos com spans e saída JSON.
- SPEC v0.1 e documentação consolidadas.

## Marcos

### 0.5.x — Confiabilidade do compilador

- Completar restrições e validações restantes do verificador de tipos.
- Melhorar consistência dos diagnósticos e clareza dos erros.
- Expandir cobertura de testes, especialmente casos negativos.

### 0.6.x — Estabilidade da geração de código e base de pipelines

- Otimizações básicas de IR (eliminação de código morto, simplificações).
- Saída estável para `fn main()` e exemplos principais.
- Benchmarks iniciais e testes de regressão.
- Sintaxe de pipeline em rascunho (orquestração, validação, hooks de auditoria).

#### 0.6.0 — Plano estratégico

**Tema central**: Máquinas de estado com garantias formais.

##### Prioridades técnicas

1. Enums com restrições éticas (parser/verificador de tipos)
   - Sintaxe EBNF para enums com genéricos.
   - Inferência de tipo de variant.
   - Propagação de restrições em variants (`Safe<T>` dentro de `Enum<Safe<T>>`).
   - Erros claros quando restrições são violadas dentro de `match`.
   - Status: concluído
2. Pattern matching com destructuring completo
   - Destructuring de tuplas dentro de padrões.
   - Guards com acesso a bindings.
   - Checagem de exaustividade.
   - Span preciso para padrão não coberto.
   - Status: concluído
3. Motor de auditoria v0.1 (protótipo determinístico)
   - CLI `tupa audit` com saída JSON (hash + fingerprint de AST).
   - Reprodutibilidade: mesma entrada → mesmo hash em máquinas diferentes.
   - Documentação em `docs/en/governance/audit_engine.md`.
   - Status: concluído
4. Diagnósticos com sugestões acionáveis
   - Código de erro específico para restrições não comprovadas.
   - Sugestões contextuais com atributos de segurança.
   - Links para documentação de restrições.

##### Critérios de aceitação

- Genéricos de enum fazem parse e checagem de tipos com inferência correta. (concluído)
- Restrições Safe preservadas através de variants de enum e braços de `match`. (concluído)
- Matches não exaustivos são rejeitados com spans acionáveis. (concluído)
- Saída JSON do `tupa audit` inclui hash SHA3-256 e fingerprint da AST.
- Saída de auditoria é estável em duas execuções independentes.
- Diagnósticos incluem hint de ajuda quando falta prova de segurança.
- `examples/audit/fraud_pipeline.tp` compila apenas com `@safety` válido.

##### Roadmap semanal

- Semana 1: Enums + genéricos em parser/verificador de tipos.
- Semana 1: Enums + genéricos em parser/verificador de tipos, EBNF atualizado, testes de parsing.
- Semana 2: Propagação de restrições em enums, 15+ testes com `Safe<T>` em variants.
- Semana 3: Exaustividade + destructuring em match, testes negativos.
- Semana 4: Protótipo do motor de auditoria + CLI, comando `tupa audit`, docs iniciais.
- Semana 5: Refino de diagnósticos com sugestões, testes goldens.
- Semana 6: RC + docs, CHANGELOG, exemplos reais em `examples/audit/`.

##### Fora de escopo

- Backend LLVM completo.
- FFI Python.
- Operador `∇`.
- Async/await.

##### Métrica de sucesso

- Um pipeline de decisão de crédito com estados approve/review/reject compila com prova formal de segurança em menos de 50 linhas.

### 0.7.x — Base de tooling e orquestração

- Formatador oficial (`fmt`) com conjunto de regras mínimo.
- Linter mínimo (`lint`) para checagens de estilo e segurança.
- Estabilização do CLI com `build`, `run`, `fmt`, `check`.

### 0.8.x — Integração controlada de Python e auditabilidade

- Execução de PyTorch/TensorFlow via adaptadores controlados.
- Chamadas Python rastreáveis com hooks de validação.
- Esquema de log de auditoria para execução externa (integrações Python).

### 0.9.x — Interoperabilidade

- FFI com C/Rust e ABI documentada.
- Bindings mínimos para bibliotecas essenciais e exemplos.
- Servidor de linguagem com autocomplete, diagnósticos e ir para definição.

### 1.0.0 — Qualidade e confiança

- Benchmarks públicos e reprodutíveis.
- Política de compatibilidade auditada e aplicada.
- SPEC finalizada com EBNF, exemplos validados e diagnósticos normativos.
- Workflows de governança validados para ambientes regulados.

## Portões de lançamento (todas as versões)

- CHANGELOG atualizado com mudanças visíveis para usuários.
- Testes e lint de docs passando.
- Exemplos principais validados.
- CI verde antes de taguear.
