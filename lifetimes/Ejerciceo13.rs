fn mayor<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let num1 = 5;
    let num2 = 10;

    // Llamada a la función mayor
    let max = mayor(&num1, &num2);

    // Imprimir el resultado
    println!("El número mayor es: {}", max);
}
