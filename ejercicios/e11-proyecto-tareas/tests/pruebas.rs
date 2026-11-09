use e11_proyecto_tareas::*;

fn ids(tareas: &[&Tarea]) -> Vec<u32> {
    tareas.iter().map(|tarea| tarea.id).collect()
}

#[test]
fn prioridad_se_parsea_y_se_muestra() {
    assert_eq!(Prioridad::desde_texto("alta"), Some(Prioridad::Alta));
    assert_eq!(Prioridad::desde_texto(" MEDIA "), Some(Prioridad::Media));
    assert_eq!(Prioridad::desde_texto("Baja"), Some(Prioridad::Baja));
    assert_eq!(Prioridad::desde_texto("urgente"), None);

    assert_eq!(Prioridad::Baja.to_string(), "baja");
    assert_eq!(Prioridad::Media.to_string(), "media");
    assert_eq!(Prioridad::Alta.to_string(), "alta");
}

#[test]
fn agregar_asigna_ids_consecutivos_y_recorta_el_titulo() {
    let mut gestor = GestorTareas::new();

    assert_eq!(gestor.agregar("  Comprar pan  ", Prioridad::Alta), Ok(1));
    assert_eq!(gestor.agregar("Estudiar Rust", Prioridad::Media), Ok(2));
    assert_eq!(gestor.agregar("Llamar a Ana", Prioridad::Baja), Ok(3));

    let tareas = gestor.listar();
    assert_eq!(tareas.len(), 3);
    assert_eq!(tareas[0].titulo, "Comprar pan");
    assert_eq!(tareas[0].prioridad, Prioridad::Alta);
    assert!(!tareas[0].completada);
}

#[test]
fn agregar_rechaza_titulos_vacios() {
    let mut gestor = GestorTareas::new();

    assert_eq!(
        gestor.agregar("   ", Prioridad::Baja),
        Err(ErrorTarea::TituloVacio)
    );
    assert_eq!(
        gestor.agregar("", Prioridad::Baja),
        Err(ErrorTarea::TituloVacio)
    );
    assert!(gestor.listar().is_empty());
}

#[test]
fn completar_cambia_el_estado() {
    let mut gestor = GestorTareas::new();
    let id = gestor.agregar("Tarea", Prioridad::Media).unwrap();

    assert_eq!(gestor.completar(id), Ok(()));
    assert!(gestor.listar()[0].completada);

    // Completar dos veces no es un error.
    assert_eq!(gestor.completar(id), Ok(()));
}

#[test]
fn completar_y_eliminar_fallan_con_id_inexistente() {
    let mut gestor = GestorTareas::new();

    assert_eq!(gestor.completar(42), Err(ErrorTarea::NoEncontrada(42)));
    assert_eq!(gestor.eliminar(42), Err(ErrorTarea::NoEncontrada(42)));
}

#[test]
fn eliminar_devuelve_la_tarea_y_la_quita() {
    let mut gestor = GestorTareas::new();
    let id = gestor.agregar("Temporal", Prioridad::Baja).unwrap();

    let eliminada = gestor.eliminar(id).unwrap();
    assert_eq!(eliminada.id, id);
    assert_eq!(eliminada.titulo, "Temporal");
    assert!(gestor.listar().is_empty());
}

#[test]
fn listar_y_pendientes_ordenan_por_id() {
    let mut gestor = GestorTareas::new();
    gestor.agregar("Uno", Prioridad::Baja).unwrap();
    gestor.agregar("Dos", Prioridad::Alta).unwrap();
    gestor.agregar("Tres", Prioridad::Media).unwrap();
    gestor.completar(2).unwrap();

    assert_eq!(ids(&gestor.listar()), vec![1, 2, 3]);
    assert_eq!(ids(&gestor.pendientes()), vec![1, 3]);
}

#[test]
fn por_prioridad_filtra_y_ordena() {
    let mut gestor = GestorTareas::new();
    gestor.agregar("Baja 1", Prioridad::Baja).unwrap();
    gestor.agregar("Alta 1", Prioridad::Alta).unwrap();
    gestor.agregar("Baja 2", Prioridad::Baja).unwrap();

    assert_eq!(ids(&gestor.por_prioridad(Prioridad::Baja)), vec![1, 3]);
    assert_eq!(ids(&gestor.por_prioridad(Prioridad::Alta)), vec![2]);
    assert!(gestor.por_prioridad(Prioridad::Media).is_empty());
}

#[test]
fn buscar_ignora_mayusculas() {
    let mut gestor = GestorTareas::new();
    gestor.agregar("Estudiar Rust", Prioridad::Alta).unwrap();
    gestor.agregar("Comprar pan", Prioridad::Baja).unwrap();
    gestor.agregar("Repasar RUST", Prioridad::Media).unwrap();

    assert_eq!(ids(&gestor.buscar("rust")), vec![1, 3]);
    assert_eq!(ids(&gestor.buscar("PAN")), vec![2]);
    assert!(gestor.buscar("java").is_empty());
}

#[test]
fn progreso_calcula_el_porcentaje() {
    let mut gestor = GestorTareas::new();
    assert_eq!(gestor.progreso(), 0.0);

    gestor.agregar("Uno", Prioridad::Baja).unwrap();
    gestor.agregar("Dos", Prioridad::Baja).unwrap();
    assert_eq!(gestor.progreso(), 0.0);

    gestor.completar(1).unwrap();
    assert_eq!(gestor.progreso(), 50.0);

    gestor.completar(2).unwrap();
    assert_eq!(gestor.progreso(), 100.0);
}

#[test]
fn display_de_tarea_muestra_estado_prioridad_y_titulo() {
    let pendiente = Tarea {
        id: 3,
        titulo: "Estudiar".to_string(),
        completada: false,
        prioridad: Prioridad::Alta,
    };
    assert_eq!(pendiente.to_string(), "[ ] #3 (alta) Estudiar");

    let completada = Tarea {
        completada: true,
        ..pendiente
    };
    assert_eq!(completada.to_string(), "[x] #3 (alta) Estudiar");
}

#[test]
fn display_de_errores_es_claro() {
    assert_eq!(
        ErrorTarea::TituloVacio.to_string(),
        "el título no puede estar vacío"
    );
    assert_eq!(
        ErrorTarea::NoEncontrada(9).to_string(),
        "no existe la tarea con id 9"
    );
    assert_eq!(
        ErrorTarea::PrioridadInvalida("urgente".to_string()).to_string(),
        "prioridad inválida: urgente (usa baja, media o alta)"
    );
    assert_eq!(
        ErrorTarea::IdInvalido("abc".to_string()).to_string(),
        "id inválido: abc"
    );
}
