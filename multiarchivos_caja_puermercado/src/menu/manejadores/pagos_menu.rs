use crate::models::compras_model;
use crate::models::pagos_model; 

use crate::operaciones::compras;

use crate::utileria::lectura_consola;

pub fn manejar_realizar_pago(items_compra: &mut Vec<compras_model::Item>) {
    println!("En construccion realizar pago");
    let monto_a_pagar = compras::total_compra(items_compra);

    println!("Monto a Pagar ${}", monto_a_pagar);
    println!("Selecciona el metodo de pago");
    println!("1. En Efectivo");
    println!("2. Tarjeta");
    println!("3. Transferencia Bancaria");

    let mut recibido_del_cliente = 0.0;
    let mut tarjeta = String::from("N/A");

    let opcion_seleccionada = lectura_consola::leer_usize();

    let metodo_de_pago = match opcion_seleccionada {
        1 => pagos_model::MetodoPago::Efectivo,
        2 => pagos_model::MetodoPago::Tarjeta,
        3 => pagos_model::MetodoPago::TransferenciaBancaria, 
        _ => pagos_model::MetodoPago::Efectivo 
    };

    if opcion_seleccionada == 1 {
        println!("Monto recibido del cliente");
        recibido_del_cliente = lectura_consola::leer_f32();
    }
    if opcion_seleccionada == 2 {
        println!("Numero de Tarjeta: ");
        /*solicitamos el numero de la tarjeta*/
        tarjeta = lectura_consola::leer_string();
    }

    let resultado_del_pago: pagos_model::ResultadoPago = compras::pagar_compra(metodo_de_pago, monto_a_pagar, recibido_del_cliente, &tarjeta);

    if resultado_del_pago.fue_exitoso{
        println!("El pago fue exitoso");
        println!("Metodo de Pago: {}", resultado_del_pago.metodo_pago);
        println!("Cambio: ${}", resultado_del_pago.cambio);
    }else{
        println!("Hubo un problema al procesar el pago, reintentelo nuevamente");
    }
}
