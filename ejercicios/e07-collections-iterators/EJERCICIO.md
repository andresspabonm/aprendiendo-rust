# Ejercicio 07 — Colecciones e iteradores

## Objetivo

Manejar `Vec` y `HashMap`, escribir closures y encadenar adaptadores de
iteradores sin bucles manuales.

## Conceptos

- `Vec<T>` es un array dinámico en el heap.
- `HashMap<K, V>` almacena pares; requiere que `K` implemente `Eq + Hash`.
- `entry()` permite insertar o modificar en una sola operación.
- Los iteradores son perezosos: nada se ejecuta hasta que se consume con
  `collect`, `sum`, `count`, `for`, etc.
- Adaptadores comunes: `map`, `filter`, `filter_map`, `flat_map`, `take`,
  `skip`, `chain`, `zip`, `enumerate`, `rev`.
- Consumidores comunes: `collect`, `sum`, `product`, `count`, `max`, `min`,
  `find`, `any`, `all`, `fold`.
- `iter()` presta, `iter_mut()` presta mutable, `into_iter()` consume.
- Los closures capturan del entorno; `Fn`, `FnMut` y `FnOnce` describen cómo lo
  hacen.

## Qué implementar

| Función | Comportamiento |
|---------|----------------|
| `frecuencias` | Cuenta palabras normalizadas. |
| `pares_al_cuadrado` | Filtra pares y eleva al cuadrado. |
| `suma_de_pares` | Suma pares. |
| `nombres_largos` | Nombres con longitud mínima. |
| `agrupar_por_longitud` | `HashMap<usize, Vec<String>>`. |
| `palabra_mas_frecuente` | Máximo conteo, desempate lexicográfico. |
| `aplicar_dos_veces` | Composición de un closure. |
| `acumular` | `fold` con closure de dos argumentos. |

## Cómo comprobar

```bash
cargo test -p e07-collections-iterators
```

## Experimentos

1. Reescribe `pares_al_cuadrado` con un `for` y compara con la versión con
   iteradores. ¿Cuál es más clara?
2. Implementa `frecuencias` usando `entry` y otra vez con `get_mut`. ¿Qué
   ventaja tiene `entry`?
3. Cambia `frecuencias` para devolver también la longitud media de las palabras.
4. Usa `filter_map` para procesar `Vec<&str>` donde algunas cadenas no son
   números.
5. Prueba `let mut contador = 0; let mut f = || contador += 1;` y luego llama a
   `f()` dos veces. ¿Qué tipo de closure es? ¿Qué pasa si lo pasas a un hilo?
6. Recorre un `HashMap` con `for (clave, valor) in &mapa`. ¿En qué orden salen
   las claves? ¿Por qué?

## Pistas

- Para normalizar palabras:
  ```rust
  texto.split_whitespace()
      .map(|p| p.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
      .filter(|p| !p.is_empty())
  ```
- `entry(palabra).or_insert(0)` devuelve `&mut usize`; increméntalo con `+= 1`.
- `agrupar_por_longitud`: `palabras.iter().fold(HashMap::new(), |mut mapa, p| { ... })`
  o un `for`.
- Para el desempate de `palabra_mas_frecuente`, `max_by` con una comparación que
  tenga en cuenta primero el conteo y luego la clave invertida, o recorre todas
  las entradas y elige manualmente.
- `acumular` es literalmente `valores.iter().fold(inicial, f)`, pero `f` recibe
  `i32`; tendrás que desreferenciar el elemento (`|acc, &x| f(acc, x)`).

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar por qué los iteradores son perezosos.
- Sabes cuándo usar `iter`, `iter_mut` e `into_iter`.
