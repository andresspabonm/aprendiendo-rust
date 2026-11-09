use e10_lifetimes::*;

#[test]
fn mas_larga_elige_la_cadena_larga() {
    assert_eq!(mas_larga("corto", "mucho más largo"), "mucho más largo");
    assert_eq!(mas_larga("abc", "xy"), "abc");
    assert_eq!(mas_larga("igual", "igual"), "igual");
}

#[test]
fn mas_larga_devuelve_un_prestamo_utilizable() {
    let a = String::from("primera");
    let b = String::from("segunda cadena");
    let resultado = mas_larga(&a, &b);

    assert_eq!(resultado, "segunda cadena");
    assert_eq!(a, "primera");
    assert_eq!(b, "segunda cadena");
}

#[test]
fn primera_palabra_sin_anotaciones() {
    assert_eq!(primera_palabra("  hola mundo"), "hola");
    assert_eq!(primera_palabra("una"), "una");
    assert_eq!(primera_palabra("   "), "");
    assert_eq!(primera_palabra(""), "");
}

#[test]
fn extracto_valido_tiene_contenido_y_texto() {
    let texto = String::from("hola mundo");
    let extracto = Extracto::new(&texto, 0, 4).unwrap();

    assert_eq!(extracto.contenido(), "hola");
    assert_eq!(extracto.texto_completo(), "hola mundo");
}

#[test]
fn extracto_rechaza_indices_invalidos() {
    let texto = String::from("hola mundo");

    assert!(Extracto::new(&texto, 5, 2).is_none());
    assert!(Extracto::new(&texto, 0, 99).is_none());
}

#[test]
fn extracto_respeta_limites_de_caracteres_utf8() {
    // "ñ" ocupa dos bytes: el índice 1 está en medio de un carácter.
    assert!(Extracto::new("ñ", 0, 1).is_none());
    assert_eq!(Extracto::new("ñ", 0, 2).unwrap().contenido(), "ñ");
}

#[test]
fn buscar_devuelve_el_primer_coincidente() {
    let textos = vec![
        "uno".to_string(),
        "dos".to_string(),
        "documento".to_string(),
    ];

    assert_eq!(buscar(&textos, "do"), Some("dos"));
    assert_eq!(buscar(&textos, "mento"), Some("documento"));
    assert_eq!(buscar(&textos, "zzz"), None);
}
