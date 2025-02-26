use crate::models::compras_model::Item;
use crate::operaciones::compras;
use crate::utileria::lectura_consola;

pub fn manejar_agregar_item(items_compra: &mut Vec<Item>) {
    println!("Escribe los detalles del Item");
    println!("NOMBRE");
    let nombre = lectura_consola::leer_string();

    println!("CANTIDAD");
    let cantidad = lectura_consola::leer_f32();

    println!("PRECIO UNITARIO");
    let precio_unitario = lectura_consola::leer_f32();

    /*creamos el item con la estructura del item importado*/
    let item: Item = Item {
        nombre, 
        precio_unitario, 
        cantidad
    };

    /*agregamos el item a la compra*/
    compras::agregar_item(items_compra, item);
    println!("Item agregado");
}

pub fn manejar_quitar_item(items_compra: &mut Vec<Item>) {
    println!("En construcciona quitar items");
    compras::mostrar_item(items_compra);
    println!("Indique el numero del item a eliminar");
    
    /*obtenemos el id a eliminar*/
    let indice = lectura_consola::leer_usize();

    /*elimino el item indicado utilizando la funcion dentro de modulo de compra*/
    compras::quitar_item(items_compra, indice);
    println!("Item Eliminado");

}


