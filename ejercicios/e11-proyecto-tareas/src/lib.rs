//! Ejercicio 11 — Proyecto final: gestor de tareas.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa los métodos y `Display`
//! marcados con `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e11-proyecto-tareas
//! cargo run -p e11-proyecto-tareas
//! ```

use std::collections::BTreeMap;
use std::fmt;

/// Prioridad de una tarea. El orden de las variantes define su orden natural:
/// `Baja < Media < Alta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prioridad {
    Baja,
    Media,
    Alta,
}

impl Prioridad {
    /// Convierte un texto en prioridad, sin distinguir mayúsculas ni espacios.
    ///
    /// Acepta `"baja"`, `"media"` y `"alta"`. Cualquier otra cosa devuelve
    /// `None`.
    pub fn desde_texto(texto: &str) -> Option<Self> {
        todo!("Implementa Prioridad::desde_texto")
    }
}

impl fmt::Display for Prioridad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implementa Display para Prioridad")
    }
}

/// Una tarea del gestor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tarea {
    pub id: u32,
    pub titulo: String,
    pub completada: bool,
    pub prioridad: Prioridad,
}

impl fmt::Display for Tarea {
    /// Formato: `[ ] #3 (alta) Comprar pan` o `[x] #3 (alta) Comprar pan`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implementa Display para Tarea")
    }
}

/// Errores del gestor.
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorTarea {
    /// El título está vacío o solo tiene espacios.
    TituloVacio,
    /// No existe una tarea con ese id.
    NoEncontrada(u32),
    /// El texto no corresponde a una prioridad válida.
    PrioridadInvalida(String),
    /// El id recibido no es un número válido.
    IdInvalido(String),
}

impl fmt::Display for ErrorTarea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Implementa Display para ErrorTarea")
    }
}

impl std::error::Error for ErrorTarea {}

/// Gestor de tareas en memoria.
///
/// Usa `BTreeMap` para que `listar` devuelva las tareas ordenadas por id.
#[derive(Debug, Default)]
pub struct GestorTareas {
    tareas: BTreeMap<u32, Tarea>,
    siguiente_id: u32,
}

impl GestorTareas {
    /// Crea un gestor vacío. El primer id asignado debe ser `1`.
    pub fn new() -> Self {
        todo!("Implementa GestorTareas::new")
    }

    /// Añade una tarea y devuelve su id.
    ///
    /// Recorta espacios del título. Si queda vacío, devuelve `TituloVacio`.
    pub fn agregar(&mut self, titulo: &str, prioridad: Prioridad) -> Result<u32, ErrorTarea> {
        todo!("Implementa GestorTareas::agregar")
    }

    /// Marca una tarea como completada.
    ///
    /// Si ya estaba completada, no falla.
    pub fn completar(&mut self, id: u32) -> Result<(), ErrorTarea> {
        todo!("Implementa GestorTareas::completar")
    }

    /// Elimina una tarea y la devuelve.
    pub fn eliminar(&mut self, id: u32) -> Result<Tarea, ErrorTarea> {
        todo!("Implementa GestorTareas::eliminar")
    }

    /// Devuelve todas las tareas ordenadas por id.
    pub fn listar(&self) -> Vec<&Tarea> {
        todo!("Implementa GestorTareas::listar")
    }

    /// Devuelve las tareas no completadas, ordenadas por id.
    pub fn pendientes(&self) -> Vec<&Tarea> {
        todo!("Implementa GestorTareas::pendientes")
    }

    /// Devuelve las tareas de una prioridad concreta, ordenadas por id.
    pub fn por_prioridad(&self, prioridad: Prioridad) -> Vec<&Tarea> {
        todo!("Implementa GestorTareas::por_prioridad")
    }

    /// Devuelve las tareas cuyo título contenga `texto`, sin distinguir
    /// mayúsculas. Ordenadas por id.
    pub fn buscar(&self, texto: &str) -> Vec<&Tarea> {
        todo!("Implementa GestorTareas::buscar")
    }

    /// Porcentaje de tareas completadas, de `0.0` a `100.0`.
    ///
    /// Si no hay tareas, devuelve `0.0`.
    pub fn progreso(&self) -> f64 {
        todo!("Implementa GestorTareas::progreso")
    }
}
