//! Ejercicio 07 — Colecciones e iteradores.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e07-collections-iterators
//! ```

use std::collections::HashMap;

/// Cuenta cuántas veces aparece cada palabra.
///
/// Debe pasar todo a minúsculas y quitar la puntuación de los extremos de cada
/// palabra. Las palabras vacías se ignoran. Por ejemplo, `"Hola, mundo. Hola!"`
/// produce `{"hola": 2, "mundo": 1}`.
pub fn frecuencias(texto: &str) -> HashMap<String, usize> {
    todo!("Implementa frecuencias")
}

/// Filtra los números pares y devuelve sus cuadrados, conservando el orden.
pub fn pares_al_cuadrado(numeros: &[i32]) -> Vec<i32> {
    todo!("Implementa pares_al_cuadrado")
}

/// Suma los números pares.
pub fn suma_de_pares(numeros: &[i32]) -> i32 {
    todo!("Implementa suma_de_pares")
}

/// Devuelve los nombres cuya longitud en caracteres sea al menos `minimo`.
pub fn nombres_largos(nombres: &[String], minimo: usize) -> Vec<String> {
    todo!("Implementa nombres_largos")
}

/// Agrupa palabras por su longitud en caracteres, conservando el orden de
/// aparición dentro de cada grupo.
pub fn agrupar_por_longitud(palabras: &[String]) -> HashMap<usize, Vec<String>> {
    todo!("Implementa agrupar_por_longitud")
}

/// Devuelve la palabra más frecuente y su conteo.
///
/// Si hay empate, devuelve la que sea menor lexicográficamente. Devuelve `None`
/// si el texto no tiene palabras. Usa `frecuencias` para no duplicar lógica.
pub fn palabra_mas_frecuente(texto: &str) -> Option<(String, usize)> {
    todo!("Implementa palabra_mas_frecuente")
}

/// Aplica una función dos veces: `f(f(valor))`.
pub fn aplicar_dos_veces<F>(f: F, valor: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    todo!("Implementa aplicar_dos_veces")
}

/// Acumula los valores empezando por `inicial`, usando la función `f`.
///
/// Equivale a `f(f(f(inicial, v0), v1), v2)`.
pub fn acumular<F>(valores: &[i32], inicial: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    todo!("Implementa acumular")
}
