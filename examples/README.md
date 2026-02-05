# Exemplos

Este diretório reúne exemplos práticos de uso da linguagem.

## O que você vai ver
- `hello.tp`: exemplo canônico
- `mnist_inference.tp`: softmax e matmul com pesos esparsos
- `fraud_detector.tp`: fusão neurosimbólica e `Safe<f64, !nan>`

## Como rodar (quando o CLI estiver disponível)
```bash
# Checar e compilar
tupa check
tupa build examples/hello.tp

# Executar
tupa run build/hello
```

## Validados vs. experimentais
- Validados: prontos para rodar no MVP
- Experimentais: requerem features além do MVP
