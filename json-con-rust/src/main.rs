//crate que nos permite el manejo de JSON 
use serde_json::{Result, Value};
fn main() -> Result<()> {
    let libro = r#"{
        "titulo": "The Pragmatic Programmer",
        "autores": [
            "David Thomas",
            "Andrew Hunt"
        ],
        "total_paginas": 352,
        "generos": [
            "programacion",
            "ingenieria",
            "educacion"
        ],
        "precios": [
            {
                "tipo": "digital",
                "precio": 15.00,
                "moneda": "USD"
            },
            {
                "tipo": "tapa dura",
                "precio": 35.50,
                "moneda": "USD"
            }
        ]
    }"#;

    //procesamos el JSON 
    //convertimos el string que representa nuestro json a un objeto tipo Value
    let libro_parsed: Value = serde_json::from_str(libro)?;
    const DESCUENT: f64 = 10.00;
    //as_str: para que muestre el dato sin comillas
    println!("Titulos de los libros: {}", libro_parsed["titulo"].as_str().unwrap());
    for precio in libro_parsed["precios"].as_array().unwrap(){
        let precio_regular = precio["precio"].as_f64().unwrap();
        let precio_descuento = precio_regular - (precio_regular * (DESCUENT/100.00));

        println!("Tipo: {}", precio["tipo"].as_str().unwrap());
        println!("Precio Regular: {} {}", precio_regular, precio["moneda"]);
        println!("Precio con -10%: {} {}", precio_descuento, precio["moneda"]);
    }

    Ok(())
}

