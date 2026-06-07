# Guía de Ejemplos

## Propósito

Definir criterios de curación y estándares para ejemplos.

## Dónde colocar ejemplos

- Ejemplos curados: `examples/`
- Migración desde `.tp`: `examples/migration/`

## Criterios de curación

- Sé pequeño y enfocado.
- Cubre un concepto específico.
- Prefiere código que pase `check`.
- Evita dependencias externas.

## Estándares

- Nombra archivos por tema (`credit_decision.rs`, `fraud_complete.rs`).
- Incluye comentarios breves cuando sea necesario.
- Actualiza `examples/README.md` al agregar/eliminar ejemplos.
- Prefiere `Safe<string, ...>` al ilustrar restricciones éticas.
- Menciona nuevos ejemplos `safe_*` en `examples/README.md`.
- Usa los ejemplos del crate `tupa-engine` como referencia de pipelines con constraints.

## Lista de verificación

- [ ] Archivo agregado en `examples/`
- [ ] Referenciado en `examples/README.md`
- [ ] Compila/ejecuta con `cargo run -p tupa-engine --example <nombre>`

## Actualizando goldens

Si la salida de los ejemplos cambia intencionalmente (por ejemplo, mejoras de formato), actualiza los archivos goldens en `examples/expected/` usando el script provisto:

```bash
# Actualiza todos los goldens ejecutando el CLI local
bash scripts/update-goldens.sh

# Luego verifica los cambios y haz commit
git add examples/expected && git commit -m "test: update examples goldens" && git push
```text

En CI, las pruebas goldens fallarán si la salida real difiere de los archivos en `examples/expected/`.
