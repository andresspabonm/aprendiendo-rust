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
        Self { ancho, alto }
    }

    /// Área del rectángulo.
    pub fn area(&self) -> f64 {
        self.ancho * self.alto
    }

    /// Perímetro del rectángulo.
    pub fn perimetro(&self) -> f64 {
        2.0 * (self.ancho + self.alto)
    }

    /// Indica si los dos lados miden lo mismo.
    pub fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }

    /// Multiplica ambos lados por `factor`, modificando el rectángulo.
    pub fn escalar(&mut self, factor: f64) {
        self.ancho *= factor;
        self.alto *= factor;
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
        match self {
            Forma::Circulo { radio } => std::f64::consts::PI * radio.powi(2),
            Forma::Rectangulo { ancho, alto } => ancho * alto,
            Forma::Triangulo { base, altura } => base * altura / 2.0
        }
    }

    /// Devuelve `"círculo"`, `"rectángulo"` o `"triángulo"`.
    pub fn nombre(&self) -> &'static str {
        match self {
            Forma::Circulo { .. } => "círculo",
            Forma::Rectangulo { .. } => "rectángulo",
            Forma::Triangulo { .. } => "triángulo"
        }
    }

    /// Devuelve una descripción con dos decimales, por ejemplo:
    /// `"círculo con área 3.14"`.
    pub fn descripcion(&self) -> String {
        format!("{} con área {:.2}", self.nombre(), self.area())
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
        Self { x, y }
    }

    /// Devuelve el origen `(0, 0)`.
    pub fn origen() -> Self {
        Self::default()
    }

    /// Suma `dx` y `dy` a las coordenadas.
    pub fn trasladar(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    /// Distancia euclidiana al origen: `sqrt(x^2 + y^2)`.
    pub fn distancia_al_origen(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
