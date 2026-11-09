//! Ejercicio 04 — Pattern matching.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e04-pattern-matching
//! ```

/// Instrucciones para una tortuga imaginaria.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruccion {
    /// Avanza `n` pasos.
    Avanzar(i32),
    /// Gira los grados indicados; no avanza.
    Girar { grados: i32 },
    /// Se detiene.
    Detener,
    /// Repite una instrucción `veces` veces.
    Repetir {
        veces: u32,
        instruccion: Box<Instruccion>,
    },
}

/// Calcula cuántos pasos avanza una instrucción.
///
/// - `Avanzar(n)` avanza `n`.
/// - `Girar { .. }` y `Detener` no avanzan.
/// - `Repetir { veces, instruccion }` avanza `veces` por lo que avance la
///   instrucción interna (puede estar anidada).
pub fn ejecutar(instruccion: &Instruccion) -> i32 {
    todo!("Implementa ejecutar")
}

/// Clasifica un entero como `"negativo"`, `"cero"`, `"positivo par"` o
/// `"positivo impar"`.
pub fn clasificar(n: i32) -> &'static str {
    todo!("Implementa clasificar")
}

/// Describe un `Option<i32>`:
///
/// - `None` → `"sin valor"`.
/// - `Some(n)` con `n > 0` → `"positivo: {n}"`.
/// - `Some(0)` → `"cero"`.
/// - `Some(n)` con `n < 0` → `"negativo: {n}"`.
pub fn describir_opcion(valor: Option<i32>) -> String {
    todo!("Implementa describir_opcion")
}

/// Suma solo los valores presentes de una lista de opciones.
pub fn sumar_opciones(valores: &[Option<i32>]) -> i32 {
    todo!("Implementa sumar_opciones")
}

/// Devuelve la primera palabra del texto que tenga al menos `minimo`
/// caracteres, o `None` si no hay ninguna.
pub fn primera_palabra_larga(texto: &str, minimo: usize) -> Option<&str> {
    todo!("Implementa primera_palabra_larga")
}

/// Describe una pareja `(número, etiqueta)` con estas reglas:
///
/// - `(0, _)` → `"cero"`.
/// - `(n, "importante")` → `"importante: {n}"`.
/// - `(n, etiqueta)` → `"{n}: {etiqueta}"`.
pub fn describir_pareja(pareja: (i32, &str)) -> String {
    todo!("Implementa describir_pareja")
}
