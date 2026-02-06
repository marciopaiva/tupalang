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

## Atualizando os *goldens*

Se as saídas dos exemplos mudarem de propósito (ex.: melhorias de formatação), atualize os arquivos *golden* em `examples/expected/` usando o script provido:

```bash
# Atualiza todos os goldens executando o CLI localmente
bash scripts/update-goldens.sh

# Depois verifique as mudanças e comite
git add examples/expected && git commit -m "test: update examples goldens" && git push
```

No CI, os *golden tests* falharão se a saída real diferir dos arquivos em `examples/expected/`.
