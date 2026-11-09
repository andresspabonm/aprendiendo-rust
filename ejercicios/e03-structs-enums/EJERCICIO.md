# Ejercicio 03 — Structs y enums

## Objetivo

Modelar datos con structs y enums, implementar comportamiento con bloques `impl`
y aprovechar `derive` para generar código común.

## Conceptos

- Un `struct` agrupa campos con nombre (o posicionales, en una tuple struct).
- Los métodos viven en `impl Tipo`; `&self`, `&mut self` y `self` indican cómo
  se recibe la instancia.
- Un `enum` puede tener variantes con datos, distintas en cada variante. Es una
  unión etiquetada, no una simple constante.
- `#[derive(...)]` genera implementaciones: `Debug`, `Clone`, `Copy`,
  `PartialEq`, `Eq`, `Default`, etc.
- `Self` es alias del tipo dentro de su `impl`.
- La sintaxis de actualización `Tipo { campo: valor, ..base }` rellena el resto
  de campos desde `base` (y mueve o copia según el tipo).
- Las funciones asociadas sin `self` se llaman con `Tipo::funcion()`.

## Qué implementar

### `Rectangulo`

- `new`: constructor.
- `area`: `ancho * alto`.
- `perimetro`: `2 * (ancho + alto)`.
- `es_cuadrado`: compara ambos lados.
- `escalar`: modifica `ancho` y `alto` en el lugar.

### `Forma`

- `area`: según la variante (`PI * r^2`, `ancho * alto`, `base * altura / 2`).
- `nombre`: `"círculo"`, `"rectángulo"` o `"triángulo"`.
- `descripcion`: `format!("{} con área {:.2}", self.nombre(), self.area())`.

### `Punto`

- `new`, `origen`, `trasladar`.
- `distancia_al_origen`: `sqrt(x^2 + y^2)`, usando `f64`.

## Cómo comprobar

```bash
cargo test -p e03-structs-enums
```

## Experimentos

1. Añade a `Rectangulo` un método `fn con_escala(self, factor: f64) -> Self` que
   consuma y devuelva un rectángulo nuevo. ¿Puedes usar el original después?
2. Intenta quitar `PartialEq` de `Rectangulo`. ¿Qué error aparece en las pruebas
   y por qué?
3. Crea una tuple struct `struct Metros(f64);` e implementa `sumar`. ¿Qué
   ventaja tiene frente a usar un `f64` suelto?
4. Añade una variante `Cuadrado { lado: f64 }` a `Forma`, actualiza `area`,
   `nombre` y `descripcion`, y observa cómo el compilador te obliga a cubrirla en
   todos los `match`.
5. Cambia `#[derive(Default)]` de `Punto` por una implementación manual. ¿Cuándo
   tiene sentido una u otra?

## Pistas

- Para el círculo, usa `std::f64::consts::PI` (o importa
  `use std::f64::consts::PI;`).
- Para `distancia_al_origen`: `((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()`.
- `Self { ancho, alto }` usa *field init shorthand*: equivale a
  `Self { ancho: ancho, alto: alto }`.
- `Punto::origen` puede ser `Self::default()`.
- En `Forma::descripcion`, `self.nombre()` y `self.area()` se llaman igual que
  cualquier método.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar la diferencia entre un struct y un enum con datos.
- Sabes cuándo un método debe recibir `&self`, `&mut self` o `self`.
