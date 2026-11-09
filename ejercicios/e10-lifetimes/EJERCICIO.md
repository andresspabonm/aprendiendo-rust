# Ejercicio 10 — Lifetimes

## Objetivo

Entender qué significan las anotaciones `'a`, cuándo son necesarias y cuándo el
compilador puede inferirlas por elisión.

## Conceptos

- Un lifetime no es cuánto vive un valor, sino una relación entre referencias:
  "esta referencia es válida al menos mientras viva aquella".
- La mayoría de las veces no se escriben: las reglas de elisión cubren los casos
  comunes.
- En una función con varias referencias de entrada y una de salida, el
  compilador necesita saber con cuál se relaciona la salida.
- Un struct que guarda referencias necesita un parámetro de lifetime:
  `struct Extracto<'a> { texto: &'a str }`.
- `'static` significa que la referencia vive toda la ejecución del programa
  (por ejemplo, los literales de cadena).
- Los lifetimes son una comprobación en tiempo de compilación; no cambian el
  código generado ni añaden coste en ejecución.

## Qué implementar

1. `mas_larga`: compara longitudes y devuelve una de las dos referencias.
2. `primera_palabra`: usa `split_whitespace().next().unwrap_or("")`.
3. `Extracto::new`: valida índices y usa `texto.get(inicio..fin)`; `get` devuelve
   `None` si el rango no es válido o no está en un límite de carácter.
4. `Extracto::contenido`: devuelve la porción; puedes guardarla con
   `self.texto.get(self.inicio..self.fin).unwrap()` o recalcular.
5. `Extracto::texto_completo`: devuelve `self.texto`.
6. `buscar`: recorre `textos` y devuelve la primera que contenga `patron`.

## Cómo comprobar

```bash
cargo test -p e10-lifetimes
```

## Experimentos

1. Intenta que `mas_larga` devuelva una referencia a un `String` local creado
   dentro de la función. ¿Qué error aparece?
2. Cambia la firma de `primera_palabra` a
   `fn primera_palabra(texto: &str) -> &str` y luego añade un segundo parámetro
   `&str`. ¿Sigue pudiendo elidirse el lifetime? Prueba.
3. Añade a `Extracto` un método `fn unir<'b>(&'b self, otro: &'b Extracto<'a>)`.
   ¿Qué relación describe `'b`?
4. Crea una función que devuelva `&'static str` y otra que devuelva un préstamo
   normal. ¿Cuándo es válido usar `'static`?
5. Intenta usar un `Extracto` después de que el `String` original se libere.
   ¿Dónde señala el error?
6. Investiga la elisión en métodos: ¿por qué `fn contenido(&self) -> &'a str`
   necesita `'a` explícito y `fn nombre(&self) -> &str` no?

## Pistas

- `a.len() >= b.len()` decide; devuelve `a` o `b`.
- `texto.get(inicio..fin)` es la forma segura de hacer slicing: no entra en
  pánico y respeta los límites UTF-8.
- Para `contenido`, puedes guardar `texto.get(...).unwrap()` en el constructor,
  pero el struct ya tiene los índices; recalcular es simple.
- En `buscar`, `textos.iter().find(|t| t.contains(patron)).map(|t| t.as_str())`.
  Fíjate en que `patron` no aparece en el tipo de retorno: su lifetime no influye.
- `String` se convierte en `&str` con `.as_str()` o con *deref coercion*.

## Criterio de finalización

- Todas las pruebas pasan.
- Puedes explicar qué relación describe `'a` en `mas_larga`.
- Sabes por qué `Extracto` necesita un parámetro de lifetime y `primera_palabra`
  no.
