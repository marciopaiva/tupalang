# Guia de mensagens de erro

## Objetivo

Padronizar o conteúdo e o formato das mensagens de erro.

## Padrão

- Mensagem curta e objetiva.
- Incluir o tipo esperado/encontrado quando aplicável.
- Exibir código (`E####`) quando disponível.
- Apontar span correto (linha/coluna).

## Exemplos

**Type mismatch**

```
error[E2001]: type mismatch: expected I64, got F64
  --> examples/types.tp:4:10
```

**Variável não definida**

```
error[E1002]: undefined variable 'x'
  --> examples/types.tp:2:1
```

## Referências

- [Glossário de Diagnósticos](DIAGNOSTICS_GLOSSARY.md)
- [Diagnostics Checklist](DIAGNOSTICS_CHECKLIST.md)
