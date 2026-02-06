# Erros comuns

## Objetivo

Descrever erros frequentes e soluções rápidas.

## 1) E1002 — Variável não definida

**Causa**: variável usada antes de ser declarada.
**Solução**: declare com `let` antes do uso.

## 2) E2001 — Type mismatch

**Causa**: tipo esperado difere do encontrado.
**Solução**: ajuste a anotação de tipo ou a expressão.

## 3) E2002 — Aridade incorreta

**Causa**: número de argumentos não bate com a assinatura.
**Solução**: verifique a definição da função.

## 4) E2007 — Retorno ausente

**Causa**: função deveria retornar valor, mas não retorna.
**Solução**: adicione `return` em todos os caminhos.

## 5) E3002 — Constraint não provada

**Causa**: o compilador não consegue provar `Safe<T, ...>`.
**Solução**: use literais finitos ou simplifique a expressão.

## Referências

- [Glossário de Diagnósticos](DIAGNOSTICS_GLOSSARY.md)
- [FAQ](FAQ.md)
