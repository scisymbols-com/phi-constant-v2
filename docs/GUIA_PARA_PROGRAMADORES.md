# `phi`: compilación y uso

Aplicación Rust que calcula el número áureo con una cantidad configurable de cifras decimales:

```text
φ = (1 + √5) / 2
```

## Dependencias

El proyecto ya declara todo lo necesario en `Cargo.toml`:

```toml
[package]
name = "phi"
version = "0.1.0"
edition = "2024"

[dependencies]
rug = { version = "1.30.0", default-features = false, features = ["float", "std"] }
```

No hay que ejecutar `cargo add` ni instalar *crates* manualmente. Durante la compilación, Cargo obtiene y construye:

- `rug`, para trabajar con números de precisión arbitraria;
- `gmp-mpfr-sys`, como enlace nativo;
- GMP y MPFR, requeridas por `rug::Float`.

La máquina debe disponer de las herramientas nativas compatibles que necesite `gmp-mpfr-sys`. Esta guía presupone que el entorno de compilación de Rust y dichas herramientas ya están preparados.

`Cargo.lock` fija actualmente las versiones resueltas. Para reproducirlas sin actualizar el archivo de bloqueo, usa `--locked`.

## Compilar

Desde la carpeta que contiene `Cargo.toml`:

```text
C:\proyectos\rust\phi
```

Compilación optimizada:

```bash
cargo build --release --locked
```

El ejecutable de Windows se genera en:

```text
target/release/phi.exe
```

La primera compilación puede tardar más porque puede tener que construir GMP y MPFR.

Para una compilación de desarrollo:

```bash
cargo build --locked
```

Su ejecutable se genera en `target/debug/phi.exe`.

## Ejecutar

La interfaz del programa es:

```text
phi.exe [decimales]
```

`decimales` es opcional y debe ser un entero entre `1` y `100000`:

- sin argumento, el programa calcula 1000 decimales;
- con un argumento válido, calcula la cantidad solicitada;
- con un valor inválido o más de un argumento, termina con código de salida `2`.

Ejecución mediante Cargo:

```bash
cargo run --release --locked -- 100
```

El primer `--` separa los argumentos de Cargo de los argumentos enviados a `phi`.

También puede ejecutarse directamente el binario ya compilado. Utiliza la sintaxis de ruta propia de tu entorno para invocar:

```text
target/release/phi.exe 100
```

## Ejemplo de salida

```text
Fórmula: φ = (1 + √5) / 2
Decimales solicitados: 100
Precisión interna: 400 bits
Resultado:
1.6180339887498948482045868343656381177203091798057628621354486227052604628189024497072072041893911375
```

El programa convierte la cantidad solicitada a bits mediante `log2(10)` y añade 64 bits de guarda antes de realizar el cálculo con `rug::Float`.
