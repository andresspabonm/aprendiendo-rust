# Ejercicio 06 — Manejo de errores

## Objetivo

Modelar fallos esperables con `Result` y `Option`, propagarlos con `?` y crear un
tipo de error propio que implemente `Display` y `std::error::Error`.

## Conceptos

- `Option<T>`: hay o no hay valor. No hay `null`.
- `Result<T, E>`: éxito (`Ok`) o error (`Err`).
- `?` propaga el error y hace una conversión implícita con `From`.
- Un error propio suele ser un `enum` con una variante por causa.
- `Display` produce el mensaje para el usuario; `Debug` es para diagnóstico.
- `std::error::Error` permite usar el error con `?` en otras funciones y acceder
  a la causa con `source()`.
- `unwrap` y `expect` son aceptables en prototipos y pruebas, pero no en código
  de producción que debe recuperarse de fallos.
- `panic!` es para errores irrecuperables o bugs; no sustituye a `Result`.

## Qué implementar

1. `Display` para `ErrorCalc` con estos mensajes exactos:
   - `DivisionPorCero` → `"no se puede dividir por cero"`
   - `SinDatos` → `"no hay datos"`
   - `NumeroInvalido(_)` → `"el texto no es un número válido"`
   - `EdadFueraDeRango(edad)` → `"la edad {edad} está fuera del rango permitido"`
2. `dividir`: devuelve `Err(DivisionPorCero)` si el divisor es cero.
3. `promedio`: suma y divide; lista vacía → `SinDatos`.
4. `sumar_textos`: parsea ambos textos con `?` y suma.
5. `leer_edad`: parsea a `u8` y valida `0..=120`.

## Cómo comprobar

```bash
cargo test -p e06-errors
```

## Experimentos

1. Cambia `leer_edad` para que devuelva `Result<u8, String>`. ¿Qué ventajas tiene
   el enum frente a `String`?
2. Añade una variante `Desbordamiento` y una función que sume dos `u8` con
   `checked_add`. ¿Qué error devuelve `250 + 10`?
3. Prueba a usar `?` dentro de `main`. ¿Qué debe devolver `main` para que
   compile?
4. Escribe una función que devuelva `Option<&Tarea>` y otra que devuelva
   `Result<&Tarea, Error>`. ¿Cuál comunica mejor la causa de un fallo?
5. Convierte un `Option` en `Result` con `ok_or` y observa el tipo del error.

## Pistas

- `f.write!(f, "...")` o `write!(f, "...")` dentro de `Display`.
- Para `EdadFueraDeRango(edad)`, `edad` es `&u8`; usa `{edad}` directamente o
  `*edad` si necesitas el valor.
- `promedio`: `let suma: i32 = numeros.iter().sum();` y luego
  `suma as f64 / numeros.len() as f64`.
- `sumar_textos`:
  ```rust
  let a: i32 = a.parse()?;
  let b: i32 = b.parse()?;
  Ok(a + b)
  ```
  El `?` usa `From<ParseIntError> for ErrorCalc`.
- En `leer_edad`, parsea primero a `u8`; si el texto no cabe en `u8`, el error
  es `NumeroInvalido`, lo cual es coherente con la documentación.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar la diferencia entre `panic!`, `unwrap` y `?`.
- Entiendes qué hace `From` en la propagación del error.
