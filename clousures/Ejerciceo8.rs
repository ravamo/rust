
// Escribe un closure que tome dos números enteros y devuelva su suma. Luego, llama al closure con los números 3 y 4, e imprime el resultado.

fn main() {
    // Definimos el closure que suma dos números enteros
    let suma = |a: i32, b: i32| -> i32 {
        a + b
    };

    // Llamamos al closure con los números 3 y 4
    let resultado = suma(3, 4);

    // Imprimimos el resultado
    println!("El resultado de la suma es: {}", resultado);
}