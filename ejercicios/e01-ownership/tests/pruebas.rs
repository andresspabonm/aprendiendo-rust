use e01_ownership::*;

#[test]
fn longitud_cuenta_bytes() {
    assert_eq!(longitud("hola".to_string()), 4);
    // "ñ" ocupa dos bytes en UTF-8.
    assert_eq!(longitud("ñ".to_string()), 2);
}

#[test]
fn los_enteros_se_copian() {
    let original = 7;
    let resultado = sumar_copias(original, 5);

    assert_eq!(resultado, 12);
    assert_eq!(original, 7, "i32 es Copy: el original sigue disponible");
}

#[test]
fn mover_y_contar_devuelve_el_vector() {
    let numeros = vec![10, 20, 30];
    let (devueltos, cantidad) = mover_y_contar(numeros);

    assert_eq!(devueltos, vec![10, 20, 30]);
    assert_eq!(cantidad, 3);
}

#[test]
fn concatenar_une_los_dos_textos() {
    let resultado = concatenar("Hola, ".to_string(), "Rust".to_string());
    assert_eq!(resultado, "Hola, Rust");
}

#[test]
fn primera_palabra_ignora_espacios() {
    assert_eq!(primera_palabra("  hola mundo  "), "hola");
    assert_eq!(primera_palabra("una"), "una");
    assert_eq!(primera_palabra("   "), "");
    assert_eq!(primera_palabra(""), "");
}

#[test]
fn contar_palabras_cuenta_las_palabras() {
    assert_eq!(contar_palabras("uno dos tres"), 3);
    assert_eq!(contar_palabras("  hola   mundo  "), 2);
    assert_eq!(contar_palabras(""), 0);
}
