use e04_pattern_matching::*;

#[test]
fn ejecutar_avanza_y_gira() {
    assert_eq!(ejecutar(&Instruccion::Avanzar(5)), 5);
    assert_eq!(ejecutar(&Instruccion::Girar { grados: 90 }), 0);
    assert_eq!(ejecutar(&Instruccion::Detener), 0);
}

#[test]
fn ejecutar_repite_instrucciones() {
    let repetir = Instruccion::Repetir {
        veces: 3,
        instruccion: Box::new(Instruccion::Avanzar(2)),
    };

    assert_eq!(ejecutar(&repetir), 6);
}

#[test]
fn ejecutar_repite_de_forma_anidada() {
    let anidada = Instruccion::Repetir {
        veces: 2,
        instruccion: Box::new(Instruccion::Repetir {
            veces: 4,
            instruccion: Box::new(Instruccion::Avanzar(3)),
        }),
    };

    assert_eq!(ejecutar(&anidada), 24);
}

#[test]
fn clasificar_cubre_los_cuatro_casos() {
    assert_eq!(clasificar(-4), "negativo");
    assert_eq!(clasificar(0), "cero");
    assert_eq!(clasificar(8), "positivo par");
    assert_eq!(clasificar(9), "positivo impar");
}

#[test]
fn describir_opcion_distingue_todos_los_casos() {
    assert_eq!(describir_opcion(None), "sin valor");
    assert_eq!(describir_opcion(Some(7)), "positivo: 7");
    assert_eq!(describir_opcion(Some(0)), "cero");
    assert_eq!(describir_opcion(Some(-3)), "negativo: -3");
}

#[test]
fn sumar_opciones_ignora_none() {
    assert_eq!(sumar_opciones(&[Some(1), None, Some(2), None, Some(3)]), 6);
    assert_eq!(sumar_opciones(&[]), 0);
    assert_eq!(sumar_opciones(&[None, None]), 0);
}

#[test]
fn primera_palabra_larga_respeta_el_minimo() {
    let texto = "sol luna estrella mar";

    assert_eq!(primera_palabra_larga(texto, 4), Some("luna"));
    assert_eq!(primera_palabra_larga(texto, 6), Some("estrella"));
    assert_eq!(primera_palabra_larga(texto, 20), None);
}

#[test]
fn describir_pareja_usa_patrones() {
    assert_eq!(describir_pareja((0, "cualquiera")), "cero");
    assert_eq!(describir_pareja((5, "importante")), "importante: 5");
    assert_eq!(describir_pareja((5, "normal")), "5: normal");
}
