# Aprendiendo Rust

Repositorio de ejercicios para aprender Rust desde la perspectiva de alguien que
ya programa en C/C++, Java o Python. No enseña a programar desde cero: se centra
en las ideas que hacen a Rust diferente, en su sintaxis y en los errores que más
suele encontrar quien viene de otros lenguajes.

Cada ejercicio es un paquete de Cargo independiente dentro del workspace. Los
esqueletos ya compilan y las pruebas describen el comportamiento esperado; tu
trabajo es implementar las funciones marcadas con `todo!()` hasta que todas las
pruebas pasen. **No hay soluciones incluidas a propósito**: intenta resolver cada
ejercicio antes de buscar ayuda.

## Requisitos

- Rust estable (el repositorio se creó con Rust 1.99).
- Git.
- Opcional pero recomendado: `rustfmt` y `clippy`.

```bash
rustup component add rustfmt clippy
```

## Cómo trabajar

Desde la raíz del repositorio:

```bash
# Ejecutar todas las pruebas (al principio fallarán casi todas)
cargo test --workspace

# Trabajar en un solo ejercicio
cargo test -p e01-ownership

# Ejecutar una prueba concreta
cargo test -p e01-ownership longitud_cuenta_bytes

# Ver la salida de las pruebas que imprimen algo
cargo test -p e07-collections-iterators -- --nocapture

# Comprobar que compila sin ejecutar pruebas
cargo check --workspace

# Formatear y revisar el código
cargo fmt --all
cargo clippy --workspace --all-targets
```

Cada paquete tiene un archivo `EJERCICIO.md` con el objetivo, los conceptos que
practica, qué implementar, pistas y experimentos para hacer por tu cuenta. Las
pruebas están en `tests/pruebas.rs`.

## Ejercicios

El orden es sugerido: cada bloque reutiliza ideas del anterior.

| # | Paquete | Tema | Qué practicas |
|---|---------|------|---------------|
| 01 | [`e01-ownership`](ejercicios/e01-ownership/EJERCICIO.md) | Propiedad | Movimientos, `Copy` frente a `Clone`, paso por valor y por referencia, `Drop` |
| 02 | [`e02-borrowing`](ejercicios/e02-borrowing/EJERCICIO.md) | Préstamos | `&T`, `&mut T`, slices, reglas de aliasing, mutabilidad |
| 03 | [`e03-structs-enums`](ejercicios/e03-structs-enums/EJERCICIO.md) | Structs y enums | Métodos, `impl`, enums con datos, `derive`, `Default` |
| 04 | [`e04-pattern-matching`](ejercicios/e04-pattern-matching/EJERCICIO.md) | Pattern matching | `match`, `if let`, `while let`, desestructuración, exhaustividad |
| 05 | [`e05-traits-generics`](ejercicios/e05-traits-generics/EJERCICIO.md) | Traits y genéricos | Traits, métodos por defecto, bounds, `impl Trait`, `dyn Trait` |
| 06 | [`e06-errors`](ejercicios/e06-errors/EJERCICIO.md) | Errores | `Result`, `Option`, `?`, errores personalizados, `From`, `Display`, `Error` |
| 07 | [`e07-collections-iterators`](ejercicios/e07-collections-iterators/EJERCICIO.md) | Colecciones e iteradores | `Vec`, `HashMap`, closures, adaptadores, iteradores perezosos |
| 08 | [`e08-smart-pointers`](ejercicios/e08-smart-pointers/EJERCICIO.md) | Punteros inteligentes | `Box`, `Rc`, `RefCell`, recursión, propiedad compartida, mutabilidad interior |
| 09 | [`e09-concurrency`](ejercicios/e09-concurrency/EJERCICIO.md) | Concurrencia | `thread`, `mpsc`, `Arc`, `Mutex`, `move`, seguridad de hilos |
| 10 | [`e10-lifetimes`](ejercicios/e10-lifetimes/EJERCICIO.md) | Lifetimes | Anotaciones `'a`, elisión, referencias en structs, retornos prestados |
| 11 | [`e11-proyecto-tareas`](ejercicios/e11-proyecto-tareas/EJERCICIO.md) | Proyecto final | Todo lo anterior junto en un gestor de tareas con CLI y errores |

Antes de empezar, o cuando necesites repasar, consulta [`GUIA.md`](GUIA.md): un
resumen de las diferencias y particularidades de Rust frente a C/C++, Java y
Python.

## Metodología sugerida

1. Lee `EJERCICIO.md` y intenta explicar el concepto con tus palabras.
2. Abre `src/lib.rs`, lee los comentarios y sustituye cada `todo!()`.
3. Ejecuta las pruebas del ejercicio y lee con calma el error del compilador.
   En Rust, el compilador suele ser parte del proceso de aprendizaje.
4. Si una prueba falla, entiende *por qué* antes de cambiar el código.
5. No uses `clone()` solo para callar al borrow checker. Primero pregúntate quién
   debería ser el dueño del valor.
6. Cuando todo pase, intenta los "experimentos" del final del enunciado.

## Nota sobre el repositorio

El repositorio es local. Para publicarlo en GitHub, autentica `gh` y crea el
remoto:

```bash
gh auth login
gh repo create aprendiendo-rust --source=. --public --push
```
