# Guia de exemplos

## Objetivo

Definir critérios de curadoria e padrões para exemplos.

## Onde colocar

- Exemplos curados: `examples/`
- Experimentos: `examples/playground/`

## Critérios de curadoria

- Ser pequeno e focado.
- Cobrir um conceito específico.
- Preferir código que passa no `check`.
- Evitar dependências externas.

## Padrões

- Nomear arquivos por tema (`match.tp`, `types.tp`).
- Incluir comentários breves quando necessário.
- Atualizar `examples/README.md` ao adicionar/remover exemplos.

## Checklist

- [ ] Arquivo adicionado em `examples/`
- [ ] Referência em `examples/README.md`
- [ ] Executa com `tupa-cli -- parse|check`
