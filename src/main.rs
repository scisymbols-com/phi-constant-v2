//! Versión 2.0
//! Cálculo del número áureo φ con precisión arbitraria.
//!
//! Fórmula: φ = (1 + √5) / 2
//!
//! Compilación:
//!     cargo +stable-x86_64-pc-windows-gnu build --release
//!
//! Calcular SHA-256:
//!     sha256sum ./target/release/phi.exe
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