# Guía rápida: Rust para quien ya programa

Esta guía no sustituye al libro oficial ni a los ejercicios. Es un mapa de las
diferencias que más chocan al llegar a Rust desde C/C++, Java o Python.

## 1. La diferencia central: propiedad

En C/C++ controlas manualmente la memoria; en Java y Python hay un recolector de
basura; en Rust cada valor tiene **un único dueño** y el compilador libera el
valor cuando el dueño sale de ámbito.

```rust
let a = String::from("hola");
let b = a;            // a se MUEVE a b; a ya no es usable
println!("{b}");

let c = 10;
let d = c;            // i32 es Copy: se copia, c sigue usable
```

- **Move**: el valor no se copia y el original deja de ser válido.
- **Copy**: tipos pequeños y simples (enteros, `bool`, `char`, tuplas de `Copy`)
  se copian bit a bit. `String`, `Vec<T>` y la mayoría de structs no son `Copy`.
- **Clone**: copia explícita y profunda, normalmente costosa.

## 2. Préstamos (`&` y `&mut`)

Prestar es pasar una referencia sin transferir la propiedad. Las reglas básicas:

1. En un momento dado puedes tener **muchas referencias inmutables** (`&T`) **o
   una sola mutable** (`&mut T`), pero no ambas.
2. Una referencia siempre debe ser válida: no puede sobrevivir al valor al que
   apunta.

```rust
fn longitud(texto: &str) -> usize {
    texto.len()
}

let s = String::from("hola");
let n = longitud(&s);       // prestado
println!("{s} mide {n}");   // s sigue siendo dueño
```

A esto se le llama *borrow checker*. No es un recolector de basura ni un
intérprete: son comprobaciones en tiempo de compilación.

## 3. `String` y `&str`

- `String`: cadena propia, modificable, en el heap.
- `&str`: vista prestada de una cadena (un slice). Un literal `"hola"` es `&str`.
- Acepta `&str` en funciones cuando solo necesites leer: sirve tanto para
  `String` (mediante *deref coercion*) como para literales.
- Las cadenas son UTF-8 y no se indexan por posición como en C o Python:
  `s[0]` no compila. Usa `.chars()`, `.bytes()` o slices validados.

## 4. Nada de `null`

- `Option<T>` representa un valor que puede faltar: `Some(valor)` o `None`.
- `Result<T, E>` representa éxito o error: `Ok(valor)` o `Err(error)`.
- No hay excepciones. Un fallo irrecuperable usa `panic!`; uno esperable se
  modela con `Result` y se propaga con `?`.

```rust
fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}
```

## 5. Enums algebraicos y `match`

Los enums de Rust pueden llevar datos distintos en cada variante y `match` obliga
a cubrir todos los casos.

```rust
enum Forma {
    Circulo(f64),
    Rectangulo { ancho: f64, alto: f64 },
}

fn area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo(r) => std::f64::consts::PI * r * r,
        Forma::Rectangulo { ancho, alto } => ancho * alto,
    }
}
```

No existe la herencia de clases. La composición y los traits cubren ese hueco.

## 6. Traits frente a interfaces

Un trait define comportamiento compartido; puede tener métodos por defecto y se
puede implementar para tipos que no controlas (si el trait es local).

```rust
trait Saluda {
    fn nombre(&self) -> &str;
    fn saludo(&self) -> String {
        format!("Hola, {}", self.nombre())
    }
}
```

- Genéricos (`fn f<T: Saluda>(x: T)`): monomorfización, sin coste en ejecución.
- `dyn Trait` (`Box<dyn Saluda>`): despacho dinámico, permite colecciones
  heterogéneas.

## 7. Mutabilidad explícita

Las variables son inmutables por defecto. `let mut x = 0;` habilita la mutación.
La mutabilidad también forma parte de los préstamos: un `&mut T` es exclusivo.

## 8. Structs, `impl` y `derive`

No hay constructores especiales: se usa una función asociada `new` si quieres.
`derive` genera implementaciones comunes:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Punto { x: i32, y: i32 }

impl Punto {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }
}
```

`Self` es un alias del tipo; `self` es la instancia. En métodos, `&self` presta,
`&mut self` presta mutable y `self` consume.

## 9. Iteradores y closures

Los iteradores son perezosos y se componen con adaptadores:

```rust
let suma: i32 = (1..=10)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .sum();
```

- `iter()` presta, `iter_mut()` presta mutable, `into_iter()` consume.
- Los closures capturan por referencia o por valor; `move` fuerza la captura por
  valor (necesario al enviar trabajo a otro hilo).

## 10. Punteros inteligentes

- `Box<T>`: propiedad única en el heap; imprescindible para tipos recursivos.
- `Rc<T>`: propiedad compartida en un solo hilo (conteo de referencias).
- `Arc<T>`: como `Rc`, pero seguro entre hilos.
- `RefCell<T>`: mutabilidad interior comprobada en tiempo de ejecución.
- `Rc<RefCell<T>>` permite compartir y mutar datos en un solo hilo.

## 11. Concurrencia sin carreras de datos

El compilador impide compartir `&mut` entre hilos. Para compartir estado se usa
`Arc<Mutex<T>>` o canales `mpsc`.

```rust
let contador = Arc::new(Mutex::new(0));
let copia = Arc::clone(&contador);
thread::spawn(move || {
    *copia.lock().unwrap() += 1;
});
```

`move` transfiere la copia del `Arc` al hilo. El `Mutex` garantiza acceso
exclusivo.

## 12. Módulos, crates y Cargo

- Un **crate** es la unidad de compilación; un **paquete** puede tener una
  librería y varios binarios.
- `mod` declara módulos; `pub` controla la visibilidad.
- `use` trae nombres al ámbito.
- Cargo gestiona dependencias, pruebas, documentación y perfiles de compilación.
- `#[cfg(test)]` y `#[test]` definen pruebas unitarias; `tests/` contiene
  pruebas de integración.

## 13. Detalles que sorprenden

- Desbordamiento de enteros: `panic` en depuración, *wrapping* en release.
- No hay conversión implícita entre tipos numéricos: usa `as` o `From`/`TryFrom`.
- `char` es un escalar Unicode de 4 bytes, no un byte.
- Los arrays `[T; N]` tienen tamaño fijo; `Vec<T>` es dinámico.
- El operador `?` solo funciona en funciones que devuelven `Result` u `Option`.
- `println!` es una macro, por eso lleva `!`.
- `unsafe` existe, pero no es necesario para los ejercicios.
- Las ediciones (2015, 2018, 2021, 2024) cambian detalles del lenguaje; este
  repositorio usa edición 2024.
- Los errores del compilador incluyen explicaciones: `rustc --explain E0382`.

## 14. Herramientas útiles

```bash
cargo check          # comprobación rápida sin binario
cargo test           # pruebas
cargo fmt            # formato
cargo clippy         # lints
cargo doc --open     # documentación
cargo tree           # dependencias
```

## Recursos

- *The Rust Programming Language*: <https://doc.rust-lang.org/book/>
- *Rust by Example*: <https://doc.rust-lang.org/rust-by-example/>
- *The Rustonomicon* (para `unsafe`, más adelante): <https://doc.rust-lang.org/nomicon/>
- Documentación estándar: <https://doc.rust-lang.org/std/>
