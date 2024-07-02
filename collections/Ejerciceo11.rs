use std::collections::HashSet;
use std::io;
use std::io::Write;

fn main() {
    println!("Ingrese una serie de palabras separadas por espacios:");

    // Leer la entrada del usuario
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer línea");

    // Crear un HashSet para almacenar palabras únicas
    let mut palabras_set: HashSet<String> = HashSet::new();

    // Separar las palabras por espacios y agregarlas al HashSet
    for palabra in input.split_whitespace() {
        palabras_set.insert(palabra.to_string());
    }

    // Obtener el número de palabras únicas usando el método len() del HashSet
    let num_palabras_unicas = palabras_set.len();

    // Imprimir el número de palabras únicas
    println!("Número de palabras únicas ingresadas: {}", num_palabras_unicas);
}
