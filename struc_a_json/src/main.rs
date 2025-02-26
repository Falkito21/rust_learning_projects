use serde_json; 
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Libro {
    titulo: String, 
    total_paginas: u32,
    autores: Vec<String>, 
    generos: Vec<String>, 
    precios: Vec<Precio>
}

#[derive(Serialize, Deserialize, Debug)]
struct Precio {
    precio: f32, 
    tipo: String, 
    moneda: String,
}

fn main() {
   let libro = Libro {
        titulo: String::from("The Pragmatic Programmer"),
        total_paginas: 320,
        autores: vec![
            String::from("David Thomas"),
            String::from("Andrew Hunt")
        ],
        generos: vec![
            String::from("Programming"),
            String::from("IT")
        ],
        precios: vec![
            Precio {
                precio: 35.00,
                tipo: String::from("Tapa Dura"),
                moneda: String::from("USD")
            },
            Precio {
                precio: 15.00,
                tipo: String::from("digital"),
                moneda: String::from("USD")
            }
        ]
    }; 

   //al tener el control del json podemos usar unwrap sino deberiamos controlar el error
   let libro_convertido_json = serde_json::to_string(&libro).unwrap();
   println!("{}", libro_convertido_json);
}
