# Glossário de Diagnósticos

## Objetivo

Listar códigos de erro e aviso emitidos pelo compilador.

## Erros

### E1001 — Tipo desconhecido
Emitido quando um tipo não existe na linguagem.

### E1002 — Variável não definida
Emitido quando uma variável é usada sem declaração prévia.

### E1003 — Função não definida
Emitido quando uma função é chamada sem definição visível.

### E2001 — Type mismatch
Emitido quando o tipo encontrado não corresponde ao esperado.

### E2002 — Aridade incorreta
Emitido quando a quantidade de argumentos em uma chamada não bate com a assinatura.

### E2003 — Operação binária inválida
Emitido quando um operador binário recebe tipos incompatíveis.

### E2004 — Operação unária inválida
Emitido quando um operador unário recebe tipo incompatível.

### E2005 — Alvo de chamada inválido
Emitido quando algo que não é função é chamado.

### E2006 — Retorno incompatível
Emitido quando o tipo retornado não corresponde ao esperado.

### E2007 — Retorno ausente
Emitido quando uma função deveria retornar valor, mas não retorna.

### E3001 — Constraint inválida
Emitido quando uma constraint não é compatível com o tipo base de `Safe<T, ...>`.

### E3002 — Constraint não provada
Emitido quando a constraint não pode ser provada em *compile-time*.

## Avisos

### W0001 — Variável não utilizada
Emitido quando uma variável é declarada e não usada.

## Referências

- [Diagnostics Checklist](DIAGNOSTICS_CHECKLIST.md)
