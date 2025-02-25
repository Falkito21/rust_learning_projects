use std::io::stdin; //para leer los inputs

//libreria
use manejo_inputs;

fn main() {
    manejo_inputs::mostrar_menu();
     
    //leemos la opreacion a realizar por el usuario 

    let numero_operacion: i32 = manejo_inputs::obtener_input("Escribe el numero de la operacion a realizar: ");

    let a: i32 = manejo_inputs::obtener_input("Primer Digito: ");
    let b: i32 = manejo_inputs::obtener_input("Segundo Digito: ");
    
    let resultado = manejo_inputs::obtener_resultados(numero_operacion, a, b);
    
    //imprimimos el resultado en pantalla 
    println!("Resultado: {}", resultado);


    
}
