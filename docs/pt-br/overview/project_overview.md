# Visão Geral do Projeto

## Propósito

Resumir a missão, princípios e status do projeto.

## Missão

Construir uma linguagem brasileira para governança de IA em sistemas críticos com segurança formal, determinismo e desempenho previsível.

## Princípios

- Segurança e alinhamento via tipos.
- Determinismo e auditabilidade por design.
- Integrar sem perder governança — toda chamada Python é rastreada, validada e auditável.
- Diferenciabilidade nativa.
- Esparsidade declarativa.
- Desempenho previsível via LLVM.

## Status atual

- Especificação v0.1 completa.
- Lexer, parser, verificador de tipos e CLI básicos.
- Saída JSON no CLI.
- Geração de código funcional (IR textual).

## Exemplo de orquestração de pipeline (rascunho)

```tupa
pipeline FraudTraining {
  data = load_dataset("fraud.csv")
  model = python.train("torch_script.py", data)

  validate(model) {
    constraint accuracy >= 0.95
    constraint no_nan(model)
  }

  audit(hash_for_all: true)
  export("fraud_model_v1.tupamodel")
}
```

## Onde contribuir

- Issues para bugs e melhorias.
- RFCs com o prefixo `[RFC]`.
