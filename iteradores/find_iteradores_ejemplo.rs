struct Persona { 
    edad: u8, 
    nombre: String, 
    identificador: String, 
}

fn main() { 
    
    let personas: Vec<Persona> = vec![
        Persona {
            edad: 25,
            nombre: "Juan".to_string(),
            identificador: "001".to_string() /* necesario poner to_string para convertirlo a String */
                /* sino lo hacemos estamos enviando un &str (literal de cadena) */
        },
        Persona {
            edad: 18,
            nombre: "Maria".to_string(),
            identificador: "002".to_string()
        },
        Persona {
            edad: 35,
            nombre: "Ana".to_string(),
            identificador: "003".to_string()
        },
        Persona {
            edad: 30,
            nombre: "Francisco".to_string(),
            identificador: "004".to_string()
        }
    ];

    let mut iterador_personas = personas.iter();

    let identificador_a_buscar = "001".to_string();

    let edad_minima_a_buscar = 30; 

    let personas_encontradas = iterador_personas.clone().filter(|&p| p.edad >= edad_minima_a_buscar);
    let mut cantidad_personas_encontradas = 0;

    for persona in personas_encontradas {
        println!("Personas Encontrada: "); 
        println!("Identificador: {}", persona.identificador); 
        println!("Nombre: {}", persona.nombre); 
        println!("Edad: {}", persona.edad); 
        println!("======================="); 
        cantidad_personas_encontradas += 1; 
    };

    println!("{} Personas Encontradas", cantidad_personas_encontradas); 

    /* |&p| -> clousure el cual va recorriendo a las personas hasta que coincide la condicion */
    if let Some(persona) = iterador_personas.find(|&p| p.identificador == identificador_a_buscar) {
        println!("Persona Encontrada: "); 
        println!("Identificador: {}", persona.identificador); 
        println!("Nombre: {}", persona.nombre); 
        println!("Edad: {}", persona.edad); 
    } else { 
        println!("Persona NO encontrada");
    }
}
