use multiarchivos_caja_puermercado::models::compras_model; 
use multiarchivos_caja_puermercado::models::pagos_model; 
use multiarchivos_caja_puermercado::operaciones::compras; 
use multiarchivos_caja_puermercado::operaciones::pagos; 

/*como estamos en otro folder que no es un modulo no es necesario usar #[cfg(test)]*/
#[test]
fn cliente_paga_con_tarjeta_de_credito() { 
    /*creamos array de items*/
    let mut items_compra: Vec<compras_model::Item> = Vec::new(); 

    /*agregamos dos items*/
    let item1: compras_model::Item = compras_model::Item { 
        nombre: String::from("item 1"),
        precio_unitario: 50.00, 
        cantidad:  1.00
    };

    let item2: compras_model::Item = compras_model::Item { 
        nombre: String::from("item 2"),
        precio_unitario: 150.00, 
        cantidad:  2.00
    };

    /*agregamos los items*/
    compras::agregar_item(&mut items_compra, item1);
    compras::agregar_item(&mut items_compra, item2);

    /*verificamos que hayan solo dos items*/
    assert_eq!(compras::total_compra(&items_compra), 350.00);

    let resultado_pago = pagos::pagar(
        pagos_model::MetodoPago::Tarjeta,
        compras::total_compra(&items_compra),
        350.00,
        "12345"
    );

    let resultado_esperado = pagos_model::ResultadoPago { 
        metodo_pago: String::from("Tarjeta"), 
        fue_exitoso: true, 
        cambio: 0.0
    }; 

    assert_eq!(resultado_pago.metodo_pago, resultado_esperado.metodo_pago); 
    assert_eq!(resultado_pago.fue_exitoso, resultado_esperado.fue_exitoso); 
    assert_eq!(resultado_pago.cambio, resultado_esperado.cambio);
}

#[test]
fn cliente_paga_en_efectivo_y_recibira_cambio() { 
    /*creamos nuestro array de items*/
    let mut items_compra: Vec<compras_model::Item> = Vec::new(); 

    let item1: compras_model::Item = compras_model::Item {
        nombre: String::from("item 1"), 
        precio_unitario: 50.00, 
        cantidad: 1.00
    };

    let item2: compras_model::Item = compras_model::Item {
        nombre: String::from("item 2"), 
        precio_unitario: 150.00, 
        cantidad: 1.00
    };

    let item3: compras_model::Item = compras_model::Item {
        nombre: String::from("item 3"), 
        precio_unitario: 150.00, 
        cantidad: 1.00
    };

    compras::agregar_item(&mut items_compra, item1);
    compras::agregar_item(&mut items_compra, item2);
    compras::agregar_item(&mut items_compra, item3);

    assert_eq!(items_compra.len(), 3);

    compras::quitar_item(&mut items_compra, 0);

    assert_eq!(items_compra.len(), 2);

    assert_eq!(compras::total_compra(&items_compra), 300.00);


    let resultado_pago = pagos::pagar(
        pagos_model::MetodoPago::Efectivo, 
        compras::total_compra(&items_compra),
        350.00,
        ""
    );

    let resultado_esperado = pagos_model::ResultadoPago {
        metodo_pago: String::from("En Efectivo"),
        fue_exitoso: true, 
        cambio: 50.0
    };

    assert_eq!(resultado_pago.metodo_pago, resultado_esperado.metodo_pago);
    assert_eq!(resultado_pago.fue_exitoso, resultado_esperado.fue_exitoso);
    assert_eq!(resultado_pago.cambio, resultado_esperado.cambio);
    
}


