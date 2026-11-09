# Ejercicio 02 — Borrowing

## Objetivo

Aprender a prestar datos con `&T` y `&mut T`, usar slices y entender las reglas
que el borrow checker hace cumplir.

## Conceptos

- `&T` es una referencia inmutable compartida.
- `&mut T` es una referencia mutable exclusiva.
- Puedes tener muchos `&T` **o** un solo `&mut T`, nunca ambos a la vez.
- Un `&[T]` es un slice: una vista de una secuencia contigua. Funciona con
  vectores, arrays y otros slices.
- Las referencias no pueden sobrevivir al valor prestado.
- La elisión de lifetimes permite escribir `fn f(&str) -> &str` sin anotar
  `'a`; el compilador aplica reglas predecibles.

## Qué implementar

| Función | Comportamiento esperado |
|---------|-------------------------|
| `contar_vocales` | Cuenta `a,e,i,o,u` sin distinguir mayúsculas. |
| `normalizar` | Recorta, pasa a minúsculas y une palabras con `-`. |
| `sumar_slice` | Suma los elementos del slice. |
| `reemplazar_negativos` | Cambia negativos por `0` en el slice original. |
| `primer_elemento` | `Some(&primero)` o `None` si está vacío. |
| `dividir_en_palabras` | `Vec<&str>` con las palabras del texto. |
| `longitud_total` | Suma de longitudes de los fragmentos. |

## Cómo comprobar

```bash
cargo test -p e02-borrowing
```

## Experimentos

Hazlos en un proyecto temporal y explica el error del compilador.

1. Dos préstamos mutables a la vez:

   ```rust
   let mut v = vec![1, 2, 3];
   let a = &mut v;
   let b = &mut v;
   a.push(4);
   b.push(5);
   ```

2. Préstamo inmutable y mutable simultáneos:

   ```rust
   let mut v = vec![1, 2, 3];
   let a = &v;
   let b = &mut v;
   println!("{a:?}");
   b.push(4);
   ```

3. Devolver una referencia a un valor local:

   ```rust
   fn malo() -> &Vec<i32> {
       let v = vec![1, 2, 3];
       &v
   }
   ```

4. Un préstamo que se usa después de que el dueño cambia:

   ```rust
   let mut s = String::from("hola");
   let r = &s;
   s.push_str(" mundo");
   println!("{r}");
   ```

5. Compara `fn f(s: &str)` con `fn f(s: String)` llamando a ambas con la misma
   variable. ¿Cuál deja el valor disponible después?

## Pistas

- `texto.to_lowercase()` crea un `String` nuevo; para modificar en el lugar,
  asigna el resultado con `*nombre = ...`.
- `split_whitespace().collect::<Vec<_>>()` sirve para `dividir_en_palabras`.
- Para `normalizar`, `split_whitespace().collect::<Vec<_>>().join("-")` es una
  forma clara.
- En `reemplazar_negativos`, recorre con `for numero in numeros` (el slice es
  `&mut [i32]`, así que `numero` es `&mut i32`).
- `primer_elemento`: `numeros.first()` ya hace exactamente lo pedido.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar por qué `&mut` es exclusivo.
- Entiendes por qué `dividir_en_palabras` no necesita un `'a` explícito.
