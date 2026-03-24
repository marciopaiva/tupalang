# Checklist: v0.8.2

## Tema

A `v0.8.2` deve ser uma release focada em ergonomia aplicada de policy runtime.

Objetivo principal da release:

- tornar o TupaLang melhor para expressar e apoiar os fluxos de policy que o ViperTrade já está exercitando em um runtime local com características de produção

## Barra da release

Barra mínima da `v0.8.2`:

- pelo menos uma melhoria concreta de ergonomia para policy temporal
- pelo menos uma melhoria concreta para outputs estruturados de decisão
- docs/exemplos aplicados mostrando o modelo de uso guiado pelo ViperTrade
- nenhuma regressão no workflow de publicação do CLI standalone e dos crates

## Épico 1: Fundamentos de Policy Temporal

### Design

- [ ] Definir a semântica-alvo para estágios como `pending`, `degrading` e `confirmed`.
- [ ] Decidir o que pertence à linguagem/runtime e o que continua no estado do host.
- [ ] Documentar o shape preferencial de contrato para outputs de policy temporal.

### Implementação

- [ ] Adicionar uma melhoria pequena e concreta na ergonomia de guards temporais.
- [ ] Garantir que a melhoria possa ser validada por exemplos ou testes.
- [ ] Confirmar que o resultado continua compatível com garantias de execução determinística.

### Validação

- [ ] Adicionar pelo menos um exemplo aplicado que espelhe um fluxo temporal real.
- [ ] Validar o exemplo com CI local e verificação de release.

## Épico 2: Contratos Estruturados de Decisão

### Design

- [ ] Padronizar um shape recomendado para outputs de decisão.
- [ ] Cobrir campos como `action`, `stage`, `reason`, `score`, `components` e `flags`.
- [ ] Decidir o que é convenção e o que vira contrato público estável.

### Implementação

- [ ] Melhorar uma superfície de runtime/codegen/typecheck para apoiar melhor outputs estruturados.
- [ ] Manter a mudança pequena o suficiente para continuar segura para release.

### Validação

- [ ] Adicionar exemplos e docs mostrando como os resultados estruturados devem aparecer.
- [ ] Verificar compatibilidade com consumidores atuais dos crates e do CLI.

## Épico 3: Fundamentos de External Typed Effects

Este épico deve continuar estreito na `0.8.2`.

### Design

- [ ] Escrever a primeira especificação técnica para external effects tipados.
- [ ] Cobrir contratos tipados de input/output, timeout, fallback e metadata de auditoria.
- [ ] Separar explicitamente passos externos `advisory` de passos `critical`.

### Implementação

- [ ] Se a implementação começar na `0.8.2`, mantê-la experimental e estreita.
- [ ] Preferir um slice simples de external effect em vez de uma integração ampla com providers.

### Validação

- [ ] Garantir regras determinísticas de fallback em todos os caminhos experimentais.
- [ ] Documentar como isso apoia integrações advisory, como o AI Analyst do ViperTrade.

## Épico 4: Documentação Aplicada

- [ ] Adicionar uma nota curta de arquitetura aplicada conectando TupaLang e ViperTrade.
- [ ] Mostrar explicitamente a separação entre policy e runtime.
- [ ] Documentar o que fica em policy Tupa e o que continua no runtime host.
- [ ] Incluir pelo menos um exemplo de output estruturado e um exemplo de policy temporal.

## Crates e Operação de Release

- [ ] Manter a paridade dos READMEs dos crates alinhada com o posicionamento do README principal.
- [ ] Verificar se o workflow de publish continua cobrindo todos os crates em releases por tag.
- [ ] Rodar a verificação de release antes do corte da tag.
- [ ] Confirmar alinhamento entre docs, changelog e release notes antes da publicação.

## O que evitar na v0.8.2

- [ ] Não expandir o escopo para uma DSL completa de seleção de portfólio.
- [ ] Não tornar integração com AI externa obrigatória para uso normal do runtime.
- [ ] Não empurrar mudanças grandes de sintaxe sem validação aplicada.
- [ ] Não sobrecarregar a release com múltiplos experimentos de linguagem sem relação.

## Critérios de sucesso

A `v0.8.2` é bem-sucedida se:

- o TupaLang estiver materialmente melhor para policy temporal e outputs estruturados
- a release continuar pequena o bastante para ser publicada com confiança
- o ViperTrade puder apontar pelo menos uma simplificação ou clarificação concreta habilitada por essa linha
