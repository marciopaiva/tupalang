# Guia de Contribuição para Documentação

## Objetivo

Padronizar mudanças em documentação para manter consistência e qualidade.

## Escopo

- README, docs e exemplos.
- Páginas do wiki (espelhadas a partir de docs).

## Padrões de escrita

- Português claro e direto.
- Frases curtas.
- Evitar jargões sem explicar.
- Usar títulos objetivos e previsíveis.

## Estrutura recomendada

- **Objetivo** logo após o título.
- Seções curtas com subtítulos.
- Listas para passos e requisitos.

## Checklist de PR (docs)

- [ ] O objetivo está claro?
- [ ] Links internos funcionam?
- [ ] Exemplos são pequenos e executáveis?
- [ ] O conteúdo está consistente com a SPEC?
- [ ] O wiki precisa ser sincronizado?

## Sincronização do wiki

O wiki é sincronizado automaticamente via workflow. Se precisar forçar, execute:

```bash
bash scripts/sync-wiki.sh
```
