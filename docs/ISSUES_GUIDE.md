# Guia de issues

## Objetivo

Padronizar a criação de issues com informações úteis para triagem.

## Quando abrir issue

- Bugs e erros inesperados.
- Propostas de melhoria (use `[RFC]`).
- Dúvidas sobre spec ou comportamento.

## Checklist

- [ ] Título claro e específico.
- [ ] Passos para reproduzir (se bug).
- [ ] Resultado esperado vs atual.
- [ ] Logs/prints relevantes.
- [ ] Versão do Rust e do projeto.

## Exemplo (bug)

**Título**: `Parser falha com match aninhado`

**Descrição**:

- Passos: `tupa-cli -- parse examples/match.tp`
- Esperado: AST válido
- Atual: erro `unexpected token`

## Exemplo (RFC)

**Título**: `[RFC] Tipos opcionais`

**Descrição**:

- Motivação
- Alternativas
- Impacto na SPEC
