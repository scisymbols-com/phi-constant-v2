# Guía completa de Rust en Windows: instalación, Cargo, GCC y cálculo del número áureo φ

**Nivel:** principiante · **Sistema de referencia:** Windows 11 de 64 bits · **Terminales:** PowerShell y MSYS2 MinGW 64-bit · **Ejemplo:** `phi` · **Revisión:** 13 de octubre de 2026.

> **Objetivo:** empezar desde cero (o desde Rust ya instalado), preparar todas las herramientas necesarias, crear el proyecto, incorporar `rug` y sus bibliotecas de precisión arbitraria, compilar un archivo `.exe` y ejecutarlo desde la línea de comandos con un parámetro que indique el número de cifras decimales deseadas.
>
> Esta guía toma como punto de partida una versión anterior de `main.rs` para calcular **φ = (1 + √5) / 2**. Se explican sus problemas y se documenta la versión mejorada que utiliza actualmente el proyecto.

## Índice

1. [Qué es cada herramienta y por qué se necesita](#1-qué-es-cada-herramienta-y-por-qué-se-necesita)
2. [Comprobar el entorno actual](#2-comprobar-el-entorno-actual)
3. [Instalar Rust o actualizarlo](#3-instalar-rust-o-actualizarlo)
4. [Instalar MSYS2, GCC y herramientas de compilación](#4-instalar-msys2-gcc-y-herramientas-de-compilación)
5. [Seleccionar Rust GNU para este proyecto](#5-seleccionar-rust-gnu-para-este-proyecto)
6. [Configurar los terminales y comprobar las rutas](#6-configurar-los-terminales-y-comprobar-las-rutas)
7. [Crear un proyecto o recuperar el existente](#7-crear-un-proyecto-o-recuperar-el-existente)
8. [Instalar la dependencia `rug`](#8-instalar-la-dependencia-rug)
9. [Preparar `src/main.rs` y entender el programa](#9-preparar-srcmainrs-y-entender-el-programa)
10. [Comprender bits y decimales](#10-comprender-bits-y-decimales)
11. [Compilar y ejecutar con Cargo](#11-compilar-y-ejecutar-con-cargo)
12. [Ejecutar directamente el `.exe` con argumentos](#12-ejecutar-directamente-el-exe-con-argumentos)
13. [Guardar los resultados y distribuir el ejecutable](#13-guardar-los-resultados-y-distribuir-el-ejecutable)
14. [Actualizar herramientas y dependencias](#14-actualizar-herramientas-y-dependencias)
15. [Errores frecuentes y soluciones](#15-errores-frecuentes-y-soluciones)
16. [Conceptos básicos de Rust y Cargo](#16-conceptos-básicos-de-rust-y-cargo)
17. [Referencia rápida de comandos](#17-referencia-rápida-de-comandos)
18. [Documentación oficial y referencias](#18-documentación-oficial-y-referencias)

---

## 1. Qué es cada herramienta y por qué se necesita

| Elemento | Función | ¿Lo utiliza este proyecto? |
|---|---|---|
| **Rust** | Lenguaje de programación. | Sí. |
| **`rustc`** | Compilador del lenguaje Rust. | Sí, normalmente a través de Cargo. |
| **Cargo** | Crea proyectos, descarga dependencias, compila y ejecuta. | Sí. |
| **rustup** | Instala y actualiza las versiones y *toolchains* de Rust. | Sí, para preparar el entorno. |
| **Crate** | Paquete o biblioteca de Rust. | Sí: `rug`. |
| **`rug`** | Aporta `Float`, `Integer` y otros números de precisión arbitraria. | Sí: usamos `rug::Float`. |
| **GMP** | Biblioteca matemática para enteros de precisión arbitraria. | Sí, como dependencia nativa indirecta. |
| **MPFR** | Biblioteca de coma flotante multiprecisión. | Sí, para `rug::Float`. |
| **MPC** | Biblioteca de números complejos multiprecisión. | No es necesaria para el ejemplo si activamos solo las funciones de `rug` que necesitamos. |
| **`gmp-mpfr-sys`** | Enlace entre Rust y GMP/MPFR; puede compilar estas bibliotecas C desde su código fuente. | Sí, indirectamente. |
| **GCC / MinGW-w64** | Compilador de C/C++ y herramientas de enlace de la familia GNU. | Sí, para construir las dependencias nativas en Windows. |
| **MSYS2** | Proporciona GCC y utilidades Unix (`make`, `m4`, `diff`, etc.) en Windows. | Sí. |
| **PowerShell** | Terminal de Windows. | Sí, para administrar el proyecto y ejecutar el `.exe`. |

**Distinción importante:** GCC no hace que Rust tenga números más grandes por sí mismo. El cálculo de alta precisión lo realiza `rug`, apoyado en **MPFR**; GCC permite construir las bibliotecas nativas utilizadas por el paquete. Rust ofrece enteros estándar como `u32`, `u64` o `u128`, y `f64` para coma flotante de precisión fija. Para miles de decimales de φ no basta con `f64`.

### Dos familias de Rust en Windows

- **MSVC** (`x86_64-pc-windows-msvc`): integración habitual con las herramientas de Visual Studio; es la elección predeterminada frecuente para Windows.
- **GNU** (`x86_64-pc-windows-gnu`): se integra con MinGW-w64/GCC y es la configuración documentada para compilar `gmp-mpfr-sys` mediante MSYS2.

**En esta guía usaremos GNU de 64 bits para el proyecto de φ.** No hace falta desinstalar MSVC si lo tienes instalado: ambas variantes pueden coexistir.

---

## 2. Comprobar el entorno actual

Abre **Windows PowerShell** o **PowerShell 7** y ejecuta cada instrucción:

```powershell
rustup --version
rustc --version
cargo --version
rustup toolchain list
rustup show active-toolchain
rustc -vV
```

Comprueba si GCC está disponible en esa terminal:

```powershell
gcc --version
Get-Command gcc -ErrorAction SilentlyContinue
```

**Cómo interpretar la situación:**

- Si `rustc` y `cargo` responden, Rust ya está instalado.
- Si `gcc` muestra «no se reconoce como nombre de un cmdlet», **no significa que Rust esté mal instalado**: falta GCC en la ruta de búsqueda de PowerShell o todavía no se ha instalado.
- `rustc -vV` muestra una línea `host:`. Si termina en `-msvc`, estás usando la herramienta MSVC; si termina en `-gnu`, estás usando GNU.
- `rustup toolchain list` enumera todas las variantes instaladas. Puede haber más de una.

> **Para el caso original:** `rustc 1.94.1` y `cargo 1.94.1` ya estaban instalados. No es necesario reinstalarlos solo porque falte `gcc`; hay que añadir el entorno de compilación y seleccionar la variante apropiada de Rust.

---

## 3. Instalar Rust o actualizarlo

### 3.1. Primera instalación

1. Visita **https://www.rust-lang.org/tools/install** (también puedes entrar por https://rustup.rs).
2. Descarga el instalador de Windows `rustup-init.exe` apropiado para tu sistema.
3. Ejecútalo. Rust instalará `rustup`, `rustc` y `cargo`.
4. Si el instalador pregunta por MSVC/Visual Studio, recuerda que el ejemplo de esta guía utilizará **GNU**; la variante GNU se puede agregar después sin sustituir los demás componentes.
5. Cierra la terminal y abre una **nueva** ventana de PowerShell para actualizar la variable `PATH`.
6. Comprueba que la instalación funciona:

```powershell
rustup --version
rustc --version
cargo --version
```

Normalmente, los ejecutables de Rust se encuentran en:

```text
C:\Users\TU_USUARIO\.cargo\bin
```

`rustup` suele añadir esa ruta automáticamente al `PATH`. Si los comandos no aparecen, revisa la sección 6.

### 3.2. Actualizar una instalación existente

En PowerShell:

```powershell
rustup update
```

Eso actualiza las **toolchains instaladas** (por ejemplo, `stable`); no actualiza las bibliotecas del proyecto ni GCC. Para ver las versiones resultantes:

```powershell
rustup toolchain list
rustc --version
cargo --version
```

Si quieres consultar la ayuda de estos comandos:

```powershell
rustup help
cargo --help
rustc --help
```

### 3.3. Instalar Rust GNU de 64 bits

En PowerShell:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
```

Comprueba que ha quedado instalado:

```powershell
rustup toolchain list
```

No es obligatorio hacerlo globalmente predeterminado. Más adelante configuraremos **solo el directorio del proyecto** para usarlo.

---

## 4. Instalar MSYS2, GCC y herramientas de compilación

La documentación de `gmp-mpfr-sys` describe explícitamente el uso de **Rust GNU + MSYS2 MinGW 64-bit** en Windows. Es la combinación elegida aquí para evitar mezclar compiladores y bibliotecas de distintos entornos.

### 4.1. Instalar MSYS2

1. Entra en **https://www.msys2.org/**.
2. Descarga el instalador y acepta la ubicación habitual **`C:\msys64`**.
3. Abre el acceso del menú Inicio llamado **«MSYS2 MinGW 64-bit»** o **«MSYS2 MINGW64»**.
4. A partir de este apartado, los comandos `pacman` son para **esa terminal MSYS2**, no para PowerShell.

**Atención: MINGW64 no es lo mismo que UCRT64.** MSYS2 también ofrece una consola `UCRT64` con paquetes de nombre `mingw-w64-ucrt-x86_64-*`. Para seguir exactamente esta receta, abre **MINGW64** e instala los paquetes `mingw-w64-x86_64-*`. No mezcles los dos conjuntos de rutas o paquetes.

### 4.2. Actualizar MSYS2

En **MSYS2 MINGW64**:

```bash
pacman -Syu
```

Si solicita cerrar la ventana para terminar de actualizar componentes esenciales, acepta, vuelve a abrir **MSYS2 MINGW64** y repite:

```bash
pacman -Syu
```

Continúa hasta que no haya actualizaciones pendientes. MSYS2 se actualiza como un conjunto: no conviene hacer actualizaciones parciales.

### 4.3. Instalar las herramientas necesarias

En **MSYS2 MINGW64**:

```bash
pacman -S --needed diffutils m4 make mingw-w64-x86_64-gcc
```

Estos paquetes proporcionan:

- `gcc`: compilador de C compatible con el entorno MINGW64.
- `make`: automatización de compilaciones.
- `m4`: procesador de macros usado al construir bibliotecas GNU.
- `diffutils`: herramientas como `diff`, empleadas en procesos de construcción y pruebas.

Acepta las preguntas de `pacman` cuando correspondan.

### 4.4. Verificar GCC y las utilidades

En la **misma terminal MSYS2 MINGW64**:

```bash
gcc --version
make --version
m4 --version
diff --version
which gcc
```

La última orden debería apuntar a una ruta parecida a:

```text
/mingw64/bin/gcc
```

Si quieres actualizar GCC u otros paquetes de MSYS2 en el futuro, usa nuevamente `pacman -Syu` en MSYS2 (véase sección 14).

---

## 5. Seleccionar Rust GNU para este proyecto

Se puede cambiar la variante predeterminada para **todo el equipo**, pero suele ser más seguro asociar GNU únicamente al proyecto que lo necesita.

### Opción recomendada: configuración por directorio

Cuando ya estés en la carpeta que contiene `Cargo.toml` (la crearemos en la sección 7), ejecuta:

```powershell
rustup override set stable-x86_64-pc-windows-gnu
```

A partir de entonces, Cargo y `rustc`, al ejecutarse desde esa carpeta, utilizarán GNU:

```powershell
rustup show active-toolchain
rustc -vV
```

La línea `host:` debe indicar:

```text
host: x86_64-pc-windows-gnu
```

Puedes consultar las anulaciones de configuración mediante:

```powershell
rustup override list
```

Para retirar la anulación desde esa misma carpeta:

```powershell
rustup override unset
```

### Alternativa: hacer GNU predeterminado para todos los proyectos

Úsala solo si realmente deseas ese comportamiento:

```powershell
rustup default stable-x86_64-pc-windows-gnu
```

Si tu instalación MSVC está disponible, puedes volver a seleccionarla con:

```powershell
rustup default stable-x86_64-pc-windows-msvc
```

### Alternativa puntual: indicar la variante en cada comando

```powershell
cargo +stable-x86_64-pc-windows-gnu build --release
cargo +stable-x86_64-pc-windows-gnu run --release -- 1000
```

Con esta sintaxis, `+stable-x86_64-pc-windows-gnu` selecciona la herramienta para **esa única invocación**, sin modificar la configuración persistente.

---

## 6. Configurar los terminales y comprobar las rutas

### 6.1. Entender `PATH`

`PATH` contiene carpetas en las que el sistema busca comandos como `cargo`, `rustc` o `gcc`.

Rutas habituales:

```text
Rust (Windows):       C:\Users\TU_USUARIO\.cargo\bin
GCC (MSYS2 MINGW64):  C:\msys64\mingw64\bin
Herramientas MSYS2:   C:\msys64\usr\bin
```

**Recomendación:** compila el proyecto que usa `rug` desde **MSYS2 MINGW64**, ya que su entorno incluye las herramientas Unix que necesitan GMP/MPFR. Puedes gestionar el código y ejecutar el `.exe` final desde PowerShell. Agregar solo `gcc.exe` al `PATH` de PowerShell no garantiza que funcionen también los scripts de construcción de GMP/MPFR.

### 6.2. Si MSYS2 no reconoce `cargo` o `rustc`

Esto puede ocurrir aunque los comandos funcionen en PowerShell, porque MSYS2 puede utilizar un `PATH` restringido.

En **MSYS2 MINGW64**, añade temporalmente la ubicación de Rust, usando la carpeta real de tu usuario:

```bash
export PATH="$(cygpath -u "$USERPROFILE")/.cargo/bin:$PATH"
```

Comprueba:

```bash
which cargo
cargo --version
rustc --version
rustup show active-toolchain
```

Si `USERPROFILE` no estuviera disponible, sustituye la instrucción anterior por una ruta explícita (ajusta `TU_USUARIO`):

```bash
export PATH="/c/Users/TU_USUARIO/.cargo/bin:$PATH"
```

Esta asignación solo dura mientras esa terminal siga abierta. Una vez verifiques que funciona, puedes agregarla al archivo `~/.bashrc` de **MSYS2 MINGW64** si necesitas repetirla habitualmente. Ten en cuenta que el `~` de MSYS2 puede **no** ser la carpeta de usuario de Windows.

**Nunca** añadas a ciegas distintas instalaciones de GCC o directorios completos de varias variantes MSYS2 al `PATH`: mezclar `MINGW64`, `UCRT64` y herramientas MSVC puede causar conflictos.

### 6.3. Si quieres consultar GCC también desde PowerShell

Aunque para compilar con `rug` aconsejamos MSYS2, puedes hacer visible GCC en **la sesión actual** de PowerShell:

```powershell
$env:Path = "C:\msys64\mingw64\bin;$env:Path"
gcc --version
```

Esta modificación desaparece al cerrar PowerShell. No convierte PowerShell en una consola MSYS2 y, por sí sola, no instala `make`, `m4` ni configura las demás herramientas Unix.

### 6.4. Evitar espacios en la ubicación del proyecto

El sistema de construcción de `gmp-mpfr-sys` advierte de problemas con rutas que contienen espacios. Utiliza una carpeta sencilla, por ejemplo:

```text
C:\proyectos\rust\phi
```

Mejor evitar para este proyecto:

```text
C:\Mis proyectos Rust\Número áureo\
```

---

## 7. Crear un proyecto o recuperar el existente

### 7.1. Crear un proyecto nuevo con Cargo

En PowerShell (o, una vez preparado Rust, en MSYS2):

```powershell
cd C:\proyectos\rust
cargo new phi
cd phi
```

Cargo crea un proyecto ejecutable. Su estructura inicial será aproximadamente:

```text
phi/
├── .git/                (si Cargo ha inicializado Git)
├── .gitignore
├── Cargo.toml
└── src/
    └── main.rs
```

Archivos importantes:

- **`Cargo.toml`**: nombre del proyecto, edición de Rust y bibliotecas de las que depende.
- **`src/main.rs`**: punto de entrada para una aplicación ejecutable.
- **`Cargo.lock`**: se genera al resolver dependencias; registra versiones concretas.
- **`target/`**: aparece al compilar y contiene archivos intermedios y ejecutables.
- **`.gitignore`**: normalmente impide subir `target/` al repositorio.

Para comprobar la ubicación actual en PowerShell:

```powershell
Get-Location
Get-ChildItem
```

Para compilar después con MSYS2 MINGW64, entra en el mismo directorio usando barras Unix:

```bash
cd /c/proyectos/rust/phi
pwd
ls
```

### 7.2. Si la carpeta ya existe y tienes `Cargo.toml`

**No ejecutes `cargo new` otra vez.** Entra directamente al proyecto:

```powershell
cd C:\proyectos\rust\phi
Get-ChildItem
Get-Content .\Cargo.toml
```

Verifica que existe `src\main.rs`; si tu código está en un archivo `main.rs` independiente, colócalo en esa subcarpeta.

### 7.3. Si la carpeta existe pero aún no es un proyecto Cargo

En PowerShell:

```powershell
cd C:\proyectos\rust\phi
cargo init --bin
```

`cargo init --bin` inicializa una aplicación ejecutable en una carpeta existente. Si ya existe un `Cargo.toml`, no es necesario.

### 7.4. Configurar GNU para esta carpeta

Desde **la carpeta raíz del proyecto**:

```powershell
rustup override set stable-x86_64-pc-windows-gnu
rustup show active-toolchain
```

Así no modificas la compilación de tus otros proyectos Rust.

---

## 8. Instalar la dependencia `rug`

En Rust, normalmente no se instalan las bibliotecas «en Windows» para todos los proyectos. Se **añaden como dependencias a un proyecto Cargo**.

### 8.1. Instalación sencilla

En el directorio que contiene `Cargo.toml`:

```powershell
cargo add rug
```

Cargo añadirá la dependencia a `Cargo.toml`, por ejemplo:

```toml
[dependencies]
rug = "1.30"
```

La versión exacta que aparezca dependerá de la disponible al ejecutar el comando; **no hace falta escribirla a mano** si se utiliza `cargo add`.

Para el archivo original de φ, **no hacen falta** las dependencias `num-bigint` ni `num-traits`: el código utiliza `rug::Float`, no sus tipos o interfaces. Si las añadiste exclusivamente para este programa y no se usan en otra parte, las puedes retirar:

```powershell
cargo remove num-bigint
cargo remove num-traits
```

Si no existen, no ejecutes esas órdenes; no son necesarias.

### 8.2. Instalación más ajustada al cálculo de φ (recomendada)

`rug` ofrece distintas funciones opcionales. Este programa solo necesita números `Float` y la biblioteca estándar para convertir el resultado en `String`. Se pueden desactivar las funciones que no utilizamos:

```powershell
cargo add rug --no-default-features --features float,std
```

En este proyecto, la configuración resultante es exactamente:

```toml
[dependencies]
rug = { version = "1.30.0", default-features = false, features = ["float", "std"] }
```

**Elige una de las dos modalidades**, no ambas como entradas duplicadas. La segunda reduce las funciones solicitadas de `rug`; las bibliotecas nativas que sí hagan falta se administran automáticamente a través de sus dependencias. Si Cargo indica una versión más nueva compatible, conserva la que haya seleccionado.

### 8.3. ¿Hay que instalar GMP o MPFR por separado?

**Normalmente no.** La dependencia indirecta `gmp-mpfr-sys` puede compilar las bibliotecas nativas incluidas en su proceso de construcción. Para eso se han preparado GCC, `make`, `m4` y `diffutils` con MSYS2. No hace falta descargar manualmente cabeceras, archivos `.a` o DLL de GMP y MPFR para seguir esta receta.

La **primera compilación** puede dedicar bastante trabajo a GMP/MPFR; compilaciones posteriores suelen aprovechar cachés.

### 8.4. Ver qué dependencias utiliza el proyecto

```powershell
cargo tree
```

`cargo tree` muestra el árbol de paquetes Rust; `gmp-mpfr-sys` aparecerá como dependencia indirecta si la configuración requiere las bibliotecas nativas.

---

## 9. Preparar `src/main.rs` y entender el programa

### 9.1. Problemas del archivo `main.rs` original

El archivo recibido tiene estas líneas importantes:

```rust
use rug::Float;

fn main() {
    let mut precision: u32 = 1000;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        precision = args[1].parse().unwrap_or(1000);
    }

    // ... calcular y mostrar phi ...
}
```

Contiene dos detalles que conviene corregir:

1. **Falta `use std::env;`**, pues se llama a `env::args()`.
2. En `Float::with_val(precision, ...)`, `precision` se mide en **bits**, mientras que el texto del programa afirma que se están mostrando esa cantidad de **dígitos decimales**. Además, `to_string_radix(10, Some(n))` determina la cantidad de cifras de la representación decimal, no la precisión real almacenada en el número.

El programa original tampoco informa si el usuario escribe un parámetro no numérico: `unwrap_or(1000)` recupera silenciosamente el valor predeterminado.

### 9.2. Versión mejorada utilizada por el proyecto

**Contrato del programa:** `phi.exe [decimales]`.

- Sin argumento: calcula φ mostrando **1.000 cifras después del punto decimal**.
- Con `20`: muestra **20 cifras después del punto decimal**.
- Con `10000`: muestra **10.000 cifras después del punto decimal**.
- Si el argumento no es válido, informa del error y termina con código distinto de cero.
- Por seguridad, se establece un límite didáctico de **100.000 decimales**, modificable si lo necesitas.

El contenido actual de `src/main.rs` es el siguiente:

```rust
//! Cálculo del número áureo φ con precisión arbitraria.
//!
//! Fórmula: φ = (1 + √5) / 2
//!
//! Uso:
//!     phi.exe [decimales]
//!
//! Ejemplos:
//!     phi.exe
//!     phi.exe 20
//!     phi.exe 1000

use rug::Float;
use std::env;
use std::process;

const DECIMALES_PREDETERMINADOS: u32 = 1000;
const MAX_DECIMALES: u32 = 100_000;

fn main() {
    // Recoge los argumentos introducidos en la línea de comandos.
    // El elemento 0 es normalmente el nombre o ruta del ejecutable.
    let argumentos: Vec<String> = env::args().collect();

    // Este programa permite un único argumento opcional.
    if argumentos.len() > 2 {
        eprintln!("Uso: phi.exe [decimales]");
        process::exit(2);
    }

    let decimales: u32 = match argumentos.get(1) {
        None => DECIMALES_PREDETERMINADOS,
        Some(valor) => match valor.parse::<u32>() {
            Ok(n) if (1..=MAX_DECIMALES).contains(&n) => n,
            _ => {
                eprintln!(
                    "Error: introduce un entero entre 1 y {}.",
                    MAX_DECIMALES
                );
                eprintln!("Uso: phi.exe [decimales]");
                process::exit(2);
            }
        },
    };

    // MPFR configura la precisión en BITS, no en cifras decimales.
    // log2(10) ≈ 3,321928 bits por cifra decimal.
    // Se añade una cifra por la parte entera de φ y 64 bits de guarda
    // para reducir el error de redondeo de las operaciones intermedias.
    let bits: u32 =
        (((decimales as f64 + 1.0) * std::f64::consts::LOG2_10).ceil() as u32) + 64;

    // Construye números Float con la misma precisión de trabajo.
    let cinco = Float::with_val(bits, 5);
    let raiz_de_cinco = cinco.sqrt();

    let mut phi = Float::with_val(bits, 1);
    phi += &raiz_de_cinco;
    phi /= 2;

    // Una cifra significativa pertenece a la parte entera (el '1').
    // Se solicitan, por tanto, decimales + 1 cifras significativas.
    let resultado = phi.to_string_radix(10, Some((decimales + 1) as usize));

    println!("Fórmula: φ = (1 + √5) / 2");
    println!("Decimales solicitados: {}", decimales);
    println!("Precisión interna: {} bits", bits);
    println!("Resultado:");
    println!("{}", resultado);
}
```

> **Nota sobre la precisión:** el programa calcula con más bits de los estrictamente equivalentes a los decimales solicitados, para obtener una salida decimal fiable. Los 64 bits de guarda son un margen práctico, no una demostración matemática formal de redondeo correcto de todos los dígitos para cualquier magnitud. Para una garantía rigurosa de redondeo final se necesitaría un algoritmo de verificación adicional.

### 9.3. Guardar el archivo correctamente

Ubicación en el proyecto:

```text
C:\proyectos\rust\phi\src\main.rs
```

Puede editarse con Visual Studio Code, cualquier editor de texto o un IDE compatible con Rust. Se recomienda guardar en **UTF-8** para conservar los caracteres `φ` y `√`.

### 9.4. Qué hace cada bloque

| Fragmento | Explicación |
|---|---|
| `use rug::Float;` | Hace accesible `Float`, un tipo numérico de precisión arbitraria. |
| `use std::env;` | Permite leer los argumentos mediante `env::args()`. |
| `use std::process;` | Permite terminar con un código de error. |
| `const ...: u32` | Define valores constantes enteros sin signo de 32 bits. |
| `fn main()` | Función por la que empieza el programa. |
| `Vec<String>` | Vector de cadenas, una por argumento. |
| `argumentos.get(1)` | Consulta el primer argumento opcional sin acceder fuera del vector. |
| `match` | Distingue entre las distintas posibilidades de entrada. |
| `parse::<u32>()` | Intenta convertir texto en un entero positivo. |
| `eprintln!()` | Imprime mensajes de error por la salida estándar de errores. |
| `Float::with_val(bits, 5)` | Crea un número multiprecisión con el valor 5. |
| `sqrt()` | Calcula la raíz cuadrada. |
| `phi += &raiz_de_cinco` | Suma sin consumir innecesariamente el número al que se hace referencia. |
| `phi /= 2` | Divide entre dos. |
| `to_string_radix(10, ...)` | Convierte a una cadena decimal con la cantidad de cifras significativas solicitada. |
| `println!()` | Escribe el resultado en la consola. |

### 9.5. Comentarios correctos en Rust

```rust
// Comentario ordinario de una línea.

/* Comentario ordinario
   de varias líneas. */

/// Documentación de la función o elemento que viene justo después.
fn ejemplo() {}

//! Documentación del módulo o crate actual.
```

No utilices `/** ... */` como si fuera siempre un comentario ordinario al estilo C o Java: Rust interpreta `/** ... */` como **documentación externa**. Para explicar una operación dentro de `main`, prefiere `//` o `/* ... */`. Un comentario de documentación colocado donde no documenta un elemento puede generar `warning: unused doc comment`.

---

## 10. Comprender bits y decimales

**Este es el detalle matemático más importante del ejemplo.**

```rust
Float::with_val(1000, 5)
```

Lo anterior crea un número con **1.000 bits de precisión de mantisa**, no con 1.000 dígitos decimales.

La equivalencia aproximada es:

```text
dígitos decimales ≈ bits × log10(2)
                  ≈ bits × 0,30103

bits necesarios   ≈ dígitos decimales × log2(10)
                  ≈ dígitos decimales × 3,32193
```

Ejemplos de conversión aproximada **antes de sumar los bits de guarda**:

| Objetivo | Bits teóricos aproximados | Con 64 bits de guarda |
|---|---:|---:|
| 20 cifras decimales | 70 bits | 134 bits |
| 100 cifras decimales | 336 bits | 400 bits |
| 1.000 cifras decimales | 3.326 bits | 3.390 bits |
| 10.000 cifras decimales | 33.223 bits | 33.287 bits |

*Los valores incluyen una cifra significativa adicional por el `1` inicial de φ, y los bits teóricos se redondean hacia arriba.*

La instrucción:

```rust
phi.to_string_radix(10, Some(1000))
```

solicita **1.000 cifras significativas en base 10** para la representación, pero **no añade información numérica nueva**. Si `phi` se hubiera calculado con solo 100 bits, imprimir 1.000 cifras mostraría muchos dígitos sin fiabilidad matemática. La nueva versión coordina la precisión interna y el número de cifras de salida.

**`f64` frente a `rug::Float`:** `f64` normalmente proporciona alrededor de 15–17 dígitos decimales significativos; `rug::Float` admite mucha más precisión, a costa de más memoria y trabajo computacional. No significa «precisión infinita»: la precisión es configurable y finita en cada operación.

---

## 11. Compilar y ejecutar con Cargo

### 11.1. Usar MSYS2 MINGW64 para la compilación

Abre **MSYS2 MINGW64** y ejecuta:

```bash
# Solo si MSYS2 no encuentra 'cargo':
export PATH="$(cygpath -u "$USERPROFILE")/.cargo/bin:$PATH"

# Entra en el directorio con Cargo.toml:
cd /c/proyectos/rust/phi

# Comprueba que se está utilizando Rust GNU:
rustup show active-toolchain
rustc -vV

# Comprueba GCC:
gcc --version
```

Si no habías configurado la herramienta GNU como anulación local, hazlo ahora:

```bash
rustup override set stable-x86_64-pc-windows-gnu
```

### 11.2. Primera compilación: depuración

```bash
cargo build
```

De forma predeterminada, Cargo construye una versión de desarrollo en:

```text
target/debug/phi.exe
```

**Si se compila `rug` por primera vez, el proceso puede ser bastante más pesado**: las dependencias nativas GMP y MPFR pueden tener que compilarse. No cierres la terminal mientras veas actividad de compilación.

### 11.3. Compilar la versión optimizada (Release)

```bash
cargo build --release
```

El ejecutable optimizado aparecerá en:

```text
target/release/phi.exe
```

En PowerShell, la ruta equivalente es:

```powershell
.\target\release\phi.exe
```

**`release` no significa que el ejecutable se haya instalado globalmente.** Solo indica que Cargo lo ha generado con el perfil optimizado.

### 11.4. Compilar y ejecutar en un mismo comando

En MSYS2 MINGW64:

```bash
cargo run
cargo run -- 20
cargo run --release -- 1000
```

En PowerShell se utilizan las mismas órdenes `cargo run` si el entorno de compilación está configurado correctamente, aunque **para este proyecto la compilación desde MINGW64 es la ruta aconsejada**.

**Explicación del separador `--`:**

```text
cargo run --release -- 1000
          └────┘ └┘ └─┘
        perfil   |  argumento del programa
              separador
```

- Antes de `--` van los argumentos de **Cargo**.
- Después de `--` van los argumentos de **tu programa**.
- La sintaxis `cargo run --getbalance`, usada como ejemplo en otros proyectos, **no es equivalente** a pasar argumentos: la forma normal es `cargo run -- --getbalance`.

### 11.5. Revisiones adicionales

```bash
cargo check
cargo fmt
cargo clippy
```

- `cargo check`: comprueba el código sin generar el ejecutable final completo.
- `cargo fmt`: aplica formato estándar al código Rust.
- `cargo clippy`: busca posibles errores y mejoras de estilo.

Si no tienes componentes de formato o análisis instalados:

```bash
rustup component add rustfmt clippy
```

---

## 12. Ejecutar directamente el `.exe` con argumentos

Una vez compilado el programa, **no necesitas utilizar `cargo run` para ejecutarlo**. Puedes invocar el `.exe` directamente y pasar los parámetros.

### 12.1. Desde PowerShell, en la carpeta del proyecto

```powershell
cd C:\proyectos\rust\phi

# 1.000 decimales (valor predeterminado):
.\target\release\phi.exe

# 20 decimales:
.\target\release\phi.exe 20

# 100 decimales:
.\target\release\phi.exe 100

# 1.000 decimales:
.\target\release\phi.exe 1000

# 10.000 decimales:
.\target\release\phi.exe 10000
```

**Muy importante:** PowerShell no ejecuta automáticamente programas del directorio actual por su nombre. Ante una ruta relativa hay que colocar `.\` al principio, como en `.\target\release\phi.exe`.

### 12.2. Desde MSYS2 MINGW64

```bash
./target/release/phi.exe 1000
```

En la terminal Bash de MSYS2 las rutas usan `/` y el ejecutable local se invoca con `./`.

### 12.3. Ejecutar desde otra carpeta utilizando una ruta absoluta

En PowerShell:

```powershell
& "C:\proyectos\rust\phi\target\release\phi.exe" 1000
```

El operador `&` es útil para ejecutar rutas entre comillas, especialmente si contienen espacios. Aunque el **proyecto al compilar** se recomienda sin espacios, puedes guardar el `.exe` compilado donde corresponda.

### 12.4. Comportamiento previsto de los parámetros

| Instrucción | Comportamiento |
|---|---|
| `...exe` | Utiliza 1.000 decimales. |
| `...exe 20` | Calcula 20 decimales. |
| `...exe 1000` | Calcula 1.000 decimales. |
| `...exe 10000` | Calcula 10.000 decimales. |
| `...exe abc` | Muestra error: no es un entero. |
| `...exe -1` | Muestra error: valor fuera del intervalo. |
| `...exe 0` | Muestra error: el mínimo didáctico es 1. |
| `...exe 1000 otro` | Muestra el modo de empleo: hay demasiados argumentos. |

### 12.5. Ejemplo de salida (20 decimales)

```text
Fórmula: φ = (1 + √5) / 2
Decimales solicitados: 20
Precisión interna: 134 bits
Resultado:
1.61803398874989484820
```

La cifra de bits mostrada corresponde al cálculo de esta versión; no se debe confundir con los decimales impresos. La representación utiliza el **punto** como separador decimal, según el formato numérico elegido por `to_string_radix`.

### 12.6. Consultar el código de salida

En PowerShell, tras ejecutar el programa:

```powershell
$LASTEXITCODE
```

- `0`: ejecución terminada con éxito.
- `2`: parámetro erróneo, según la versión actual de `main.rs`.

---

## 13. Guardar los resultados y distribuir el ejecutable

### 13.1. Guardar la salida en un archivo de texto

En **PowerShell**:

```powershell
.\target\release\phi.exe 1000 | Out-File -FilePath .\phi_1000.txt -Encoding utf8
```

Puedes abrir el resultado:

```powershell
Get-Content .\phi_1000.txt
```

O inspeccionar su existencia:

```powershell
Get-Item .\phi_1000.txt
```

En **MSYS2 MINGW64**:

```bash
./target/release/phi.exe 1000 > phi_1000.txt
```

Los ejemplos guardan el **texto completo de la consola**, incluidos encabezados y número. Si quisieras generar un archivo que contenga exclusivamente los dígitos de φ, habría que modificar el programa para enviar los encabezados a `stderr` o añadir un modo de salida especial.

### 13.2. Copiar el ejecutable a otra carpeta

Desde PowerShell:

```powershell
New-Item -ItemType Directory -Force .\distribucion | Out-Null
Copy-Item .\target\release\phi.exe .\distribucion\
.\distribucion\phi.exe 100
```

La carpeta `distribucion` no es obligatoria; es simplemente un ejemplo.

### 13.3. ¿Se puede ejecutar sin tener Rust instalado?

**Sí, normalmente.** Rust y Cargo son herramientas para construir el programa, no requisitos generales para ejecutarlo. Pero al distribuir un `.exe` construido con GNU hay que comprobar si depende de determinadas **DLL de tiempo de ejecución**. Si otro equipo muestra mensajes sobre `libgcc_s_seh-1.dll`, `libwinpthread-1.dll` u otra DLL, faltan dependencias de ejecución y será necesario empaquetarlas o utilizar el entorno de ejecución correspondiente; no significa que haga falta instalar el compilador Rust.

Para identificar dependencias en el entorno MinGW se puede usar una herramienta como `objdump` cuando esté instalada. No presupongas que cualquier `.exe` es completamente autónomo sin probarlo en un equipo limpio.

### 13.4. ¿Hay que conservar `target/`?

- Conserva **`src/`**, **`Cargo.toml`** y **`Cargo.lock`** para poder reconstruir el programa.
- Conserva el **`.exe`** si deseas ejecutarlo sin recompilar.
- La carpeta **`target/`** puede eliminarse y regenerarse, pero perderás los objetos compilados y tendrás que reconstruirlos.

Para limpiar artefactos de compilación:

```powershell
cargo clean
```

**Precaución:** `cargo clean` puede obligar a recompilar también dependencias pesadas.

---

## 14. Actualizar herramientas y dependencias

**Cada herramienta se actualiza por separado.** No existe un único comando que actualice correctamente Rust, GCC/MSYS2 y las bibliotecas del proyecto a la vez.

### 14.1. Rust, Cargo y componentes de las toolchains

En PowerShell:

```powershell
rustup update
rustup toolchain list
rustc --version
cargo --version
```

Si instalaste más de una variante, `rustup update` contempla las instaladas según corresponda.

### 14.2. GCC, `make`, `m4` y MSYS2

En **MSYS2 MINGW64**:

```bash
pacman -Syu
```

Cierra y reabre la ventana si se solicita y ejecuta el comando nuevamente. Luego verifica:

```bash
gcc --version
make --version
m4 --version
```

**No utilices `rustup update` para actualizar GCC**: GCC no forma parte de rustup.

### 14.3. Dependencias Rust del proyecto (`rug` y dependencias indirectas)

Desde la carpeta que contiene `Cargo.toml`:

```powershell
cargo update
```

`cargo update` actualiza las versiones registradas en `Cargo.lock` **dentro de los intervalos permitidos por `Cargo.toml`**. No cambia automáticamente cualquier versión mayor ni actualiza el compilador.

Para actualizar selectivamente un paquete concreto dentro de sus restricciones:

```powershell
cargo update rug
```

Si quieres modificar las características con las que se utiliza `rug`:

```powershell
cargo add rug --no-default-features --features float,std
```

### 14.4. Comprobar el proyecto tras actualizar

En MSYS2 MINGW64:

```bash
cargo fmt
cargo check
cargo clippy
cargo build --release
./target/release/phi.exe 20
```

Es recomendable revisar los cambios en `Cargo.lock` antes de confirmarlos en Git. Para un programa ejecutable, normalmente conviene **versionar `Cargo.lock`** y no subir `target/`.

---

## 15. Errores frecuentes y soluciones

### «`gcc` no se reconoce como comando»

**Causa habitual:** GCC no está instalado o PowerShell no tiene su carpeta en `PATH`.

**Solución recomendada:** instala MSYS2, abre **MINGW64** e instala `mingw-w64-x86_64-gcc`; verifica con `gcc --version` **dentro de esa terminal**. No confundas «no disponible en PowerShell» con «no instalado en MSYS2».

### «`cargo` no se reconoce» en MSYS2

**Causa:** MSYS2 no está viendo `C:\Users\...\.cargo\bin`.

**Solución:**

```bash
export PATH="$(cygpath -u "$USERPROFILE")/.cargo/bin:$PATH"
cargo --version
```

### La línea `host:` de `rustc -vV` termina en `-msvc`

**Causa:** no se está usando la toolchain GNU.

**Solución:**

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup override set stable-x86_64-pc-windows-gnu
rustc -vV
```

Ejecuta `rustup override set` desde el directorio **del proyecto**.

### `use of unresolved module or unlinked crate 'rug'`

**Causa:** el proyecto no tiene declarada la dependencia o estás compilando directamente un `.rs` aislado con `rustc` sin la configuración de Cargo.

**Solución:** sitúate en la raíz del proyecto y ejecuta:

```bash
cargo add rug --no-default-features --features float,std
cargo build
```

No uses `rustc src/main.rs` como método normal de compilación de este programa; Cargo gestiona las dependencias y el enlazado.

### `failed to resolve: use of unresolved module ... env`

**Causa:** falta la importación de `std::env` en el código original.

**Solución:** incluye:

```rust
use std::env;
```

O invoca explícitamente `std::env::args()`.

### Error de `make`, `m4`, `diff`, `sh` o `gcc` al compilar `gmp-mpfr-sys`

**Causa posible:** herramientas ausentes, terminal equivocada, rutas mezcladas o build GNU ejecutado fuera del entorno esperado.

**Solución:** en **MSYS2 MINGW64** instala y comprueba:

```bash
pacman -Syu
pacman -S --needed diffutils m4 make mingw-w64-x86_64-gcc
which gcc
which make
which m4
which diff
```

Asegúrate de tener activa `stable-x86_64-pc-windows-gnu` y de trabajar en una carpeta sin espacios.

### Error por rutas con espacios

**Causa:** el sistema de compilación nativo de `gmp-mpfr-sys` no admite ciertas rutas con espacios.

**Solución:** mueve el proyecto a `C:\proyectos\rust\phi`. Si el problema proviene también de carpetas internas del usuario, puede ser necesario establecer rutas personalizadas sin espacios para las cachés y/o el directorio de Cargo; es una configuración avanzada que solo se recomienda cuando se identifica esa causa.

### `linker 'link.exe' not found`

**Causa frecuente:** se está intentando compilar para MSVC sin sus herramientas de enlace instaladas.

**Solución en este proyecto:** selecciona la toolchain GNU y compila desde MSYS2 MINGW64. Si necesitases realmente la variante MSVC, deberías instalar las herramientas de C++ correspondientes de Visual Studio.

### `linking with ... failed` o bibliotecas incompatibles

**Causa posible:** se han mezclado versiones GNU/MSVC o rutas de `MINGW64` y `UCRT64`.

**Solución:** comprueba `rustc -vV`, `which gcc` y el `PATH` utilizado, deja solo las herramientas compatibles y vuelve a compilar. Una vez corregido el entorno, podría ser necesario ejecutar `cargo clean` y `cargo build --release`.

### Cargo no encuentra `Cargo.toml`

**Causa:** estás en una carpeta que no contiene el proyecto.

**Solución:** entra en el directorio raíz, por ejemplo:

```powershell
cd C:\proyectos\rust\phi
Get-ChildItem Cargo.toml
```

### En PowerShell, escribir `phi.exe 1000` no lo ejecuta

**Causa:** PowerShell no busca automáticamente ejecutables en el directorio actual.

**Solución:** utiliza la ruta relativa con `.\`:

```powershell
.\target\release\phi.exe 1000
```

### `cargo run --release 1000` no pasa bien el parámetro

**Causa:** falta el separador `--` entre argumentos de Cargo y argumentos del programa.

**Solución:**

```bash
cargo run --release -- 1000
```

Al ejecutar el `.exe` directamente no necesitas ese separador.

### El programa muestra 1.000 cifras pero muchas son incorrectas

**Causa:** confusión entre bits de precisión interna y cifras decimales de salida.

**Solución:** utiliza la versión mejorada de `main.rs`, que convierte los decimales solicitados en bits y añade un margen de guarda. Comprueba siempre que la precisión de cálculo sea suficiente para las cifras que imprimes.

### `warning: unused doc comment`

**Causa:** se ha puesto `/** ... */` (documentación) donde debía aparecer un comentario ordinario.

**Solución:** utiliza `// ...` o `/* ... */` para describir instrucciones dentro de `main`.

### El `.exe` funciona en el ordenador de desarrollo, pero falta una DLL en otro equipo

**Causa:** dependencia de una biblioteca de ejecución no disponible en el ordenador de destino.

**Solución:** identifica la DLL concreta y prepara una distribución compatible con tu ejecutable; prueba el resultado en un entorno limpio. No copies DLL de fuentes desconocidas.

### El resultado no muestra correctamente `φ` o `√`

**Causa posible:** consola o editor configurados con una codificación que no representa esos caracteres.

**Solución:** guarda los archivos Rust en UTF-8, utiliza una consola moderna y, para guardar la salida desde PowerShell, usa `Out-File -Encoding utf8`.

---

## 16. Conceptos básicos de Rust y Cargo

### 16.1. `let` y `let mut`

```rust
let decimales: u32 = 1000; // Inmutable.
let mut contador: u32 = 0;  // Se puede modificar.
contador += 1;
```

Rust hace que las variables sean inmutables de forma predeterminada; necesitas `mut` si pretendes asignarles un valor nuevo.

### 16.2. `String`, `Vec<String>` y argumentos

```rust
let argumentos: Vec<String> = std::env::args().collect();
```

- Un `String` almacena texto.
- Un `Vec<String>` almacena una secuencia de cadenas.
- `argumentos[0]` suele corresponder al nombre/ruta del ejecutable.
- `argumentos[1]`, si existe, es el primer parámetro escrito por el usuario.
- Antes de acceder con índices `[]`, hay que comprobar que están dentro de los límites; `get(1)` evita el acceso fuera de rango y devuelve una opción.

### 16.3. `Option` y `Result`

Una operación puede no disponer de un valor (`Option`) o fallar (`Result`).

```rust
let texto: Option<&String> = argumentos.get(1);
let numero: Result<u32, _> = "1000".parse::<u32>();
```

El programa actual utiliza `match` para decidir qué hacer cuando falta un argumento o cuando la conversión es errónea. Esto es preferible a provocar un fallo con `unwrap()` en entradas de usuario sin verificar.

### 16.4. Tipos numéricos

```rust
let a: u32 = 1000;  // Entero de 32 bits sin signo.
let b: u64 = 1000;  // Entero de 64 bits sin signo.
let c: f64 = 1.618; // Coma flotante de 64 bits.

let d = rug::Float::with_val(4096, 5); // 4.096 bits de precisión.
```

Para enteros extremadamente grandes se puede utilizar `rug::Integer`; para valores con parte decimal y miles de cifras, `rug::Float`.

### 16.5. ¿Cuándo utilizar `cargo build`, `cargo run` y `rustc`?

- **`cargo new` / `cargo init`**: crear el esqueleto del proyecto.
- **`cargo add`**: agregar bibliotecas.
- **`cargo build`**: compilar.
- **`cargo run -- ...`**: compilar si hace falta y ejecutar.
- **`cargo build --release`**: crear la versión optimizada.
- **`rustc`**: compilador directo; muy útil para ejemplos sin dependencias externas o casos especializados, pero **no sustituye a Cargo** en proyectos que utilizan bibliotecas como `rug`.

---

## 17. Referencia rápida de comandos

### 17.1. Instalación y actualización (PowerShell)

```powershell
# Verificar Rust:
rustup --version
rustc --version
cargo --version

# Actualizar Rust:
rustup update

# Instalar la variante GNU de 64 bits:
rustup toolchain install stable-x86_64-pc-windows-gnu

# Comprobar las toolchains:
rustup toolchain list
```

### 17.2. Preparar GCC (MSYS2 MINGW64)

```bash
pacman -Syu
# Si hay que cerrar y reabrir MSYS2, repetir pacman -Syu.
pacman -S --needed diffutils m4 make mingw-w64-x86_64-gcc
gcc --version
```

### 17.3. Crear el proyecto (PowerShell)

```powershell
cd C:\proyectos\rust
cargo new phi
cd phi
rustup override set stable-x86_64-pc-windows-gnu
cargo add rug --no-default-features --features float,std
```

**Después:** pegar el código de la sección 9.2 en `src\main.rs`.

### 17.4. Compilar (MSYS2 MINGW64)

```bash
export PATH="$(cygpath -u "$USERPROFILE")/.cargo/bin:$PATH"
cd /c/proyectos/rust/phi
rustup show active-toolchain
gcc --version
cargo build --release
```

### 17.5. Ejecutar el `.exe` (PowerShell)

```powershell
cd C:\proyectos\rust\phi
.\target\release\phi.exe 20
.\target\release\phi.exe 1000
.\target\release\phi.exe 10000
```

### 17.6. Ejecutar con Cargo (MSYS2 MINGW64)

```bash
cargo run --release -- 20
cargo run --release -- 1000
```

### 17.7. Actualizar bibliotecas del proyecto

```bash
cargo update
cargo build --release
```

---

## 18. Documentación oficial y referencias

Estas páginas permiten contrastar los procedimientos cuando aparezcan nuevas versiones de Rust, Cargo, MSYS2 o `rug`:

1. **Instalación oficial de Rust:** https://www.rust-lang.org/tools/install
2. **Manual de rustup para Windows y toolchains GNU/MSVC:** https://rust-lang.github.io/rustup/installation/windows.html
3. **Manual de selección de toolchains (overrides):** https://rust-lang.github.io/rustup/overrides.html
4. **Guía oficial de Cargo (crear y compilar proyectos):** https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html
5. **Comando `cargo add`:** https://doc.rust-lang.org/cargo/commands/cargo-add.html
6. **Comando `cargo update`:** https://doc.rust-lang.org/cargo/commands/cargo-update.html
7. **Página oficial de MSYS2:** https://www.msys2.org/
8. **Actualización de MSYS2:** https://www.msys2.org/docs/updating/
9. **GCC mediante MSYS2 / MinGW-w64:** https://www.mingw-w64.org/getting-started/msys2/
10. **Biblioteca `rug` y sus funciones opcionales:** https://docs.rs/rug/latest/rug/
11. **Tipo `Float` y `to_string_radix`:** https://docs.rs/rug/latest/rug/struct.Float.html
12. **Construcción de `gmp-mpfr-sys` en Windows:** https://docs.rs/gmp-mpfr-sys/latest/gmp_mpfr_sys/
13. **Libro de Rust (para seguir aprendiendo):** https://doc.rust-lang.org/book/

---

### Resultado final esperado

Al terminar tendrás:

```text
C:\proyectos\rust\phi\
├── Cargo.toml
├── Cargo.lock
├── src\
│   └── main.rs
└── target\
    ├── debug\
    │   └── phi.exe
    └── release\
        └── phi.exe
```

Y podrás ejecutar directamente desde PowerShell:

```powershell
.\target\release\phi.exe 1000
```

**Conclusión:** `rustup` mantiene Rust, Cargo administra el proyecto y sus crates, MSYS2/GCC permite construir las bibliotecas nativas y el `.exe` resultante acepta la precisión decimal mediante un argumento opcional. Para este cálculo no es necesario programar directamente en C ni invocar GCC a mano.

