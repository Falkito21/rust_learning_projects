struct Venta { 
    total_sin_descuento: f32, 
    total_con_descuento: f32, 
}

fn main() { 
    let totales = vec![25.35, 12.25, 10.15, 20.55]; 

    let estructura_totales_con_descuento: Vec<Venta> = totales.iter().map(
        /* como funciona esto ?? */
        |elemento| Venta { 
            total_sin_descuento: *elemento, 
            total_con_descuento: *elemento - (*elemento * 0.1)
        }
    ).collect();
    /* necesitamos utilizar collect porque los iteradores son lazy
     * (no se ejecutara a menos que sea explicitamente requerido) */
    for venta in estructura_totales_con_descuento { 
        println!("total: ${} - total con descuento: ${}", venta.total_sin_descuento, venta.total_con_descuento);
    }
}

