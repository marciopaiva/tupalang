# Guia de Exemplos

## Propósito

Definir critérios de curadoria e padrões para exemplos.

## Onde colocar exemplos

- Exemplos curados: `examples/`
- Migração de `.tp`: `examples/migration/`

## Critérios de curadoria

- Seja pequeno e focado.
- Cubra um conceito específico.
- Prefira código que passe em `check`.
- Evite dependências externas.

## Padrões

- Nomeie arquivos por tema (`credit_decision.rs`, `fraud_complete.rs`).
- Inclua comentários breves quando necessário.
- Atualize `examples/README.md` ao adicionar/remover exemplos.
- Prefira `Safe<string, ...>` ao ilustrar restrições éticas.
- Mencione novos exemplos `safe_*` em `examples/README.md`.
- Use os exemplos do crate `tupa-engine` como referência de pipelines com constraints.

## Lista de verificação

- [ ] Arquivo adicionado em `examples/`
- [ ] Referenciado em `examples/README.md`
- [ ] Compila/roda com `cargo run -p tupa-engine --example <nome>`

## Atualizando goldens

Se a saída dos exemplos mudar de propósito (por exemplo, melhorias de formatação), atualize os arquivos goldens em `examples/expected/` usando o script fornecido:

```bash
# Atualiza todos os goldens rodando o CLI local
bash scripts/update-goldens.sh

# Depois verifique as mudanças e faça commit
git add examples/expected && git commit -m "test: update examples goldens" && git push
```text

No CI, os testes goldens falham se a saída real diferir dos arquivos em `examples/expected/`.
