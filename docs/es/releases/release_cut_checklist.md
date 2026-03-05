# Checklist de Corte de Release

Usa este checklist al crear un nuevo tag de release.

## Antes del tag

- [ ] El Release Draft está actualizado y agrupado por labels.
- [ ] El CHANGELOG fue actualizado a partir del draft.
- [ ] Los checks requeridos están en verde en `main`.
- [ ] La validación local pasó: `./scripts/ci-local.sh`.
- [ ] No hay bloqueadores abiertos en issues de alta prioridad.

## Tag y publicación

- [ ] Tag creado: `vX.Y.Z`.
- [ ] Tag enviado a origin.
- [ ] GitHub Release creado desde el draft.

## Después del release

- [ ] Anunciar el release en los canales del equipo.
- [ ] Abrir issues de seguimiento para ítems diferidos.
- [ ] Confirmar el alcance del próximo milestone.
