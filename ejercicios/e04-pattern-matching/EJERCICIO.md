# Ejercicio 04 — Pattern matching

## Objetivo

Usar `match`, `if let`, `while let` y la desestructuración para trabajar con
enums, tuplas, opciones y referencias.

## Conceptos

- `match` compara un valor con patrones y **debe ser exhaustivo**: no puedes
  olvidar una variante.
- Los patrones pueden ligar variables, ignorar partes con `_` o `..`, añadir
  guardas con `if` y desestructurar structs, enums y tuplas.
- `if let` sirve para un solo caso; `while let` repite mientras el patrón case.
- `match` produce un valor: cada rama debe devolver el mismo tipo.
- Los patrones pueden coincidir por referencia (`&valor`) o usar *match ergonomics*
  para no escribir `&` en todas partes.

## Qué implementar

| Función | Comportamiento |
|---------|----------------|
| `ejecutar` | Recorre recursivamente `Instruccion` y devuelve los pasos avanzados. |
| `clasificar` | Clasifica un entero en cuatro categorías. |
| `describir_opcion` | Describe `Option<i32>` con las cuatro reglas del documento. |
| `sumar_opciones` | Suma los `Some`; ignora los `None`. |
| `primera_palabra_larga` | Primera palabra con longitud mínima. |
| `describir_pareja` | Desestructura una tupla y aplica reglas. |

## Cómo comprobar

```bash
cargo test -p e04-pattern-matching
```

## Experimentos

1. Añade `Instruccion::Retroceder(i32)` y observa cómo el compilador señala
   todos los `match` que dejaron de ser exhaustivos.
2. Reescribe `clasificar` usando `if`/`else if` y compara legibilidad.
3. Usa una guarda en un `match`:

   ```rust
   match n {
       x if x % 2 == 0 => "par",
       _ => "impar",
   }
   ```

4. Desestructura un struct:

   ```rust
   let p = Punto { x: 1, y: 2 };
   let Punto { x, y } = p;
   ```

5. Implementa `sumar_opciones` dos veces: una con `match`, otra con
   `if let Some(n) = valor` dentro de un `for`.

## Pistas

- En `ejecutar`, `Instruccion` se recibe por referencia: los patrones pueden ser
  `Instruccion::Avanzar(n)` y `n` será `&i32`; usa `*n` para obtener el valor.
- `veces` es `u32`; para multiplicar por un `i32`, convierte con
  `*veces as i32`.
- En `clasificar`, el orden de los casos importa: comprueba primero el signo.
- `palabra.chars().count()` cuenta caracteres, no bytes. Longitud mínima se
  refiere a caracteres.
- En `primera_palabra_larga`, `iter().find(|p| ...)` o un `for` con retorno
  temprano funcionan.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar por qué `match` es exhaustivo y qué aporta eso frente a un
  `switch` de C o Java.
- Sabes cuándo usar `if let` en lugar de un `match` completo.
