use std::io;
use std::str::FromStr;

fn main() {
    println!("Ingrese una lista de números enteros separados por espacios:");

    // Leer la entrada del usuario
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer línea");

    // Separar los números por espacios y convertirlos a enteros
    let numeros: Vec<i32> = input
        .split_whitespace()
        .map(|s| i32::from_str(s).expect("Error al convertir a entero"))
        .collect();

    // Calcular la suma de los números usando el método sum() de los iteradores
    let suma: i32 = numeros.iter().sum();

    // Imprimir la suma
    println!("La suma de los números ingresados es: {}", suma);
}
