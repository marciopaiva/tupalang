# Sugestões adicionais (IA)

## Produto
- Mantenha um core estável e documentado
- Evite features grandes antes do MVP
- Declare claramente o que não está pronto

## Comunidade
- Centralize suporte em Discussions
- Mantenha um FAQ curto e direto
- Publique RFCs para decisões de design

## Qualidade
- Testes de regressão para parsing e types
- Linters e formatters com regras mínimas
- Mensagens de erro com exemplos de correção

## Exemplos de diagnóstico
- E1002 (parser): vírgula faltando no argumento  adicione `,`
- E2001 (types): `!nan` não provada  valide divisor ou use `Safe<f64, !nan>` com evidência
- E4003 (alignment): score RLHF abaixo do limiar → eleve threshold ou forneça dataset validado

## Integração com IA
- Saída `--json` nos diagnósticos para automação
- Use mensagens curtas, com sugestões acionáveis
