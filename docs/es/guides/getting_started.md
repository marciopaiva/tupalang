# Guía de inicio rápido

## Propósito

Proporcionar el camino mínimo para compilar el proyecto y ejecutar el primer ejemplo.

## Requisitos previos

- Rust estable (vía rustup)
- Git

## Pasos

1) Clona el repositorio:

```bash
git clone https://github.com/marciopaiva/tupalang.git
cd tupalang
```

1) Ejecuta el ejemplo básico:

```bash
cargo run -p tupa-cli -- parse examples/hello.tp
cargo run -p tupa-cli -- parse examples/lambda_basic.tp
```

1) Ejecuta el verificador de tipos:

```bash
cargo run -p tupa-cli -- check examples/hello.tp
cargo run -p tupa-cli -- check examples/lambda_basic.tp
```

1) Salida JSON (opcional):

```bash
cargo run -p tupa-cli -- parse --format json examples/hello.tp
```

1) Ejecuta las pruebas golden (recomendado para validar el pipeline completo):

```bash
cargo test -p tupa-cli --test cli_golden
```

## Próximos pasos

- Explora [Ejemplos](../../examples/README.md)
- Configura el entorno en [Entorno de desarrollo](dev_env.md)
