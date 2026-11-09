use e09_concurrency::*;

#[test]
fn sumar_en_paralelo_suma_todo() {
    let numeros: Vec<i32> = (1..=100).collect();
    assert_eq!(sumar_en_paralelo(numeros, 4), 5050);
}

#[test]
fn sumar_en_paralelo_maneja_casos_limite() {
    assert_eq!(sumar_en_paralelo(vec![], 4), 0);
    assert_eq!(sumar_en_paralelo(vec![7], 8), 7);
    assert_eq!(sumar_en_paralelo(vec![1, 2, 3], 1), 6);
}

#[test]
fn incrementar_compartido_no_pierde_incrementos() {
    assert_eq!(incrementar_compartido(1_000, 4), 4_000);
}

#[test]
fn incrementar_compartido_casos_limite() {
    assert_eq!(incrementar_compartido(0, 4), 0);
    assert_eq!(incrementar_compartido(5, 1), 5);
}

#[test]
fn enviar_por_canal_conserva_los_mensajes() {
    let mensajes = vec!["uno".to_string(), "dos".to_string(), "tres".to_string()];

    assert_eq!(enviar_por_canal(mensajes.clone()), mensajes);
}

#[test]
fn enviar_por_canal_maneja_lista_vacia() {
    assert_eq!(enviar_por_canal(vec![]), Vec::<String>::new());
}
