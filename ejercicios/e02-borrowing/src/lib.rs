//! Ejercicio 02 — Borrowing (préstamos).
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e02-borrowing
//! ```

/// Cuenta las vocales (a, e, i, o, u) sin distinguir mayúsculas ni tildes.
///
/// Recibe `&str`: solo presta el texto, no toma propiedad de él.
pub fn contar_vocales(texto: &str) -> usize {
    todo!("Implementa contar_vocales")
}

/// Normaliza un nombre en el lugar: quita espacios de los extremos, pasa a
/// minúsculas y convierte los grupos de espacios intermedios en un solo `-`.
///
/// Por ejemplo, `"  Hola   Mundo  "` pasa a ser `"hola-mundo"`.
///
/// Recibe `&mut String` porque modifica el valor del llamador sin quedarse con
/// la propiedad.
pub fn normalizar(nombre: &mut String) {
    todo!("Implementa normalizar")
}

/// Suma los elementos de un slice.
///
/// Un `&[i32]` sirve tanto para `Vec<i32>` como para arrays y para slices.
pub fn sumar_slice(numeros: &[i32]) -> i32 {
    todo!("Implementa sumar_slice")
}

/// Sustituye los números negativos por cero, modificando el slice original.
pub fn reemplazar_negativos(numeros: &mut [i32]) {
    todo!("Implementa reemplazar_negativos")
}

/// Devuelve una referencia al primer elemento, o `None` si el slice está vacío.
///
/// El `Option<&i32>` demuestra que una referencia prestada puede viajar dentro
/// de un `Option`.
pub fn primer_elemento(numeros: &[i32]) -> Option<&i32> {
    todo!("Implementa primer_elemento")
}

/// Divide un texto en palabras y devuelve préstamos a esas palabras.
///
/// Las `&str` del resultado apuntan al `texto` original; el llamador debe
/// mantenerlo vivo mientras use el vector. Es un ejemplo de elisión de
/// lifetimes: el compilador entiende que la salida vive lo mismo que la entrada.
pub fn dividir_en_palabras(texto: &str) -> Vec<&str> {
    todo!("Implementa dividir_en_palabras")
}

/// Suma las longitudes de todos los fragmentos.
pub fn longitud_total(fragmentos: &[&str]) -> usize {
    todo!("Implementa longitud_total")
}
