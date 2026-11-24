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
        match texto.trim().to_lowercase().as_str() {
            "baja" => Some(Self::Baja),
            "media" => Some(Self::Media),
            "alta" => Some(Self::Alta),
            _ => None,
        }
    }
}

impl fmt::Display for Prioridad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Baja => write!(f, "baja"),
            Self::Media => write!(f, "media"),
            Self::Alta => write!(f, "alta"),
        }
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
        write!(
            f,
            "[{}] #{} ({}) {}",
            if self.completada { "x" } else { " " },
            self.id,
            self.prioridad,
            self.titulo
        )
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
        match self {
            Self::TituloVacio => write!(f, "el título no puede estar vacío"),
            Self::NoEncontrada(id) => write!(f, "no existe la tarea con id {id}"),
            Self::PrioridadInvalida(texto) => {
                write!(f, "prioridad inválida: {texto} (usa baja, media o alta)")
            }
            Self::IdInvalido(texto) => write!(f, "id inválido: {texto}"),
        }
    }
}

impl std::error::Error for ErrorTarea {}

/// Gestor de tareas en memoria.
///
/// Usa `BTreeMap` para que `listar` devuelva las tareas ordenadas por id.
#[derive(Debug)]
pub struct GestorTareas {
    tareas: BTreeMap<u32, Tarea>,
    siguiente_id: u32,
}

impl Default for GestorTareas {
    fn default() -> Self {
        Self::new()
    }
}

impl GestorTareas {
    /// Crea un gestor vacío. El primer id asignado debe ser `1`.
    pub fn new() -> Self {
        Self {
            tareas: BTreeMap::new(),
            siguiente_id: 1,
        }
    }

    /// Añade una tarea y devuelve su id.
    ///
    /// Recorta espacios del título. Si queda vacío, devuelve `TituloVacio`.
    pub fn agregar(&mut self, titulo: &str, prioridad: Prioridad) -> Result<u32, ErrorTarea> {
        let t = String::from(titulo.trim());
        if t.is_empty() {
            return Err(ErrorTarea::TituloVacio);
        }

        let id = self.siguiente_id;

        self.tareas.insert(
            id,
            Tarea {
                id,
                titulo: t,
                prioridad,
                completada: false,
            },
        );
        self.siguiente_id += 1;

        Ok(id)
    }

    /// Marca una tarea como completada.
    ///
    /// Si ya estaba completada, no falla.
    pub fn completar(&mut self, id: u32) -> Result<(), ErrorTarea> {
        match self.tareas.get_mut(&id) {
            None => Err(ErrorTarea::NoEncontrada(id)),
            Some(tarea) => {
                tarea.completada = true;
                Ok(())
            }
        }
    }

    /// Elimina una tarea y la devuelve.
    pub fn eliminar(&mut self, id: u32) -> Result<Tarea, ErrorTarea> {
        match self.tareas.remove(&id) {
            None => Err(ErrorTarea::NoEncontrada(id)),
            Some(tarea) => Ok(tarea),
        }
    }

    /// Devuelve todas las tareas ordenadas por id.
    pub fn listar(&self) -> Vec<&Tarea> {
        self.tareas.values().collect()
    }

    /// Devuelve las tareas no completadas, ordenadas por id.
    pub fn pendientes(&self) -> Vec<&Tarea> {
        self.listar()
            .into_iter()
            .filter(|tarea| !tarea.completada)
            .collect()
    }

    /// Devuelve las tareas de una prioridad concreta, ordenadas por id.
    pub fn por_prioridad(&self, prioridad: Prioridad) -> Vec<&Tarea> {
        self.listar()
            .into_iter()
            .filter(|tarea| tarea.prioridad == prioridad)
            .collect()
    }

    /// Devuelve las tareas cuyo título contenga `texto`, sin distinguir
    /// mayúsculas. Ordenadas por id.
    pub fn buscar(&self, texto: &str) -> Vec<&Tarea> {
        self.listar()
            .into_iter()
            .filter(|tarea| tarea.titulo.to_lowercase().contains(&texto.to_lowercase()))
            .collect()
    }

    /// Porcentaje de tareas completadas, de `0.0` a `100.0`.
    ///
    /// Si no hay tareas, devuelve `0.0`.
    pub fn progreso(&self) -> f64 {
        match self.listar().len() {
            0 => 0.0,
            total => {
                let resto = self.pendientes().len();
                let faltante = resto as f64 / total as f64;
                (1.0 - faltante) * 100.0
            }
        }
    }
}
