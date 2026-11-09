//! Ejercicio 03 — Structs y enums.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa los métodos marcados con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e03-structs-enums
//! ```

/// Rectángulo con lados de tipo `f64`.
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangulo {
    pub ancho: f64,
    pub alto: f64,
}

impl Rectangulo {
    /// Crea un rectángulo con las dimensiones dadas.
    pub fn new(ancho: f64, alto: f64) -> Self {
        todo!("Implementa Rectangulo::new")
    }

    /// Área del rectángulo.
    pub fn area(&self) -> f64 {
        todo!("Implementa Rectangulo::area")
    }

    /// Perímetro del rectángulo.
    pub fn perimetro(&self) -> f64 {
        todo!("Implementa Rectangulo::perimetro")
    }

    /// Indica si los dos lados miden lo mismo.
    pub fn es_cuadrado(&self) -> bool {
        todo!("Implementa Rectangulo::es_cuadrado")
    }

    /// Multiplica ambos lados por `factor`, modificando el rectángulo.
    pub fn escalar(&mut self, factor: f64) {
        todo!("Implementa Rectangulo::escalar")
    }
}

/// Formas geométricas heterogéneas.
///
/// Fíjate en que cada variante puede llevar datos con nombre o posicionales.
#[derive(Debug, Clone, PartialEq)]
pub enum Forma {
    Circulo { radio: f64 },
    Rectangulo { ancho: f64, alto: f64 },
    Triangulo { base: f64, altura: f64 },
}

impl Forma {
    /// Calcula el área según la variante.
    ///
    /// - Círculo: `PI * radio^2`.
    /// - Rectángulo: `ancho * alto`.
    /// - Triángulo: `base * altura / 2`.
    pub fn area(&self) -> f64 {
        todo!("Implementa Forma::area")
    }

    /// Devuelve `"círculo"`, `"rectángulo"` o `"triángulo"`.
    pub fn nombre(&self) -> &'static str {
        todo!("Implementa Forma::nombre")
    }

    /// Devuelve una descripción con dos decimales, por ejemplo:
    /// `"círculo con área 3.14"`.
    pub fn descripcion(&self) -> String {
        todo!("Implementa Forma::descripcion")
    }
}

/// Punto en el plano cartesiano.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Punto {
    pub x: i32,
    pub y: i32,
}

impl Punto {
    /// Crea un punto con las coordenadas dadas.
    pub fn new(x: i32, y: i32) -> Self {
        todo!("Implementa Punto::new")
    }

    /// Devuelve el origen `(0, 0)`.
    pub fn origen() -> Self {
        todo!("Implementa Punto::origen")
    }

    /// Suma `dx` y `dy` a las coordenadas.
    pub fn trasladar(&mut self, dx: i32, dy: i32) {
        todo!("Implementa Punto::trasladar")
    }

    /// Distancia euclidiana al origen: `sqrt(x^2 + y^2)`.
    pub fn distancia_al_origen(&self) -> f64 {
        todo!("Implementa Punto::distancia_al_origen")
    }
}
