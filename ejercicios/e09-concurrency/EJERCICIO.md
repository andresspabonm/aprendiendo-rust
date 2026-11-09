# Ejercicio 09 — Concurrencia

## Objetivo

Crear hilos, compartir estado de forma segura y comunicar hilos con canales,
usando las garantías que aporta el compilador.

## Conceptos

- `thread::spawn` crea un hilo; devuelve un `JoinHandle<T>`.
- `move` en un closure transfiere las capturas al hilo. Sin `move`, el closure
  podría prestar datos que no viven lo suficiente.
- `Arc<T>` es un `Rc` seguro entre hilos (`Send + Sync` cuando `T` lo es).
- `Mutex<T>` garantiza acceso exclusivo; `lock()` devuelve un `Result` porque el
  mutex puede estar envenenado si un hilo entró en pánico.
- `mpsc` significa *multiple producer, single consumer*. `Sender` se puede
  clonar; `Receiver` no.
- Al caer todos los `Sender`, el `Receiver` recibe un error de cierre y un
  `for mensaje in receptor` termina.
- El compilador impide compartir `&mut` entre hilos y usar tipos no `Send`. Las
  carreras de datos se convierten en errores de compilación.

## Qué implementar

1. `sumar_en_paralelo`
   - Divide `numeros` en `num_hilos` trozos (o menos si hay pocos elementos).
   - Crea un hilo por trozo, cada uno suma su parte.
   - Recoge los resultados con `join()` y súmalos.
   - Si el vector está vacío, devuelve `0` sin crear hilos innecesarios.

2. `incrementar_compartido`
   - Crea `Arc::new(Mutex::new(0))`.
   - Por cada hilo, clona el `Arc`, usa `move` y dentro hace
     `*contador.lock().unwrap() += veces_por_hilo`.
   - Espera a todos los hilos y devuelve el valor final.

3. `enviar_por_canal`
   - Crea `let (tx, rx) = mpsc::channel();`.
   - Mueve `tx` a un hilo con `move` y envía todos los mensajes.
   - Recibe en el hilo principal y devuelve el `Vec<String>`.

## Cómo comprobar

```bash
cargo test -p e09-concurrency
```

Las pruebas usan suficiente trabajo como para detectar incrementos perdidos,
pero no son una prueba de estrés. Ejecútalas varias veces:

```bash
for i in $(seq 1 20); do cargo test -p e09-concurrency --quiet || break; done
```

## Experimentos

1. Quita `move` del closure de `incrementar_compartido`. ¿Qué error aparece?
2. Intenta usar `Rc<Mutex<i32>>` en lugar de `Arc`. ¿Qué trait falta y por qué?
3. Cambia `Mutex` por `RefCell` dentro de los hilos. ¿Compila? ¿Qué pasaría en
   ejecución si compilara?
4. Haz que dos hilos productores envíen al mismo canal clonando el `Sender`.
5. Prueba `try_lock` en lugar de `lock` y explica cuándo es útil.
6. ¿Qué ocurre si un hilo entra en pánico mientras tiene el mutex bloqueado?
   Investiga `PoisonError` y `into_inner`.

## Pistas

- Para dividir: `numeros.chunks((numeros.len() + num_hilos - 1) / num_hilos)`.
  También puedes usar `chunks` directamente y limitar el número de hilos.
- Un trozo es `&[i32]`; para moverlo al hilo, conviértelo en `Vec` con
  `trozo.to_vec()`.
- `JoinHandle::join()` devuelve `Result<T, Box<dyn Any + Send>>`; usa `.unwrap()`
  en las pruebas.
- `Arc::clone(&contador)` es más explícito que `contador.clone()`.
- El receptor se puede recorrer con `for mensaje in receptor { ... }`; termina
  cuando todos los emisores se han liberado.

## Criterio de finalización

- Todas las pruebas pasan de forma repetida.
- Puedes explicar por qué `Arc<Mutex<T>>` es seguro y `Rc<RefCell<T>>` no sirve
  entre hilos.
- Entiendes qué hace `move` al crear un hilo.
