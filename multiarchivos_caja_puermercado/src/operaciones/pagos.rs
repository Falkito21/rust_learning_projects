use crate::models::pagos_model;

    pub fn pagar(metodo_de_pago: pagos_model::MetodoPago, monto_a_pagar: f32, recibido_del_cliente: f32, tarjeta: &str) -> pagos_model::ResultadoPago {
        /*metodo_de_pago: forma de pago
         * monto_a_pagar: total a pagar
         * recibido_del_cliente: dinero del cliente, si no es efectivo, es igual al monto a pagar
         * tarjeta: numero de la misma, si es en efectivo no es necesario*/
        let resultado = match metodo_de_pago { 
            pagos_model::MetodoPago::Efectivo => pago_en_efectivo(monto_a_pagar, recibido_del_cliente),
            pagos_model::MetodoPago::Tarjeta => pago_con_tarjeta(monto_a_pagar, tarjeta),
            pagos_model::MetodoPago::TransferenciaBancaria => pago_por_transferencia_bancaria(monto_a_pagar)
        };
        resultado
    }

    fn pago_en_efectivo(monto_a_pagar: f32, recibido_del_cliente: f32) -> pagos_model::ResultadoPago { 
        println!("El pago sera en efectivo");
        let tipo_pago:pagos_model::ResultadoPago = pagos_model::ResultadoPago {
            metodo_pago: String::from("En Efectivo"),
            fue_exitoso: true, 
            cambio: recibido_del_cliente - monto_a_pagar
        };

        tipo_pago
    }

    fn pago_con_tarjeta(monto_a_pagar: f32, numero_tarjeta: &str) -> pagos_model::ResultadoPago {
        println!("El pago sera con tarjeta");
        /*procesamos lo critico a nivel de seguridad*/
        println!("Realizando el pago con el servicio de la tarjeta");
        println!("Monto a pagar: {}", monto_a_pagar); println!("Tarjeta: {}", numero_tarjeta);
        /*simulamos resultado*/
        let resultado_del_pago:pagos_model::ResultadoPago = pagos_model::ResultadoPago {
            metodo_pago: String::from("Tarjeta"),
            fue_exitoso: true, 
            cambio: 0.0,
        };

        resultado_del_pago
    }

    fn pago_por_transferencia_bancaria(monto_a_pagar: f32) -> pagos_model::ResultadoPago { 
        println!("El pago sera por transeferencia"); 

        println!("Realizando las conexiones y transferencias con el banco");
        println!("Monto a pagar: {}", monto_a_pagar);
        println!("Cuenta Bancaria Secreta xD: 12345678-0");

        pagos_model::ResultadoPago { 
            metodo_pago: String::from("Transferencia Bancaria"),
            fue_exitoso: true, 
            cambio: 0.0,
        } 
    }

