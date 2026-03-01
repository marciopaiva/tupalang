# Guia de Issues

## Propósito

Padronizar a criação de issues com informações úteis de triagem.

## Quando abrir uma issue

- Bugs e erros inesperados.
- Propostas de melhoria (use `[RFC]`).
- Dúvidas sobre spec ou comportamento.

## Checklist

- [ ] Título claro e específico.
- [ ] Passos para reproduzir (se bug).
- [ ] Resultado esperado vs. atual.
- [ ] Logs/prints relevantes.
- [ ] Versão do Rust e do projeto.

## Exemplo (bug)

**Título**: `Parser falha com match aninhado`

**Description**:

- Passos: `tupa-cli -- parse examples/match.tp`
- Esperado: AST válido
- Atual: erro `unexpected token`

## Exemplo (RFC)

**Título**: `[RFC] Tipos opcionais`

**Description**:

- Motivação
- Alternativas
- Impacto na SPEC
