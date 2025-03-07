fn main() {
    let list = vec!["Primer Elemento", "Segundo Elemento", "Tercer Elemento"];
    /* enumerate() genera el iterador */
    let mut mi_iter = list.iter().enumerate();

    /* recorre nuetro iterador hasta encontrar algun None */
    while let Some((indice, valor)) = mi_iter.next() {
        println!("{} - {}", indice, valor);
    }
}
