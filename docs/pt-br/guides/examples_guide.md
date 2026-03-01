# Guia de Exemplos

## Propósito

Definir critérios de curadoria e padrões para exemplos.

## Onde colocar exemplos

- Exemplos curados: `examples/`
- Experimentos: `examples/playground/`

## Critérios de curadoria

- Seja pequeno e focado.
- Cubra um conceito específico.
- Prefira código que passe em `check`.
- Evite dependências externas.

## Padrões

- Nomeie arquivos por tema (`match.tp`, `types.tp`).
- Inclua comentários breves quando necessário.
- Atualize `examples/README.md` ao adicionar/remover exemplos.
- Prefira `Safe<string, ...>` ao ilustrar restrições éticas.
- Mencione novos exemplos `safe_*` em `examples/README.md`.
- Use `safe_misinformation_hate_speech.tp` como referência de restrições combinadas.

## Lista de verificação

- [ ] Arquivo adicionado em `examples/`
- [ ] Referenciado em `examples/README.md`
- [ ] Roda com `tupa-cli -- parse|check`

## Atualizando goldens

Se a saída dos exemplos mudar de propósito (por exemplo, melhorias de formatação), atualize os arquivos goldens em `examples/expected/` usando o script fornecido:

```bash
# Atualiza todos os goldens rodando o CLI local
bash scripts/update-goldens.sh

# Depois verifique as mudanças e faça commit
git add examples/expected && git commit -m "test: update examples goldens" && git push
```

No CI, os testes goldens falham se a saída real diferir dos arquivos em `examples/expected/`.
