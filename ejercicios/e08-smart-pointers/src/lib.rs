//! Ejercicio 08 — Punteros inteligentes.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa los métodos marcados con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e08-smart-pointers
//! ```

use std::cell::RefCell;
use std::rc::Rc;

/// Lista enlazada recursiva construida con `Box`.
///
/// Sin `Box`, el enum tendría tamaño infinito porque cada `Cons` contendría
/// otra `Lista` directamente.
#[derive(Debug, Clone, PartialEq)]
pub enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}

impl Lista {
    /// Crea una lista vacía (`Nil`).
    pub fn new() -> Self {
        Self::Nil
    }

    /// Inserta `valor` al principio y devuelve la lista resultante.
    ///
    /// Consume `self` para no clonar toda la lista.
    pub fn push(self, valor: i32) -> Self {
        Self::Cons(valor, Box::new(self))
    }

    /// Crea una lista con los valores en el mismo orden del slice.
    ///
    /// `Lista::desde_slice(&[1, 2, 3])` debe representar `1 -> 2 -> 3 -> Nil`.
    pub fn desde_slice(valores: &[i32]) -> Self {
        valores
            .iter()
            .rev()
            .fold(Self::new(), |lista, &valor| lista.push(valor))
    }

    /// Número de elementos.
    pub fn longitud(&self) -> usize {
        let mut l = 0;
        let mut actual = self;
        while let Self::Cons(_, siguiente) = actual {
            l += 1;
            actual = siguiente;
        }
        l
    }

    /// Suma de todos los elementos.
    pub fn suma(&self) -> i32 {
        let mut s = 0;
        let mut actual = self;
        while let Self::Cons(valor, siguiente) = actual {
            s += *valor;
            actual = siguiente;
        }
        s
    }

    /// Convierte la lista en un `Vec` en el mismo orden.
    pub fn a_vec(&self) -> Vec<i32> {
        let mut v = Vec::new();
        let mut actual = self;
        while let Self::Cons(valor, siguiente) = actual {
            v.push(*valor);
            actual = siguiente;
        }
        v
    }
}

impl Default for Lista {
    fn default() -> Self {
        Lista::Nil
    }
}

/// Datos compartidos y mutables en un solo hilo.
///
/// `Rc` permite varios dueños y `RefCell` permite mutar a través de una
/// referencia compartida. La comprobación de préstamos ocurre en ejecución: si
/// intentas tener dos préstamos mutables a la vez, el programa entra en pánico.
#[derive(Debug, Clone)]
pub struct Compartido {
    datos: Rc<RefCell<Vec<i32>>>,
}

impl Compartido {
    /// Crea un contenedor compartido vacío.
    pub fn nuevo() -> Self {
        Self {
            datos: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Añade un valor al vector interno.
    pub fn agregar(&self, valor: i32) {
        self.datos.borrow_mut().push(valor);
    }

    /// Devuelve una copia del contenido actual.
    pub fn valores(&self) -> Vec<i32> {
        self.datos.borrow().clone()
    }

    /// Suma todos los valores.
    pub fn total(&self) -> i32 {
        self.datos.borrow().iter().sum()
    }

    /// Número de dueños del `Rc` interno.
    pub fn num_referencias(&self) -> usize {
        Rc::strong_count(&self.datos)
    }
}
