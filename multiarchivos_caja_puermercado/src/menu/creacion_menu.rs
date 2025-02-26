use crate::models::compras_model;
use crate::menu::manejadores::{principal_menu, pagos_menu, compras_menu};

use crate::operaciones::compras;

use crate::utileria::lectura_consola;


pub fn menu_principal(items_compra: &mut Vec<compras_model::Item>) {
    
    loop {
        principal_menu::mostrar_menu();

        let opcion_seleccionada = lectura_consola::leer_usize();

        match opcion_seleccionada { 
            1 => compras_menu::manejar_agregar_item(items_compra),
            2 => compras_menu::manejar_quitar_item(items_compra),
            3 => compras::mostrar_item(&items_compra),
            4 => println!("TOTAL A PAGAR: ${}", compras::total_compra(&items_compra)),
            5 => pagos_menu::manejar_realizar_pago(items_compra), 
            6 => break,
            _ => println!("OPCION INVALIDA!!")
        }
    }
    println!("Programa Finalizado");
}
