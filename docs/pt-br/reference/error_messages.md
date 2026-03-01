# Guia de Mensagens de Erro

## Propósito

Padronizar o conteúdo e o formato das mensagens de erro.

## Padrão

- Mensagem curta e objetiva.
- Inclui tipos esperado/encontrado quando aplicável.
- Mostra código (`E####`) quando disponível.
- Aponta para o span correto (linha/coluna).

## Exemplos

### Tipo incompatível

```text
error[E2001]: type mismatch: expected I64, got F64
  --> examples/types.tp:4:10
```

### Variável indefinida

```text
error[E1002]: undefined variable 'x'
  --> examples/types.tp:2:1
```

### Restrição não comprovada

```text
error[E3002]: cannot prove constraint 'hate_speech' at compile time
  --> examples/invalid_safe_hate_speech.tp:2:38
```

### Restrição não comprovada (misinformation)

```text
error[E3002]: cannot prove constraint 'misinformation' at compile time
  --> examples/invalid_safe_misinformation.tp:2:41
```

### Restrição não comprovada com sugestão

```text
error[E3002]: constraint !misinformation not proven
  --> pipeline.tp:42:15
   |
42 | let summary = summarize(article)
   |               ^^^^^^^^^^^^^^^^^^^
   |
   = help: add safety proof: `@safety(score=0.98, dataset="factcheck-v3")`
   = note: required by return type `Safe<string, !misinformation>`
```

### Restrição não comprovada (misinformation, JSON)

```json
{
  "error": {
    "code": "E3002",
    "col": 41,
    "label": "examples/invalid_safe_misinformation.tp",
    "line": 2,
    "line_text": "\tlet x: Safe<string, !misinformation> = \"ok\";",
    "message": "cannot prove constraint 'misinformation' at compile time\nhelp: constraint must be provable at compile time; use a provable literal or pass a Safe value already proven",
    "span": {
      "end": 56,
      "start": 52
    }
  }
}
```

### Restrição inválida

```text
error[E3001]: invalid constraint 'hate_speech' for base type F64
  --> examples/invalid_safe_hate_speech_base.tp:2:35
```

### Restrição inválida (misinformation)

```text
error[E3001]: invalid constraint 'misinformation' for base type F64
  --> examples/invalid_safe_misinformation_base.tp:2:38
```

## Referências

- [Glossário de Diagnósticos](diagnostics_glossary.md)
- [Checklist de Diagnósticos](diagnostics_checklist.md)
- [invalid_safe_misinformation.tp](../../examples/invalid_safe_misinformation.tp)
- [invalid_safe_misinformation_base.tp](../../examples/invalid_safe_misinformation_base.tp)
