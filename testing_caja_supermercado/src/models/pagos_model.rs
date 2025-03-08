pub enum MetodoPago {
    Efectivo, 
    Tarjeta, 
    TransferenciaBancaria, 
}

pub struct ResultadoPago { 
    pub metodo_pago: String, 
    pub fue_exitoso: bool, 
    pub cambio: f32,
}
