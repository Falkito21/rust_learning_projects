fn main() {
    let a = vec![1, 2, 3, 4];

    /* construimos el iterador */
    /* el metodo iter() se encarga de crear un iterador */
    let mut mi_iterador = a.iter();

   
    /* se utiliza next() para conseguir el siguiente valor */
    /* se utiliza Some porque cuando el iterador llega a su fin puede retornar None */
    assert_eq!( mi_iterador.next(), Some(&1) );
    assert_eq!( mi_iterador.next(), Some(&2) );
    assert_eq!( mi_iterador.next(), Some(&3) );
    assert_eq!( mi_iterador.next(), Some(&4) );
    assert_eq!( mi_iterador.next(), None );

}

