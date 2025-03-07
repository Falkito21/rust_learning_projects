fn main() {
    let elementos = vec![10,25, 50, 100, 201, 23]; 

    let resultado: i32 = elementos.iter().sum();

    println!("resultado suma = {}", resultado); 
}
