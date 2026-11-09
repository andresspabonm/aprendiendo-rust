# Ejercicio 01 — Ownership

## Objetivo

Entender quién es dueño de cada valor, cuándo se mueve, cuándo se copia y por
qué Rust puede liberar memoria sin recolector de basura.

## Conceptos

- Cada valor tiene exactamente un dueño.
- Al asignar o pasar un valor que no es `Copy`, la propiedad se **mueve**.
- Los tipos `Copy` (enteros, `f64`, `bool`, `char`, referencias, tuplas de tipos
  `Copy`) se copian automáticamente.
- `Clone` es una copia explícita; puede ser costosa.
- Al salir de ámbito, el dueño llama a `Drop` y libera sus recursos.
- Una función que recibe un `String` por valor se queda con él; puede devolverlo.

## Qué implementar

Abre `src/lib.rs` y sustituye cada `todo!()`:

| Función | Comportamiento esperado |
|---------|-------------------------|
| `longitud` | Longitud en bytes del `String` (`.len()`). |
| `sumar_copias` | Suma de dos `i32`. |
| `mover_y_contar` | Devuelve el mismo vector y su longitud. |
| `concatenar` | Une dos `String` en uno nuevo. |
| `primera_palabra` | Primera palabra como `String`; vacío si no hay. |
| `contar_palabras` | Cuenta palabras usando un `Vec<String>` intermedio. |

## Cómo comprobar

```bash
cargo test -p e01-ownership
```

Cuando termines, no debe quedar ningún `todo!()` ni prueba fallida.

## Experimentos

Copia estos fragmentos en un proyecto temporal (`cargo new pruebas --bin`) y
explica cada error antes de corregirlo. No forman parte de las pruebas.

1. Movimiento:

   ```rust
   let a = String::from("hola");
   let b = a;
   println!("{a}");
   ```

2. La solución explícita:

   ```rust
   let a = String::from("hola");
   let b = a.clone();
   println!("{a} {b}");
   ```

3. Paso a una función:

   ```rust
   fn consumir(s: String) -> usize { s.len() }
   let texto = String::from("Rust");
   let n = consumir(texto);
   println!("{texto} mide {n}");
   ```

4. Un tipo propio: quita `Copy` y observa; vuelve a ponerlo y compara.

   ```rust
   #[derive(Clone, Copy)]
   struct Id(u32);

   let a = Id(1);
   let b = a;
   println!("{} {}", a.0, b.0);
   ```

5. ¿Qué pasa si devuelves una referencia a una variable local?

   ```rust
   fn malo() -> &str {
       let s = String::from("hola");
       &s
   }
   ```

## Pistas

- `split_whitespace()` es más robusto que `split(' ')` para este ejercicio.
- `.len()` de un `String` cuenta **bytes**, no caracteres.
- `format!` y `+` son dos formas de concatenar; con `+` el primer operando se
  consume.
- Para `contar_palabras`, crea el `Vec<String>` y pásalo por valor a una función
  que devuelva `len()`.

## Criterio de finalización

- `cargo test -p e01-ownership` pasa por completo.
- Puedes explicar la diferencia entre mover, copiar y clonar con tus palabras.
- Resolviste los cinco experimentos y sabes por qué cada error ocurre.
