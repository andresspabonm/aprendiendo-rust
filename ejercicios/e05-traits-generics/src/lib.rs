//! Ejercicio 05 — Traits y genéricos.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa los métodos marcados con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e05-traits-generics
//! ```

use std::fmt;

/// Comportamiento común de las figuras.
pub trait Figura {
    /// Área de la figura.
    fn area(&self) -> f64;

    /// Nombre de la figura.
    fn nombre(&self) -> &'static str;

    /// Descripción con dos decimales.
    ///
    /// Este método tiene una implementación por defecto: los tipos que
    /// implementan `Figura` no necesitan escribirlo.
    fn descripcion(&self) -> String {
        format!("{} con área {:.2}", self.nombre(), self.area())
    }
}

/// Círculo.
#[derive(Debug, Clone, PartialEq)]
pub struct Circulo {
    pub radio: f64,
}

/// Cuadrado.
#[derive(Debug, Clone, PartialEq)]
pub struct Cuadrado {
    pub lado: f64,
}

impl Figura for Circulo {
    fn area(&self) -> f64 {
        todo!("Implementa Figura::area para Circulo")
    }

    fn nombre(&self) -> &'static str {
        todo!("Implementa Figura::nombre para Circulo")
    }
}

impl Figura for Cuadrado {
    fn area(&self) -> f64 {
        todo!("Implementa Figura::area para Cuadrado")
    }

    fn nombre(&self) -> &'static str {
        todo!("Implementa Figura::nombre para Cuadrado")
    }
}

/// Suma las áreas de cualquier colección de figuras del mismo tipo.
pub fn area_total<T: Figura>(figuras: &[T]) -> f64 {
    todo!("Implementa area_total")
}

/// Devuelve la figura con mayor área, o `None` si la colección está vacía.
///
/// Si hay empate, devuelve la primera.
pub fn mayor_area<T: Figura>(figuras: &[T]) -> Option<&T> {
    todo!("Implementa mayor_area")
}

/// Duplica un valor clonándolo.
pub fn duplicar<T: Clone>(valor: T) -> (T, T) {
    todo!("Implementa duplicar")
}

/// Devuelve el mayor de dos valores.
pub fn maximo<T: PartialOrd + Copy>(a: T, b: T) -> T {
    todo!("Implementa maximo")
}

/// Crea una colección heterogénea usando despacho dinámico.
///
/// Debe contener un círculo de radio `1.0` y un cuadrado de lado `2.0`.
pub fn figuras_mixtas() -> Vec<Box<dyn Figura>> {
    todo!("Implementa figuras_mixtas")
}

/// Suma las áreas de figuras heterogéneas.
pub fn area_de_mixtas(figuras: &[Box<dyn Figura>]) -> f64 {
    todo!("Implementa area_de_mixtas")
}

/// Devuelve las descripciones de las figuras. El bound `Debug` no se usa en el
/// cuerpo, pero demuestra que se pueden combinar varios bounds.
pub fn describir_todas<T>(figuras: &[T]) -> Vec<String>
where
    T: Figura + fmt::Debug,
{
    todo!("Implementa describir_todas")
}
