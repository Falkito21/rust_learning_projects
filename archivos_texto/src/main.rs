//le indicamos a rust que usaremos el modulo File(nativo de rust)
use std::fs::{OpenOptions, File};
use std::io::prelude::*;

//agregamos modulo Buffer para lectura
use std::io::BufReader;

//la funcion retornara un Resultado del paquete io de tipo funcion
fn main() -> std::io::Result<()>{
    /**
     const CONTENIDO: &str = "Bacon ipsum dolor amet salami t-bone picanha jowl,
turducken pastrami fatback. Pig beef pancetta corned beef andouille rump ground round.
Sausage pork leberkas, drumstick turkey shankle brisket ball tip salami. Shoulder beef
pastrami venison bacon rump sirloin. Filet mignon venison flank ground round spare
ribs boudin shankle buffalo hamburger chislic. Kevin kielbasa corned beef tail burgdoggen
biltong salami, leberkas porchetta.";
*/
    //creamos el archivo
    //el '?' indica que si crea el archivo, el programa continue con un resultado tipo io
    //sino logra crearlo el programa termina con un error
    //la referencia de su creacion la vamos a guardar en una variable mutable
    /** let mut archivo = File::create("ejemplo2.txt")?; */

    //creamos el archivo
    /**
    let mut archivo = OpenOptions::new()
        .append(true)//abrimos el archivo en modo append (simpre agrega, no re escribe)
        .create(true)//si el archivo no exist. lo crea  
        .open("ejemplo.txt")?;
    */

    //ahora no utilizamos mut para almacenar el archivo (mas optimo)
    let archivo = File::open("poema.txt")?;
    
    //creamos un buffer con el archivo 
    let mut buf_reader = BufReader::new(archivo);
    //let mut archivo = File::open("poema.txt")?;
    let mut contenido = String::new();


    buf_reader.read_to_string(&mut contenido)?;

    //leemos el contenido y lo guardamos en una variable que le indiquemos 
    //archivo.read_to_string(&mut contenido)?;
    
    //insertamos el texto en el archivo en formato byte
    //el programa cerrara el archivo por nosotros,por el scope o porque termina el prog.
    //no es necesario ser explicito en que queremos cerrar el archivo
    //write_all -> sobre escribe los cambios
    /**
    archivo.write_all(CONTENIDO.as_bytes())?;
    */

    println!("Contenido del archivo");
    

    //leemos el contenido completo
    println!("{}", contenido);

    //leemos el contenido linea por linea
    println!("leemos el contenido linea por linea");
    let mut contador = 0;
    for linea in contenido.lines(){
        contador += 1;
        if linea.len() != 0 {
            let mut linea_modificada = format!("linea Nro {}: ", contador);
            linea_modificada.push_str(&linea);
            println!("{}", linea_modificada);
        }
    }

    
    //retorno que espera el main
    Ok(())
}
