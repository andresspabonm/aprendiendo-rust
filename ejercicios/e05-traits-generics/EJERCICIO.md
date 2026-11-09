# Ejercicio 05 — Traits y genéricos

## Objetivo

Definir comportamiento compartido con traits, escribir funciones genéricas con
bounds y distinguir despacho estático (`T`) de despacho dinámico (`dyn Trait`).

## Conceptos

- Un trait es un contrato: métodos que un tipo puede implementar.
- Los métodos pueden tener implementación por defecto.
- `impl Trait for Tipo` implementa el trait para un tipo concreto.
- Un genérico `fn f<T: Trait>(x: T)` se monomorfiza: el compilador genera una
  versión por cada tipo concreto. Sin coste en ejecución.
- `Box<dyn Trait>` usa despacho dinámico mediante una tabla virtual y permite
  colecciones heterogéneas.
- Los bounds se pueden escribir en línea (`T: Trait`) o en una cláusula `where`.
- `Clone`, `Copy`, `PartialOrd`, `Debug` también son traits de la biblioteca
  estándar.

## Qué implementar

1. Implementa `Figura` para `Circulo` y `Cuadrado`.
   - `Circulo::area` = `PI * radio^2`; nombre `"círculo"`.
   - `Cuadrado::area` = `lado^2`; nombre `"cuadrado"`.
2. `area_total`: suma todas las áreas (un `for` o un iterador).
3. `mayor_area`: recorre y guarda la mayor; en empate conserva la primera.
4. `duplicar`: usa `clone()`.
5. `maximo`: compara y devuelve el mayor.
6. `figuras_mixtas`: `vec![Box::new(Circulo { radio: 1.0 }), Box::new(Cuadrado { lado: 2.0 })]`.
7. `area_de_mixtas`: suma las áreas.
8. `describir_todas`: devuelve `descripcion()` de cada figura.

## Cómo comprobar

```bash
cargo test -p e05-traits-generics
```

## Experimentos

1. Añade un trait `Perimetro` con método `perimetro` e impleméntalo para las dos
   figuras. Luego escribe `fn resumen<T: Figura + Perimetro>(f: &T)`.
2. Intenta usar `dyn Figura` sin `Box`. ¿Qué error aparece y por qué?
3. Añade un método `fn escala(&self, factor: f64) -> Self` a `Figura` con un
   bound `where Self: Sized`. ¿Por qué hace falta `Sized`?
4. Implementa `Figura` para `Vec<Circulo>` sumando áreas. ¿Es buena idea?
5. Compara el tamaño de `Vec<Circulo>` y `Vec<Box<dyn Figura>>` con
   `std::mem::size_of_val`.

## Pistas

- Para el área del círculo, usa `std::f64::consts::PI`.
- Para `mayor_area`, puedes usar `figuras.iter().max_by(|a, b| ...)`, pero
  `f64` no implementa `Ord`; compara con `partial_cmp` o usa un `for` con una
  variable `Option<&T>`.
- Un `for figura in figuras` sobre `&[T]` da `&T`.
- `Box<dyn Figura>` se crea con `Box::new(...)`.
- `describir_todas` puede ser `figuras.iter().map(|f| f.descripcion()).collect()`.
- El bound `fmt::Debug` no se usa en el cuerpo: es solo una demostración.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar cuándo elegir `T: Trait` y cuándo `Box<dyn Trait>`.
- Entiendes por qué el método por defecto no hay que reimplementarlo.
