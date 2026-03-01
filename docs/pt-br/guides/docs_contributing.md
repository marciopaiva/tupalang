# Guia de Contribuição da Documentação

## Propósito

Padronizar mudanças na documentação para manter qualidade e consistência.

## Escopo

- README, docs e exemplos.
- Páginas do wiki espelhadas a partir de docs.

## Política de idioma

Inglês é o idioma canônico em `docs/`. As alternativas em PT-BR ficam em `docs/pt-br`. Mantenha as traduções alinhadas com a versão em inglês e registre quando uma página em PT-BR estiver pendente.

## Fonte única

O conteúdo canônico vive em `docs/`. O wiki é atualizado automaticamente via CI.

## Padrões de escrita

- Português claro e direto.
- Frases curtas.
- Evite jargão sem explicação.
- Use títulos objetivos e previsíveis.

## Estrutura recomendada

- **Propósito** logo após o título.
- Seções curtas com subtítulos.
- Listas para passos e requisitos.

## Checklist de PR (docs)

- [ ] O propósito está claro?
- [ ] Links internos funcionam?
- [ ] Exemplos são pequenos e executáveis?
- [ ] O conteúdo está consistente com a SPEC?
- [ ] O wiki precisa de sync?
- [ ] Se existir PT-BR, está alinhado ou marcado como pendente?

## Sincronização do wiki

O wiki é sincronizado automaticamente via workflow. Se precisar forçar, execute:

```bash
bash scripts/sync-wiki.sh
```
