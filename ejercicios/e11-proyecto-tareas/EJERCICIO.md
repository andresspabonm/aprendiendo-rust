# Ejercicio 11 — Proyecto final: gestor de tareas

## Objetivo

Combinar structs, enums, `Result`, `BTreeMap`, iteradores, `Display`, módulos y
pruebas en un pequeño proyecto con biblioteca y binario.

## Enunciado

Implementa un gestor de tareas en memoria. La biblioteca ya define los tipos y
las firmas; tú implementas la lógica. El binario de ejemplo (`src/main.rs`) usa
la biblioteca y funciona en cuanto termines.

Cada tarea tiene:

- `id`: entero asignado automáticamente (empieza en 1).
- `titulo`: texto no vacío, sin espacios sobrantes.
- `completada`: `bool`.
- `prioridad`: `Baja`, `Media` o `Alta`.

## Qué implementar

### `Prioridad`

- `desde_texto`: acepta `baja`, `media` y `alta` sin distinguir mayúsculas ni
  espacios.
- `Display`: `"baja"`, `"media"`, `"alta"`.

### `Tarea`

- `Display`: `"[ ] #3 (alta) Comprar pan"` o
  `"[x] #3 (alta) Comprar pan"`.

### `ErrorTarea`

- `Display` con estos mensajes exactos:
  - `TituloVacio` → `"el título no puede estar vacío"`
  - `NoEncontrada(id)` → `"no existe la tarea con id {id}"`
  - `PrioridadInvalida(texto)` → `"prioridad inválida: {texto} (usa baja, media o alta)"`
  - `IdInvalido(texto)` → `"id inválido: {texto}"`

### `GestorTareas`

- `new`: gestor vacío; el primer id será 1.
- `agregar`: recorta el título; vacío → `TituloVacio`; asigna id incremental.
- `completar`: marca; id inexistente → `NoEncontrada`.
- `eliminar`: quita y devuelve la tarea; id inexistente → `NoEncontrada`.
- `listar`: todas, ordenadas por id.
- `pendientes`: solo `completada == false`, ordenadas por id.
- `por_prioridad`: filtra por prioridad, ordenadas por id.
- `buscar`: título contiene el texto sin distinguir mayúsculas, ordenadas por id.
- `progreso`: `completadas / total * 100.0`; sin tareas → `0.0`.

## Cómo comprobar

```bash
cargo test -p e11-proyecto-tareas
```

Después de implementar la biblioteca, prueba la CLI:

```bash
cargo run -p e11-proyecto-tareas
```

Dentro del programa:

```text
> agregar alta Comprar pan
> agregar media Estudiar Rust
> listar
> completar 1
> progreso
> pendientes
> salir
```

También puedes ejecutar un comando suelto:

```bash
cargo run -p e11-proyecto-tareas -- listar
```

## Experimentos

1. Añade persistencia sencilla en un archivo de texto (`tareas.txt`). Guarda al
   salir y carga al iniciar. ¿Qué errores nuevos necesitas modelar?
2. Añade una fecha límite usando `std::time::SystemTime`. ¿Cómo la formatearías
   sin dependencias externas?
3. Añade un comando `editar <id> <nuevo título...>`.
4. Ordena por prioridad en lugar de por id. ¿Qué pasa con las tareas que tienen
   la misma prioridad?
5. Sustituye `BTreeMap` por `HashMap` y ordena explícitamente en `listar`. ¿Qué
   cambia?
6. Escribe pruebas de integración que ejecuten el binario con
   `std::process::Command` y comprueben la salida.
7. Separa el parseo de argumentos en un módulo `cli` con su propio `mod` y
   pruebas. ¿Qué debe ser `pub` y qué puede quedar privado?

## Pistas

- `BTreeMap` ya mantiene las claves ordenadas; `values()` devuelve las tareas en
  orden de id.
- `values().filter(|t| !t.completada).collect()`.
- Para `buscar`, `let consulta = texto.to_lowercase();` y compara con
  `tarea.titulo.to_lowercase().contains(&consulta)`.
- `agregar`:
  ```rust
  let titulo = titulo.trim();
  if titulo.is_empty() { return Err(ErrorTarea::TituloVacio); }
  self.siguiente_id += 1;
  let id = self.siguiente_id;
  self.tareas.insert(id, Tarea { ... });
  Ok(id)
  ```
- `eliminar`: `self.tareas.remove(&id).ok_or(ErrorTarea::NoEncontrada(id))`.
- En `Display` de `Tarea`, el estado se puede calcular con
  `if self.completada { 'x' } else { ' ' }`.
- La CLI no guarda datos entre ejecuciones: es intencional. La persistencia es
  uno de los experimentos.

## Criterio de finalización

- Todas las pruebas pasan.
- La CLI funciona en modo interactivo y con un comando directo.
- Puedes explicar cómo se combinan `Result`, `BTreeMap` y `Display` en el
  proyecto.
- No usas `unwrap()` en la lógica de la biblioteca salvo en casos imposibles.
