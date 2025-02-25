use std::io::stdin;
use operaciones;

pub fn mostrar_menu() {
    //mostamos menu de opciones 
    println!("OPERACIONES:");
    println!("1. Suma");
    println!("2. Resta");
    println!("3. Multiplicacion");
    println!("4. Division");
}

pub fn obtener_input(input: &str) -> i32 {

    println!("{}", input);
    let mut input_user = String::new(); 
    
    stdin().read_line(&mut input_user).unwrap();

    input_user = input_user.replace("\n", "").replace("\r", "").replace("t", "");
    
    let input_parse: i32 = input_user.parse::<i32>().unwrap();

    input_parse
    
}

pub fn obtener_resultados(opcion: i32, a: i32, b: i32) -> i32 {
    match opcion {
        1  => operaciones::suma(a, b),
        2  => operaciones::resta(a, b),
        3  => operaciones::multiplicacion(a, b),
        4  => operaciones::division(a, b),
        _  => 0

    }
}


















