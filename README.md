# Phi

Aplicación de línea de comandos escrita en Rust para calcular el número áureo con precisión arbitraria:

```text
φ = (1 + √5) / 2
```

El programa utiliza [`rug`](https://crates.io/crates/rug) y MPFR para generar la cantidad de cifras decimales indicada por el usuario. Si no se proporciona un argumento, calcula 1.000 decimales; admite valores entre 1 y 100.000.

## Compilar

```bash
cargo build --release --locked
```

## Ejecutar

Mediante Cargo:

```bash
cargo run --release --locked -- 100
```

O ejecutando directamente el binario compilado:

```bash
./target/release/phi.exe 100
```

El argumento `100` indica la cantidad de cifras decimales que se mostrarán.

## Documentación

- [Guía completa para Windows y MSYS2 MINGW64](docs/GUIA_RUST_WINDOWS_PHI.md)
- [Guía rápida para programadores](docs/GUIA_PARA_PROGRAMADORES.md)

## Proyecto

Este programa es un recurso del proyecto [SciSymbols](https://scisymbols.com).
