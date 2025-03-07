fn main() { 
    let lista1 = vec!["primer elemento de la lista1", "segundo elemento de la lista1", "tercer elemento de la lista1"];
    let lista2 = vec!["primer elemento de la lista2", "segundo elemento de la lista2", "tercer elemento de la lista2"]; 

    let iterador_lista1 = lista1.iter(); 
    let iterador_lista2 = lista2.iter(); 

    let mut lista_unificada = iterador_lista1.zip(iterador_lista2);

    while let Some((elemento_lista1, elemento_lista2)) = lista_unificada.next() {
        println!("[{}, {}]", elemento_lista1, elemento_lista2);
    }
}
