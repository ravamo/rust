fn main() {
    // Iterando sobre un rango de números
    for numero in 1..5 {
        println!("Número: {}", numero);
    }

    // Iterando sobre un array
    let colores = ["rojo", "verde", "azul", "amarillo"];
    for color in colores.iter() {
        println!("Color: {}", color);
    }

    // Iterando con índices
    for (indice, &color) in colores.iter().enumerate() {
        println!("Índice: {}, Color: {}", indice, color);
    }
}