# API y Extensibilidad

> **Actualizado para 0.9.x.** Los crates `tupa-parser`, `tupa-typecheck`,
> `tupa-codegen` y `tupa-runtime`, junto con el trait `TupaExtension`, el Schema
> Registry y el Hot Reload, fueron **eliminados** en 0.9.0.

## Extensión en la arquitectura actual

- **Step functions:** funciones Rust ordinarias llamadas desde el cuerpo de un
  `step` en la macro `pipeline!`.
- **Plugins dinámicos:** carga en tiempo de ejecución vía `tupa-plugin`
  (`PluginManager`); los plugins exportan `_tupa_plugin_name` y
  `_tupa_plugin_register`. Ver el README del crate `tupa-plugin`.
- **Python:** vía `tupa-pyffi` (ver [python_ffi_spec.md](python_ffi_spec.md)).
- **Embedding:** ver [embedding.md](embedding.md).
