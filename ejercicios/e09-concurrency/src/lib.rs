//! Ejercicio 09 — Concurrencia.
//!
//! Lee `EJERCICIO.md` antes de empezar. Implementa las funciones marcadas con
//! `todo!()` y ejecuta:
//!
//! ```text
//! cargo test -p e09-concurrency
//! ```

/// Suma un vector repartiendo el trabajo entre varios hilos.
///
/// - Si el vector está vacío, devuelve `0`.
/// - Si `num_hilos` es mayor que la cantidad de elementos, no debe fallar.
/// - El orden de los elementos no importa para una suma.
///
/// Sugerencia: divide el vector en trozos con `chunks`, crea un hilo por trozo,
/// guarda los `JoinHandle` y suma los resultados de `join()`.
pub fn sumar_en_paralelo(numeros: Vec<i32>, num_hilos: usize) -> i32 {
    todo!("Implementa sumar_en_paralelo")
}

/// Incrementa un contador compartido desde varios hilos.
///
/// Crea `num_hilos` hilos; cada uno suma `veces_por_hilo` al mismo contador.
/// Debe usar `Arc<Mutex<i32>>` para evitar carreras de datos y devolver el valor
/// final.
pub fn incrementar_compartido(veces_por_hilo: usize, num_hilos: usize) -> i32 {
    todo!("Implementa incrementar_compartido")
}

/// Envía mensajes por un canal desde un hilo productor y los devuelve en orden.
///
/// El hilo productor recibe un `mpsc::Sender<String>` y envía cada mensaje. El
/// hilo principal recibe hasta que el canal se cierra. Debe usar
/// `thread::spawn(move || ...)`.
pub fn enviar_por_canal(mensajes: Vec<String>) -> Vec<String> {
    todo!("Implementa enviar_por_canal")
}
