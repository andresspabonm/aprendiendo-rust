use e06_errors::*;
use std::error::Error;

#[test]
fn dividir_devuelve_result() {
    assert_eq!(dividir(6.0, 3.0), Ok(2.0));
    assert_eq!(dividir(0.0, 5.0), Ok(0.0));
    assert_eq!(dividir(1.0, 0.0), Err(ErrorCalc::DivisionPorCero));
}

#[test]
fn promedio_maneja_listas() {
    assert_eq!(promedio(&[1, 2, 3]), Ok(2.0));
    assert_eq!(promedio(&[5]), Ok(5.0));
    assert_eq!(promedio(&[]), Err(ErrorCalc::SinDatos));
}

#[test]
fn sumar_textos_usa_el_operador_interrogacion() {
    assert_eq!(sumar_textos("2", "3"), Ok(5));
    assert_eq!(sumar_textos("-4", "10"), Ok(6));

    let error = sumar_textos("2", "no-es-un-numero").unwrap_err();
    assert!(matches!(error, ErrorCalc::NumeroInvalido(_)));
}

#[test]
fn leer_edad_valida_el_rango() {
    assert_eq!(leer_edad("0"), Ok(0));
    assert_eq!(leer_edad("30"), Ok(30));
    assert_eq!(leer_edad("120"), Ok(120));
    assert_eq!(leer_edad("121"), Err(ErrorCalc::EdadFueraDeRango(121)));
    assert_eq!(leer_edad("200"), Err(ErrorCalc::EdadFueraDeRango(200)));

    assert!(matches!(
        leer_edad("veinte"),
        Err(ErrorCalc::NumeroInvalido(_))
    ));
    assert!(matches!(leer_edad("-1"), Err(ErrorCalc::NumeroInvalido(_))));
}

#[test]
fn display_describe_cada_error() {
    assert_eq!(
        ErrorCalc::DivisionPorCero.to_string(),
        "no se puede dividir por cero"
    );
    assert_eq!(ErrorCalc::SinDatos.to_string(), "no hay datos");
    assert_eq!(
        ErrorCalc::EdadFueraDeRango(130).to_string(),
        "la edad 130 está fuera del rango permitido"
    );

    let error = "x".parse::<i32>().unwrap_err();
    assert_eq!(
        ErrorCalc::NumeroInvalido(error).to_string(),
        "el texto no es un número válido"
    );
}

#[test]
fn el_error_expone_su_causa() {
    let error = "x".parse::<i32>().unwrap_err();
    let error = ErrorCalc::NumeroInvalido(error);

    assert!(error.source().is_some());
    assert!(ErrorCalc::SinDatos.source().is_none());
}
