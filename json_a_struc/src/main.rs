   //UTILIZAREMOS EL FRAMEWORK SERDE
    //framework que serializa y deserializa eficientemente estructuras de datos de rust 
    //UTILIZAREMOS MACROS DERIVES
    //es lo que se conoce como decorador (activamos la macro de serde en el .toml)
    //estas macros me permitiran convertir el json que recibimos en estructuras de datos
    //la forma de funcionar es la siguiente : 
    //(Deserialize) Busca nombres de atributos en el json que coincidan con la struc
    //y que esos atributos contengan el mismo tipo de dato en ambos lados
use serde_json::Result; 
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Precio {
    precio: f32, 
    tipo: String, 
    moneda: String, 
}

#[derive(Serialize, Deserialize, Debug)]
struct Libro {
    titulo: String, 
    total_paginas: u32,
    autores: Vec<String>,
    generos: Vec<String>,
    precios: Vec<Precio>,
}
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

let libro_parsed: Libro = serde_json::from_str(libro)?;

println!("libro Procesado por rust");

println!("Titulo del libro: {}", libro_parsed.titulo);

const DESCUENTO:f32 = 10.00;

println!("PRECIOS");
for precio in libro_parsed.precios {
    println!("Tipo: {}", precio.tipo);
    println!("precio: {}", precio.precio);
    println!("Descuento del 10%: {}", (precio.precio - (precio.precio * (DESCUENTO / 100.00))));
    println!("Moneda: {}", precio.moneda);
}

Ok(())
    
}
