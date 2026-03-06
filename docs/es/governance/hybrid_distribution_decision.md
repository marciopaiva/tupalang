# Decisión de Distribución Híbrida (Binario Standalone + Crates Rust Públicas)

## Estado

Aceptada para `v0.8.0-rc`.

## Contexto

Tupa debe atender dos audiencias distintas:

- Usuarios finales (equipos de ML/compliance/plataforma) que necesitan distribución simple por ejecutable.
- Integradores Rust que necesitan embeddability mediante crates estables.

Una estrategia solo binario perjudica la embeddability. Una estrategia solo crates perjudica la adopción y operación.

## Decisión

Tupa adopta un modelo de distribución híbrido:

1. Distribución principal: binario CLI standalone (`tupa-cli`) publicado en artifacts de release.
2. Distribución secundaria: crates Rust públicas seleccionadas para embedding (`tupa-parser`, `tupa-typecheck`, `tupa-runtime`).
3. Núcleo compartido: un código base y una política de CI para ambas superficies.

## Alcance para `v0.8.0-rc`

- Agregar workflow de release para generar/publicar binarios multiplataforma y checksums.
- Documentar instalación por binario y flujo de embedding en Rust.
- Mantener compromisos de estabilidad de API solo para las crates seleccionadas.

## Fuera de Alcance para este RC

- Automatización de Homebrew tap.
- Publicación de imagen Docker oficial.
- Endpoint externo de instalador (por ejemplo, `tupa.dev/install.sh`).

## Consecuencias

- Mejor camino de adopción para usuarios no Rust.
- Embeddability preservada para ecosistemas Rust.
- Responsabilidades de release más claras (artifacts + checksums + docs).
