// Parte1

fn main() {
    // Creamos un vector de números enteros
    let numeros = vec![1, 2, 3, 4, 5];

    // Usamos un iterador para sumar todos los elementos del vector
    let suma: i32 = numeros.iter().sum();

    // Imprimimos la suma total
    println!("La suma de los elementos es: {}", suma);
}


// Parte 2
fn main() {
    // Creamos un vector de números del 1 al 10
    let numeros: Vec<i32> = (1..=10).collect();

    // Usamos un iterador para filtrar los números pares
    let pares: Vec<i32> = numeros.iter().filter(|&num| num % 2 == 0).cloned().collect();

    // Imprimimos el vector resultante de números pares
    println!("Números pares: {:?}", pares);
}


// Parte3
fn main() {
    // Creamos un vector de números enteros
    let numeros = vec![1, 2, 3, 4, 5];

    // Usamos el método map para incrementar cada número en 1
    let incrementados: Vec<i32> = numeros.iter().map(|&num| num + 1).collect();

    // Imprimimos el vector resultante con los números incrementados
    println!("Vector incrementado: {:?}", incrementados);
}
