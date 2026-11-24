//! Ejercicio 06 — Manejo de errores.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa los métodos y funciones
//! marcados con `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e06-errors
//! ```

use std::fmt;
use std::num::ParseIntError;

/// Errores de la calculadora.
#[derive(Debug, PartialEq)]
pub enum ErrorCalc {
    /// Se intentó dividir por cero.
    DivisionPorCero,
    /// No hay datos suficientes.
    SinDatos,
    /// El texto no era un número entero válido.
    NumeroInvalido(ParseIntError),
    /// La edad está fuera del rango permitido.
    EdadFueraDeRango(u8),
}

impl fmt::Display for ErrorCalc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCalc::DivisionPorCero => write!(f, "no se puede dividir por cero"),
            ErrorCalc::SinDatos => write!(f, "no hay datos"),
            ErrorCalc::NumeroInvalido(_e) => write!(f, "el texto no es un número válido"),
            ErrorCalc::EdadFueraDeRango(edad) => write!(f, "la edad {edad} está fuera del rango permitido")
        }
    }
}

impl std::error::Error for ErrorCalc {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorCalc::NumeroInvalido(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParseIntError> for ErrorCalc {
    fn from(error: ParseIntError) -> Self {
        ErrorCalc::NumeroInvalido(error)
    }
}

/// Divide `a` entre `b`.
///
/// Devuelve `Err(ErrorCalc::DivisionPorCero)` si `b` es cero.
pub fn dividir(a: f64, b: f64) -> Result<f64, ErrorCalc> {
    if b == 0.0 {
        Err(ErrorCalc::DivisionPorCero)
    } else {
        Ok(a / b)
    }
}

/// Calcula el promedio de una lista.
///
/// Devuelve `Err(ErrorCalc::SinDatos)` si la lista está vacía.
pub fn promedio(numeros: &[i32]) -> Result<f64, ErrorCalc> {
    if numeros.len() == 0 {
        Err(ErrorCalc::SinDatos)
    } else {
        Ok(numeros.iter().sum::<i32>() as f64 / numeros.len() as f64)
    }
}

/// Suma dos textos que representan enteros.
///
/// Debe usar el operador `?` para propagar el error de `parse`.
pub fn sumar_textos(a: &str, b: &str) -> Result<i32, ErrorCalc> {
    let a_num: i32 = a.parse()?;
    let b_num: i32 = b.parse()?;
    Ok(a_num + b_num)
}

/// Convierte un texto en edad.
///
/// Acepta valores entre 0 y 120 inclusive. Si el número es válido pero está
/// fuera de rango, devuelve `EdadFueraDeRango`.
pub fn leer_edad(texto: &str) -> Result<u8, ErrorCalc> {
    let edad: u8 = texto.parse()?;
    if edad > 120 {
        Err(ErrorCalc::EdadFueraDeRango(edad))
    } else {
        Ok(edad)
    }
}
