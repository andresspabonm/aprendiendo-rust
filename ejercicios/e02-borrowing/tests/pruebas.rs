use e02_borrowing::*;

#[test]
fn contar_vocales_ignora_mayusculas() {
    assert_eq!(contar_vocales("Hola Mundo"), 4);
    assert_eq!(contar_vocales("Rust"), 1);
    assert_eq!(contar_vocales("rhythm"), 0);
    assert_eq!(contar_vocales(""), 0);
}

#[test]
fn normalizar_modifica_en_el_lugar() {
    let mut nombre = String::from("  Hola   Mundo  ");
    normalizar(&mut nombre);

    assert_eq!(nombre, "hola-mundo");
}

#[test]
fn sumar_slice_acepta_vec_y_array() {
    let vector = vec![1, 2, 3, 4];
    let array = [10, 20, 30];

    assert_eq!(sumar_slice(&vector), 10);
    assert_eq!(sumar_slice(&array), 60);
    assert_eq!(sumar_slice(&[]), 0);
}

#[test]
fn reemplazar_negativos_modifica_el_original() {
    let mut numeros = vec![3, -1, 0, -7, 8];
    reemplazar_negativos(&mut numeros);

    assert_eq!(numeros, vec![3, 0, 0, 0, 8]);
}

#[test]
fn primer_elemento_presta_o_devuelve_none() {
    let numeros = vec![4, 5, 6];

    assert_eq!(primer_elemento(&numeros), Some(&4));
    assert_eq!(primer_elemento(&[]), None);
}

#[test]
fn dividir_en_palabras_devuelve_prestamos() {
    let texto = String::from("  hola   mundo  ");
    let palabras = dividir_en_palabras(&texto);

    assert_eq!(palabras, vec!["hola", "mundo"]);

    // Las palabras siguen apuntando a `texto`: si lo usamos, sigue vivo.
    assert_eq!(palabras[0], &texto[2..6]);
}

#[test]
fn longitud_total_suma_los_fragmentos() {
    assert_eq!(longitud_total(&["hola", "mundo"]), 9);
    assert_eq!(longitud_total(&[]), 0);
}
