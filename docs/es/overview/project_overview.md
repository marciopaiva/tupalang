# Resumen del Proyecto

## Propósito

Resumir la misión, principios y estado del proyecto.

## Misión

Construir un lenguaje brasileño para la gobernanza de IA en sistemas críticos con seguridad formal, determinismo y rendimiento predecible.

## Principios

- Seguridad y alineación vía tipos.
- Determinismo y auditabilidad por diseño.
- Integrar sin perder gobernanza — cada llamada Python se rastrea, valida y audita.
- Diferenciabilidad nativa.
- Esparsidad declarativa.
- Rendimiento predecible vía LLVM.

## Estado actual

- Especificación v0.1 completa.
- Lexer, parser, verificador de tipos y CLI básicos.
- Salida JSON en el CLI.
- Generación de código funcional (IR textual).

## Ejemplo de orquestación de pipeline (borrador)

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

## Dónde contribuir

- Issues para bugs y mejoras.
- RFCs con el prefijo `[RFC]`.
