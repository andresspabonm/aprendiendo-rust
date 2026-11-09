use e07_collections_iterators::*;
use std::collections::HashMap;

#[test]
fn frecuencias_ignora_mayusculas_y_puntuacion() {
    let resultado = frecuencias("Hola, mundo. Hola!");

    let esperado = HashMap::from([("hola".to_string(), 2), ("mundo".to_string(), 1)]);
    assert_eq!(resultado, esperado);
}

#[test]
fn pares_al_cuadrado_filtra_y_transforma() {
    assert_eq!(pares_al_cuadrado(&[1, 2, 3, 4, 5]), vec![4, 16]);
    assert_eq!(pares_al_cuadrado(&[]), Vec::<i32>::new());
}

#[test]
fn suma_de_pares_solo_suma_pares() {
    assert_eq!(suma_de_pares(&[1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(suma_de_pares(&[1, 3, 5]), 0);
}

#[test]
fn nombres_largos_respeta_el_minimo() {
    let nombres = vec![
        "Ana".to_string(),
        "Beatriz".to_string(),
        "Luis".to_string(),
        "Alejandro".to_string(),
    ];

    assert_eq!(
        nombres_largos(&nombres, 4),
        vec!["Beatriz", "Luis", "Alejandro"]
    );
    assert_eq!(nombres_largos(&nombres, 8), vec!["Alejandro"]);
    assert_eq!(nombres_largos(&nombres, 20), Vec::<String>::new());
}

#[test]
fn agrupar_por_longitud_conserva_el_orden() {
    let palabras = vec![
        "a".to_string(),
        "bb".to_string(),
        "cc".to_string(),
        "ddd".to_string(),
    ];

    let grupos = agrupar_por_longitud(&palabras);

    assert_eq!(grupos.get(&1).unwrap(), &vec!["a".to_string()]);
    assert_eq!(
        grupos.get(&2).unwrap(),
        &vec!["bb".to_string(), "cc".to_string()]
    );
    assert_eq!(grupos.get(&3).unwrap(), &vec!["ddd".to_string()]);
}

#[test]
fn palabra_mas_frecuente_elige_la_menor_en_empate() {
    assert_eq!(
        palabra_mas_frecuente("b a b a c"),
        Some(("a".to_string(), 2))
    );
    assert_eq!(
        palabra_mas_frecuente("Rust es genial y Rust es rápido"),
        Some(("es".to_string(), 2))
    );
    assert_eq!(palabra_mas_frecuente(""), None);
}

#[test]
fn aplicar_dos_veces_compone_la_funcion() {
    assert_eq!(aplicar_dos_veces(|x| x * 2, 3), 12);
    assert_eq!(aplicar_dos_veces(|x| x + 1, 0), 2);
}

#[test]
fn acumular_usa_fold() {
    assert_eq!(acumular(&[1, 2, 3, 4], 0, |acc, x| acc + x), 10);
    assert_eq!(acumular(&[1, 2, 3, 4], 1, |acc, x| acc * x), 24);
    assert_eq!(acumular(&[], 42, |acc, x| acc + x), 42);
}
