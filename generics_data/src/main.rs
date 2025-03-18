fn numeros_mayores_a<T: std::cmp::PartialOrd> (lista: &[T], numero: T) -> Vec<&T> {
    let mut resultado: Vec<&T> = vec![];
    for elemento in lista {
        if *elemento > numero {
            resultado.push(elemento);
        }
    }
    resultado
}
fn main() {
    let lista = [10, 12, 5, 6, 7];
    let numero = 9; 

    let resultado = numeros_mayores_a(&lista, numero); 

    println!("Resultados obtenidos: "); 
    for elemento in resultado { 
        println!("{}", elemento);
    }

    let lista2 = [11, 21, 3, 15, 1]; 
    let numero2 = 6; 

    let resultado2 = numeros_mayores_a(&lista2, numero2); 

    println!("Resultados obtenidos para la segunda lista: ");

    for elemento in resultado2 {
        println!("{}", elemento); 
    }

    let lista_f32 = [1.4, 5.67, 5.25, 8.90, 10.55]; 
    let numero_f32 = 5.5; 

    let resultado_f32 = numeros_mayores_a(&lista_f32, numero_f32); 

    println!("Resultados obtenidos para la lista f32 utilizando generics"); 
    for elemento in resultado_f32 {
        println!("{}", elemento); 
    }
}


