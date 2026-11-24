//! Ejercicio 10 — Lifetimes.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones y métodos
//! marcados con `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e10-lifetimes
//! ```

/// Devuelve la cadena más larga de las dos.
///
/// Si miden lo mismo, devuelve `a`. El lifetime `'a` indica que el resultado
/// vive al menos tanto como la más corta de las dos entradas.
pub fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.chars().count() >= b.chars().count() {
        a
    } else {
        b
    }
}

/// Devuelve la primera palabra del texto, o `""` si está vacío.
///
/// Aquí no hace falta anotar lifetimes: la elisión permite que el resultado
/// herede el lifetime del único parámetro de entrada.
pub fn primera_palabra(texto: &str) -> &str {
    texto.split_whitespace().next().unwrap_or("")
}

/// Vista de una porción de un texto prestado.
///
/// El struct guarda una referencia, así que necesita un parámetro de lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Extracto<'a> {
    texto: &'a str,
    inicio: usize,
    fin: usize,
}

impl<'a> Extracto<'a> {
    /// Crea un extracto si los índices son válidos.
    ///
    /// Devuelve `None` si `inicio > fin`, si `fin` supera la longitud del texto
    /// o si los índices no caen en límites de carácter UTF-8.
    pub fn new(texto: &'a str, inicio: usize, fin: usize) -> Option<Self> {
        texto.get(inicio..fin)?;
        Some(Self { texto, inicio, fin })
    }

    /// Devuelve la porción seleccionada.
    pub fn contenido(&self) -> &'a str {
        self.texto.get(self.inicio..self.fin).unwrap()
    }

    /// Devuelve el texto completo del que se tomó el extracto.
    pub fn texto_completo(&self) -> &'a str {
        self.texto
    }
}

/// Busca la primera cadena que contenga `patron`.
///
/// La referencia devuelta apunta a uno de los elementos de `textos`, por eso el
/// lifetime del resultado es el de la colección, no el de `patron`.
pub fn buscar<'a>(textos: &'a [String], patron: &str) -> Option<&'a str> {
    textos
        .iter()
        .find(|t| t.contains(patron))
        .map(|t| t.as_str())
}
