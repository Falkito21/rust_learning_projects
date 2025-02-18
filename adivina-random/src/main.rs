//COMANDOS UTILES DE CARGO EDIT
/**
 * cargo add <crate name> : agrega la dependencia indicada
 * cargo upgrade : actualiza todas las dependencias
 * cargo rm <crate name> : quita la dependencia especificada
 * cargo set-version <version> : coloca la dependencia en la version especificada
*/
// MANEJO DE DEPENDENCIAS COMO LO HACE npm : cargo edit para rust
/**
 * cargo install cargo-edit
 * */
use std::io::stdin;
use read_line::read_line_prompt; //importamos un lector de teclado user
use rand::{thread_rng, Rng};//importamos generador de numeros random

fn main() {
    //let mut input_string = String::new();
    println!("ADIVINA EL NUMERO!!");
    let mut rng = thread_rng(); // instanciamos 
    // En rust, los rangos si toman el primer valor y excluyen el segundo 
    let numero: u8 = rng.gen_range(1..10);
    println!("Se ha generado un numero del 1 al 9");
    loop{
        let user_input = read_line_prompt("Ingresa tu numero:");
        /**
        //limpiamos el valor anterior de la variable
        input_string.clear();
        //almacenamos en la variable el valor
        //stdin().read_line(&mut input_string).unwrap();
        // si el valor es igual a 5 
        //if input_string.trim().eq("5") { 
        */
        if user_input.contains(&numero.to_string()){
            println!("ADIVINASTE!!"); //le hacemos trim para que elimine espacios en blanco
            break;
        }
        println!("Intentalo nuevamente!");
    }
    println!("Fin del programa");
}
