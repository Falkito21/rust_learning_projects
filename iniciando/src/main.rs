/*
- programa que recibe un string 
- el programa codificara el string a base64 
- lo mostrara en pantalla
*/

use std::env; //use == import
use base64::encode; 

fn main() {
    // guarda ccada palabra dentro del vector ,del input en la terminal 
    let args: Vec<String> = env::args().collect(); //vector == array
    //accede al valor (&) de args en la posicion 1 y lo guarda en parametro
    let parametro = &args[1]; 
    println!("Parametro {}", parametro);

    //codificamos a base64 el valor que hay en la referencia &parametro
    let parametro_base64 = encode(&parametro);
    println!("Parametro {} - Base64 {}", parametro, parametro_base64);
}

//cargo run <parametro> -> para ejecutar el programa
