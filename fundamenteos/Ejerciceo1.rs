/*
1. Se declara una variable inmutable x con el valor 10.
2. Se intenta cambiar el valor de x a 20. Esto causará un error porque x es inmutable. La línea está comentada para evitar el error durante la ejecución.
3. Se declara una nueva variable y utilizando mut, lo que permite que su valor se pueda cambiar.
4. Se cambia el valor de y a 40.
5. Se imprimen los valores de x y y.

Cuando ejecutes este código sin la línea comentada, verás el error de inmutabilidad. Si eliminas la línea comentada, el programa se ejecutará correctamente, mostrando los valores de x y y.
*/

fn main() {
    // Paso 1: Declara una variable x de tipo i32 con el valor 10.
    let x: i32 = 10;
    
    // Paso 2: Intenta cambiar el valor de x a 20 y observa el error.
    // x = 20; // Esta línea causará un error porque x es inmutable.
    
    // Paso 3: Declara una nueva variable y usando la palabra clave mut y asígnale el valor 30.
    let mut y: i32 = 30;
    
    // Paso 4: Cambia el valor de y a 40.
    y = 40;
    
    // Imprime ambos valores.
    println!("El valor de x es: {}", x);
    println!("El valor de y es: {}", y);
}
