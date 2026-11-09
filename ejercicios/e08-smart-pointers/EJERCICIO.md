# Ejercicio 08 — Punteros inteligentes

## Objetivo

Entender `Box`, `Rc` y `RefCell`: cuándo hacen falta, qué propiedad representan
y cómo permiten estructuras recursivas o datos compartidos.

## Conceptos

- `Box<T>` guarda `T` en el heap y deja en la pila un puntero de tamaño fijo.
  Tiene propiedad única. Es la forma habitual de romper la recursión de tipos.
- `Rc<T>` (reference counted) permite varios dueños en un mismo hilo. Cada
  `clone()` incrementa el contador; al caer el último, se libera el valor.
  `Rc` no es `Send`, por lo que no puede cruzar hilos.
- `RefCell<T>` mueve la comprobación de préstamos a tiempo de ejecución y
  permite mutabilidad interior: mutar a través de `&self`.
- `Rc<RefCell<T>>` es el patrón clásico para compartir y mutar datos en un solo
  hilo. En concurrencia se sustituye por `Arc<Mutex<T>>`.
- `borrow()` y `borrow_mut()` de `RefCell` devuelven guardas; si violas las
  reglas en ejecución, el programa entra en pánico.
- `Box` no sirve para compartir: si necesitas dos dueños, usa `Rc` o `Arc`.

## Qué implementar

### `Lista`

Una lista enlazada recursiva:

```rust
Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
```

- `new`: `Nil`.
- `push`: antepone un elemento (`Cons(valor, Box::new(self))`).
- `desde_slice`: construye en orden; puedes iterar el slice al revés y hacer
  `push`, o recursión.
- `longitud`, `suma`, `a_vec`: recorren la lista.

### `Compartido`

- `nuevo`: `Rc::new(RefCell::new(Vec::new()))`.
- `agregar`: `self.datos.borrow_mut().push(valor)`.
- `valores`: clona el vector interno.
- `total`: suma los valores.
- `num_referencias`: `Rc::strong_count(&self.datos)`.

## Cómo comprobar

```bash
cargo test -p e08-smart-pointers
```

## Experimentos

1. Intenta definir `enum Lista { Cons(i32, Lista), Nil }` sin `Box`. Lee el error
   sobre tamaño infinito.
2. En `Compartido`, haz dos `borrow_mut()` simultáneos y ejecuta. ¿Qué mensaje
   aparece? ¿En qué se diferencia de un error de compilación?
3. Cambia `Rc` por `Arc` y `RefCell` por `Mutex` en una copia del ejercicio.
   ¿Qué métodos cambian?
4. Crea dos `Rc` que apunten al mismo `String` y comprueba
   `Rc::strong_count` y `Rc::ptr_eq`.
5. Intenta enviar un `Rc` a un hilo con `thread::spawn`. ¿Qué trait falta?
6. Investiga `Weak<T>`: ¿para qué sirve y cómo evita ciclos de referencias?

## Pistas

- `Box::new(self)` es suficiente para `push`.
- `desde_slice`:
  ```rust
  valores.iter().rev().fold(Lista::Nil, |lista, &valor| lista.push(valor))
  ```
- Para recorrer: `match self { Lista::Nil => ..., Lista::Cons(valor, resto) => ... }`.
- `a_vec` puede ser recursivo: procesa el resto y luego inserta al principio, o
  usa un `while let` con una referencia mutable.
- `Rc::strong_count` es una función asociada: `Rc::strong_count(&self.datos)`.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar por qué `Box` es necesario en `Lista`.
- Sabes la diferencia entre `Rc<RefCell<T>>`, `Arc<Mutex<T>>` y `Box<T>`.
