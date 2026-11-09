//! Ejercicio 01 — Ownership (propiedad).
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e01-ownership
//! ```

/// Devuelve la longitud en bytes del `String` recibido.
///
/// Recibe el `String` por valor: la función pasa a ser dueña de él y lo libera
/// al terminar. El llamador ya no puede usarlo después de esta llamada.
pub fn longitud(texto: String) -> usize {
    todo!("Implementa longitud")
}

/// Suma dos enteros.
///
/// Los enteros implementan `Copy`, así que se copian al pasarlos. El llamador
/// conserva sus variables originales.
pub fn sumar_copias(a: i32, b: i32) -> i32 {
    todo!("Implementa sumar_copias")
}

/// Devuelve el vector recibido y cuántos elementos tiene.
///
/// Sirve para observar que el vector se mueve a la función y luego se devuelve:
/// la propiedad vuelve al llamador a través del valor de retorno.
pub fn mover_y_contar(numeros: Vec<i32>) -> (Vec<i32>, usize) {
    todo!("Implementa mover_y_contar")
}

/// Concatena dos `String` tomando propiedad de ambos.
///
/// Por ejemplo, `concatenar("Hola, ".to_string(), "Rust".to_string())` debe
/// devolver `"Hola, Rust"`.
pub fn concatenar(izquierda: String, derecha: String) -> String {
    todo!("Implementa concatenar")
}

/// Devuelve la primera palabra de `texto` como un `String` nuevo.
///
/// Si no hay palabras, devuelve un `String` vacío. Fíjate en que `texto` se
/// presta (`&str`) y el resultado es un valor propio (`String`).
pub fn primera_palabra(texto: &str) -> String {
    todo!("Implementa primera_palabra")
}

/// Crea un `Vec<String>` con las palabras de `texto` y devuelve cuántas hay.
///
/// Demuestra que un valor local puede moverse a otra función: el vector se crea
/// aquí, se pasa por valor a una función auxiliar y esta lo consume.
pub fn contar_palabras(texto: &str) -> usize {
    todo!("Implementa contar_palabras")
}
