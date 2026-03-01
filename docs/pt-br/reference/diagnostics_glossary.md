# Glossário de Diagnósticos

## Propósito

Listar códigos de erro e aviso emitidos pelo compilador.

## Erros

### E1001 — Tipo desconhecido

Emitido quando um tipo não existe na linguagem.

### E1002 — Variável indefinida

Emitido quando uma variável é usada sem declaração prévia.

### E1003 — Função indefinida

Emitido quando uma função é chamada sem definição visível.

### E2001 — Tipo incompatível

Emitido quando o tipo encontrado não corresponde ao tipo esperado.

### E2002 — Aridade incorreta

Emitido quando o número de argumentos em uma chamada não corresponde à assinatura.

### E2003 — Operação binária inválida

Emitido quando um operador binário recebe tipos incompatíveis.

### E2004 — Operação unária inválida

Emitido quando um operador unário recebe um tipo incompatível.

### E2005 — Alvo de chamada inválido

Emitido quando algo que não é uma função é chamado.

### E2006 — Retorno incompatível

Emitido quando o tipo retornado não corresponde ao tipo esperado.

### E2007 — Retorno ausente

Emitido quando uma função deveria retornar um valor mas não retorna.

### E3001 — Restrição inválida

Emitido quando uma restrição não é compatível com o tipo base de `Safe<T, ...>`.
Exemplos: `Safe<f64, !misinformation>`, `Safe<string, !nan>`.

### E3002 — Restrição não comprovada

Emitido quando uma restrição não pode ser comprovada em tempo de compilação.
Exemplos: `Safe<string, !misinformation>` sem uma fonte comprovada.

### E5001 — Match não exaustivo

Emitido quando uma expressão `match` não cobre todos os padrões possíveis.

## Avisos

### W0001 — Variável não utilizada

Emitido quando uma variável é declarada e não é usada.

## Referências

- [Checklist de Diagnósticos](diagnostics_checklist.md)
