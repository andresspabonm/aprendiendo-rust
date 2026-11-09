use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use e11_proyecto_tareas::{ErrorTarea, GestorTareas, Prioridad};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut gestor = GestorTareas::new();

    if args.is_empty() {
        return repl(&mut gestor);
    }

    match ejecutar(&mut gestor, &args) {
        Ok(salida) => {
            print!("{salida}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Bucle interactivo para no perder el estado entre comandos.
fn repl(gestor: &mut GestorTareas) -> ExitCode {
    println!("Gestor de tareas. Escribe 'ayuda' o 'salir'.");

    let stdin = io::stdin();
    let mut linea = String::new();

    loop {
        print!("> ");
        if io::stdout().flush().is_err() {
            return ExitCode::FAILURE;
        }

        linea.clear();
        match stdin.read_line(&mut linea) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error leyendo la entrada: {error}");
                return ExitCode::FAILURE;
            }
        }

        let args: Vec<String> = linea.split_whitespace().map(str::to_string).collect();
        if args.is_empty() {
            continue;
        }

        if matches!(args[0].as_str(), "salir" | "quit" | "exit") {
            break;
        }

        match ejecutar(gestor, &args) {
            Ok(salida) => print!("{salida}"),
            Err(error) => eprintln!("Error: {error}"),
        }
    }

    ExitCode::SUCCESS
}

fn ejecutar(gestor: &mut GestorTareas, args: &[String]) -> Result<String, ErrorTarea> {
    let Some(comando) = args.first().map(String::as_str) else {
        return Ok(ayuda());
    };

    match comando {
        "agregar" | "a" => {
            if args.len() < 3 {
                return Ok(ayuda());
            }

            let prioridad = Prioridad::desde_texto(&args[1])
                .ok_or_else(|| ErrorTarea::PrioridadInvalida(args[1].clone()))?;
            let titulo = args[2..].join(" ");
            let id = gestor.agregar(&titulo, prioridad)?;

            Ok(format!("Tarea #{id} agregada.\n"))
        }
        "listar" | "ls" => Ok(formatear_lista(gestor.listar())),
        "pendientes" => Ok(formatear_lista(gestor.pendientes())),
        "completar" | "ok" => {
            let id = parsear_id(args.get(1))?;
            gestor.completar(id)?;
            Ok(format!("Tarea #{id} completada.\n"))
        }
        "eliminar" | "rm" => {
            let id = parsear_id(args.get(1))?;
            let tarea = gestor.eliminar(id)?;
            Ok(format!("Tarea eliminada: {tarea}\n"))
        }
        "buscar" => {
            if args.len() < 2 {
                return Ok(ayuda());
            }
            let consulta = args[1..].join(" ");
            Ok(formatear_lista(gestor.buscar(&consulta)))
        }
        "progreso" => Ok(format!("Progreso: {:.1}%\n", gestor.progreso())),
        "ayuda" | "--help" | "-h" => Ok(ayuda()),
        otro => Ok(format!("Comando desconocido: {otro}\n\n{}", ayuda())),
    }
}

fn parsear_id(valor: Option<&String>) -> Result<u32, ErrorTarea> {
    let texto = valor.ok_or_else(|| ErrorTarea::IdInvalido("falta el id".to_string()))?;

    texto
        .parse()
        .map_err(|_| ErrorTarea::IdInvalido(texto.clone()))
}

fn formatear_lista(tareas: Vec<&e11_proyecto_tareas::Tarea>) -> String {
    if tareas.is_empty() {
        return "No hay tareas.\n".to_string();
    }

    let mut salida = String::new();
    for tarea in tareas {
        salida.push_str(&format!("{tarea}\n"));
    }
    salida
}

fn ayuda() -> String {
    "\
Comandos:
  agregar <baja|media|alta> <título...>   Añade una tarea
  listar                                  Muestra todas las tareas
  pendientes                              Muestra las tareas sin completar
  completar <id>                          Marca una tarea como completada
  eliminar <id>                           Elimina una tarea
  buscar <texto...>                       Busca por título
  progreso                                Porcentaje completado
  ayuda                                   Muestra esta ayuda
  salir                                   Termina el programa
"
    .to_string()
}
