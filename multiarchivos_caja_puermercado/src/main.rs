/*le hacemos saber al crate root que existe este modulo*/
mod models; 
mod operaciones;
mod utileria;
mod menu; 

/*especificamos cual vamos a utilizar*/
use models::compras_model;
use menu::creacion_menu;

fn main() {
    let mut items_compra: Vec<compras_model::Item> = Vec::new();
    creacion_menu::menu_principal(&mut items_compra);

    println!("Programa Finalizado");
}
