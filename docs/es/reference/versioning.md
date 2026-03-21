# Guía de Versionado

## Propósito

Definir la política de versionado y compatibilidad para lenguaje, crates y distribución binaria.

## SemVer

Seguimos [SemVer](https://semver.org/):

- **MAJOR**: cambios incompatibles.
- **MINOR**: nuevas features compatibles.
- **PATCH**: correcciones compatibles.

## Pre-1.0

Antes de 1.0, los cambios pueden ocurrir con más frecuencia. Seguimos SemVer y documentamos cambios incompatibles en CHANGELOG.

## Release Candidates

Los release candidates usan el formato `vX.Y.Z-rc.N` y deben tratarse como builds pre-GA.

- Los tags RC publican artifacts de release para validación.
- Las garantías de API se limitan a superficies estables documentadas.

## Modelo de distribución (v0.8.1)

Tupa usa un modelo híbrido:

- Artifacts binarios standalone para adopción de usuarios finales.
- Crates Rust públicas para embedding en sistemas Rust.

Ver [Decisión de distribución híbrida](../governance/hybrid_distribution_decision.md).

## Sincronización de documentación

Cualquier cambio relevante en docs, ejemplos o API debe reflejarse en CHANGELOG y, si aplica, en index.md y README.
