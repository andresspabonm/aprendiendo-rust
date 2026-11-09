use e05_traits_generics::*;
use std::f64::consts::PI;

fn aproximado(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn circulo_y_cuadrado_implementan_figura() {
    let circulo = Circulo { radio: 1.0 };
    let cuadrado = Cuadrado { lado: 2.0 };

    assert!(aproximado(circulo.area(), PI));
    assert_eq!(cuadrado.area(), 4.0);
    assert_eq!(circulo.nombre(), "círculo");
    assert_eq!(cuadrado.nombre(), "cuadrado");
}

#[test]
fn metodo_por_defecto_usa_area_y_nombre() {
    let circulo = Circulo { radio: 1.0 };
    assert_eq!(circulo.descripcion(), "círculo con área 3.14");
}

#[test]
fn area_total_funciona_con_cualquier_figura() {
    let circulos = [Circulo { radio: 1.0 }, Circulo { radio: 2.0 }];
    let cuadrados = [Cuadrado { lado: 1.0 }, Cuadrado { lado: 3.0 }];

    assert!(aproximado(area_total(&circulos), PI * 5.0));
    assert_eq!(area_total(&cuadrados), 10.0);
    assert_eq!(area_total::<Cuadrado>(&[]), 0.0);
}

#[test]
fn mayor_area_devuelve_la_primera_en_empate() {
    let figuras = [
        Cuadrado { lado: 2.0 },
        Cuadrado { lado: 5.0 },
        Cuadrado { lado: 5.0 },
    ];

    let mayor = mayor_area(&figuras).unwrap();
    assert_eq!(mayor.lado, 5.0);
    assert_eq!(mayor_area::<Cuadrado>(&[]), None);
}

#[test]
fn duplicar_clona_cualquier_tipo_clonable() {
    let (a, b) = duplicar(String::from("rust"));
    assert_eq!(a, "rust");
    assert_eq!(b, "rust");

    let (x, y) = duplicar(7);
    assert_eq!((x, y), (7, 7));
}

#[test]
fn maximo_es_generico() {
    assert_eq!(maximo(3, 9), 9);
    assert_eq!(maximo('a', 'z'), 'z');
    assert_eq!(maximo("abc", "abd"), "abd");
}

#[test]
fn figuras_mixtas_usa_dyn_trait() {
    let figuras = figuras_mixtas();

    assert_eq!(figuras.len(), 2);
    assert_eq!(figuras[0].nombre(), "círculo");
    assert_eq!(figuras[1].nombre(), "cuadrado");
    assert!(aproximado(area_de_mixtas(&figuras), PI + 4.0));
}

#[test]
fn describir_todas_devuelve_una_descripcion_por_figura() {
    let figuras = [Circulo { radio: 1.0 }, Circulo { radio: 2.0 }];
    let descripciones = describir_todas(&figuras);

    assert_eq!(descripciones.len(), 2);
    assert_eq!(descripciones[0], "círculo con área 3.14");
    assert_eq!(descripciones[1], "círculo con área 12.57");
}
