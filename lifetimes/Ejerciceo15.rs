fn dame_ownership(s: String) -> &str {
    // Devolvemos una referencia al contenido del String
    &s
}

fn main() {
    let mi_string = String::from("Hola, mundo!");

    // Llamamos a la función dame_ownership y obtenemos una referencia
    let referencia = dame_ownership(mi_string);

    // Intentamos imprimir el contenido referenciado
    println!("El contenido del String es: {}", referencia);
}
