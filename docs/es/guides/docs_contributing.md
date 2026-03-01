# Guía de Contribución de la Documentación

## Propósito

Estandarizar los cambios de documentación para mantener calidad y consistencia.

## Alcance

- README, docs y ejemplos.
- Páginas del wiki reflejadas desde docs.

## Política de idioma

El inglés es el idioma canónico en `docs/`. Las alternativas PT-BR viven en `docs/pt-br`. Mantén las traducciones alineadas con la versión en inglés y registra cuándo una página PT-BR esté pendiente.

## Fuente única

El contenido canónico vive en `docs/`. El wiki se actualiza automáticamente vía CI.

## Estándares de escritura

- Español claro y directo.
- Frases cortas.
- Evita jerga sin explicación.
- Usa títulos objetivos y predecibles.

## Estructura recomendada

- **Propósito** justo después del título.
- Secciones cortas con subtítulos.
- Listas para pasos y requisitos.

## Checklist de PR (docs)

- [ ] ¿El propósito está claro?
- [ ] ¿Funcionan los enlaces internos?
- [ ] ¿Los ejemplos son pequeños y ejecutables?
- [ ] ¿El contenido es consistente con la SPEC?
- [ ] ¿El wiki necesita sync?
- [ ] Si existe PT-BR, ¿está alineado o marcado como pendiente?

## Sincronización del wiki

El wiki se sincroniza automáticamente vía workflow. Si necesitas forzar, ejecuta:

```bash
bash scripts/sync-wiki.sh
```
