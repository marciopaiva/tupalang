# Guía de Ejemplos

## Propósito

Definir criterios de curación y estándares para ejemplos.

## Dónde colocar ejemplos

- Ejemplos curados: `examples/`
- Experimentos: `examples/playground/`

## Criterios de curación

- Sé pequeño y enfocado.
- Cubre un concepto específico.
- Prefiere código que pase `check`.
- Evita dependencias externas.

## Estándares

- Nombra archivos por tema (`match.tp`, `types.tp`).
- Incluye comentarios breves cuando sea necesario.
- Actualiza `examples/README.md` al agregar/eliminar ejemplos.
- Prefiere `Safe<string, ...>` al ilustrar restricciones éticas.
- Menciona nuevos ejemplos `safe_*` en `examples/README.md`.
- Usa `safe_misinformation_hate_speech.tp` como referencia de restricciones combinadas.

## Lista de verificación

- [ ] Archivo agregado en `examples/`
- [ ] Referenciado en `examples/README.md`
- [ ] Ejecuta con `tupa-cli -- parse|check`

## Actualizando goldens

Si la salida de los ejemplos cambia intencionalmente (por ejemplo, mejoras de formato), actualiza los archivos goldens en `examples/expected/` usando el script provisto:

```bash
# Actualiza todos los goldens ejecutando el CLI local
bash scripts/update-goldens.sh

# Luego verifica los cambios y haz commit
git add examples/expected && git commit -m "test: update examples goldens" && git push
```

En CI, las pruebas goldens fallarán si la salida real difiere de los archivos en `examples/expected/`.
