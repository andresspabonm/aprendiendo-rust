//! Ejercicio 09 — Concurrencia.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e09-concurrency
//! ```

use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/// Suma un vector repartiendo el trabajo entre varios hilos.
///
/// - Si el vector está vacío, devuelve `0`.
/// - Si `num_hilos` es mayor que la cantidad de elementos, no debe fallar.
/// - El orden de los elementos no importa para una suma.
///
/// Sugerencia: divide el vector en trozos con `chunks`, crea un hilo por trozo,
/// guarda los `JoinHandle` y suma los resultados de `join()`.
pub fn sumar_en_paralelo(numeros: Vec<i32>, num_hilos: usize) -> i32 {
    if numeros.is_empty() {
        return 0;
    } else if num_hilos == 0 {
        return numeros.iter().sum();
    }

    let trozos = numeros.chunks(numeros.len().div_ceil(num_hilos));
    let mut handles = Vec::new();
    for trozo in trozos {
        let vector = trozo.to_vec();
        let handle = thread::spawn(move || vector.into_iter().sum::<i32>());
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

/// Incrementa un contador compartido desde varios hilos.
///
/// Crea `num_hilos` hilos; cada uno suma `veces_por_hilo` al mismo contador.
/// Debe usar `Arc<Mutex<i32>>` para evitar carreras de datos y devolver el valor
/// final.
pub fn incrementar_compartido(veces_por_hilo: usize, num_hilos: usize) -> i32 {
    let contador = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 1..=num_hilos {
        let c = Arc::clone(&contador);
        handles.push(thread::spawn(move || *c.lock().unwrap() += veces_por_hilo));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *contador.lock().unwrap() as i32
}

/// Envía mensajes por un canal desde un hilo productor y los devuelve en orden.
///
/// El hilo productor recibe un `mpsc::Sender<String>` y envía cada mensaje. El
/// hilo principal recibe hasta que el canal se cierra. Debe usar
/// `thread::spawn(move || ...)`.
pub fn enviar_por_canal(mensajes: Vec<String>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        for mensaje in mensajes {
            tx.send(mensaje).unwrap();
        }
    });

    let recibidos = rx.into_iter().collect();
    handle.join().unwrap();

    recibidos
}
