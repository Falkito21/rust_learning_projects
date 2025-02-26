use crate::models::compras_model::Item;
use crate::models::pagos_model;

use crate::operaciones::pagos;

pub fn agregar_item(items_compra: &mut Vec<Item>, item: Item) {
        items_compra.push(item);
    }

    pub fn quitar_item(items_compra: &mut Vec<Item>, indice: usize) {
        items_compra.remove(indice);
    }

    pub fn mostrar_item(items_compra: &Vec<Item>) {
        /* iter(): me muestra el valor; enumerate(): el indice de cada valor*/
        for (i, item) in items_compra.iter().enumerate() {
            let subtotal = item.cantidad * item.precio_unitario; 
            println!("[{}]. {} - Cantidadd: {} - Precio U: ${} - Subtotal: ${}", i, item.nombre, item.cantidad, item.precio_unitario, subtotal);
        }
    }

    pub fn total_compra(items_compra: &Vec<Item>) -> f32 {
        let mut total: f32 = 0.0;
        for item in items_compra {
            total = total + (item.cantidad * item.precio_unitario);
        }
        let y = 10i32.pow(2) as f32; 
        total = (total * y).round() / y; 
        total
    }

    pub fn pagar_compra(metodo_de_pago: pagos_model::MetodoPago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> pagos_model::ResultadoPago {
        /*invocamos las funciones privadas de pago, lo podemos hacer porq eta dentro del alcance*/ 
        let resultado = pagos::pagar(metodo_de_pago, monto_a_pagar, recibido_del_cliente, tarjeta);
        resultado
    }
        
