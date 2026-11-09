use e03_structs_enums::*;
use std::f64::consts::PI;

fn aproximado(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn rectangulo_calcula_area_y_perimetro() {
    let rect = Rectangulo::new(3.0, 4.0);

    assert_eq!(rect.ancho, 3.0);
    assert_eq!(rect.alto, 4.0);
    assert_eq!(rect.area(), 12.0);
    assert_eq!(rect.perimetro(), 14.0);
    assert!(!rect.es_cuadrado());

    assert!(Rectangulo::new(5.0, 5.0).es_cuadrado());
}

#[test]
fn rectangulo_se_escala_en_el_lugar() {
    let mut rect = Rectangulo::new(2.0, 3.0);
    rect.escalar(2.0);

    assert_eq!(rect, Rectangulo::new(4.0, 6.0));
}

#[test]
fn formas_calculan_su_area() {
    let circulo = Forma::Circulo { radio: 1.0 };
    let rectangulo = Forma::Rectangulo {
        ancho: 2.0,
        alto: 5.0,
    };
    let triangulo = Forma::Triangulo {
        base: 4.0,
        altura: 3.0,
    };

    assert!(aproximado(circulo.area(), PI));
    assert_eq!(rectangulo.area(), 10.0);
    assert_eq!(triangulo.area(), 6.0);
}

#[test]
fn formas_tienen_nombre_y_descripcion() {
    let circulo = Forma::Circulo { radio: 1.0 };

    assert_eq!(circulo.nombre(), "círculo");
    assert_eq!(circulo.descripcion(), "círculo con área 3.14");

    assert_eq!(
        Forma::Rectangulo {
            ancho: 1.0,
            alto: 1.0
        }
        .nombre(),
        "rectángulo"
    );
    assert_eq!(
        Forma::Triangulo {
            base: 1.0,
            altura: 1.0
        }
        .nombre(),
        "triángulo"
    );
}

#[test]
fn punto_nuevo_y_traslado() {
    let mut punto = Punto::new(2, -3);
    assert_eq!(punto.x, 2);
    assert_eq!(punto.y, -3);

    punto.trasladar(5, 8);
    assert_eq!(punto, Punto { x: 7, y: 5 });
}

#[test]
fn punto_origen_y_default() {
    assert_eq!(Punto::origen(), Punto { x: 0, y: 0 });
    assert_eq!(Punto::default(), Punto::origen());
}

#[test]
fn punto_calcula_distancia() {
    assert_eq!(Punto::new(3, 4).distancia_al_origen(), 5.0);
    assert_eq!(Punto::origen().distancia_al_origen(), 0.0);
}

#[test]
fn actualizacion_de_struct_copia_el_resto() {
    let base = Punto::new(1, 2);
    let otro = Punto { x: 10, ..base };

    assert_eq!(otro, Punto { x: 10, y: 2 });
    assert_eq!(base, Punto { x: 1, y: 2 }, "Punto es Copy");
}
