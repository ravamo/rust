/*
1. Se define una constante PI de tipo f64 con el valor 3.141592.
2. Se imprime el valor de PI.
3. Se intenta cambiar el valor de PI, lo cual causará un error porque las constantes no pueden ser modificadas.
4. La línea está comentada para evitar el error durante la ejecución.
5. Se define una variable mutable radio de tipo f64 con el valor 5.0.
6. Se calcula el área de un círculo usando la fórmula PI * radio * radio y se asigna el resultado a la nueva variable area.
7. Se imprime el valor del área del círculo.
*/
fn main() {
    // Paso 1: Define una constante PI de tipo f64 con el valor 3.141592.
    const PI: f64 = 3.141592;
    
    // Paso 2: Imprime el valor de PI.
    println!("El valor de PI es: {}", PI);
    
    // Paso 3: Intenta cambiar el valor de PI y observa el error.
    // PI = 3.14; // Esta línea causará un error porque PI es una constante.
    
    // Paso 4: Define una variable mutable radio de tipo f64 con el valor 5.0.
    let mut radio: f64 = 5.0;
    
    // Paso 5: Calcula el área de un círculo usando PI y radio y asigna el resultado a una nueva variable área.
    let area: f64 = PI * radio * radio;
    
    // Paso 6: Imprime el área del círculo.
    println!("El área del círculo es: {}", area);
}