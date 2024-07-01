fn main() {
    let mut contador = 0;

    // Bucle while que cuenta hasta 5
    while contador < 5 {
        println!("Contador: {}", contador);
        contador += 1;
    }

    // Uso de `while` con una condición compleja
    let mut numero = 10;
    while numero != 0 {
        println!("Número actual: {}", numero);
        numero -= 1;
    }
    println!("¡Despegue!");
}