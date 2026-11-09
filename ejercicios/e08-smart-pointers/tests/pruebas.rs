use e08_smart_pointers::*;

#[test]
fn lista_vacia_y_push() {
    assert_eq!(Lista::new(), Lista::Nil);

    let lista = Lista::new().push(4).push(5);
    assert_eq!(lista.a_vec(), vec![5, 4]);
}

#[test]
fn lista_desde_slice_conserva_el_orden() {
    let lista = Lista::desde_slice(&[1, 2, 3]);

    assert_eq!(lista.longitud(), 3);
    assert_eq!(lista.suma(), 6);
    assert_eq!(lista.a_vec(), vec![1, 2, 3]);
}

#[test]
fn lista_vacia_tiene_longitud_cero() {
    let lista = Lista::default();

    assert_eq!(lista.longitud(), 0);
    assert_eq!(lista.suma(), 0);
    assert_eq!(lista.a_vec(), Vec::<i32>::new());
}

#[test]
fn compartido_permite_mutar_desde_varios_duenos() {
    let a = Compartido::nuevo();
    let b = a.clone();

    a.agregar(1);
    b.agregar(2);
    a.agregar(3);

    assert_eq!(a.valores(), vec![1, 2, 3]);
    assert_eq!(b.valores(), vec![1, 2, 3]);
    assert_eq!(a.total(), 6);
    assert_eq!(b.total(), 6);
}

#[test]
fn compartido_cuenta_sus_referencias() {
    let a = Compartido::nuevo();
    assert_eq!(a.num_referencias(), 1);

    let b = a.clone();
    assert_eq!(a.num_referencias(), 2);
    assert_eq!(b.num_referencias(), 2);

    drop(b);
    assert_eq!(a.num_referencias(), 1);
}
