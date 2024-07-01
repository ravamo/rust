fn main() {
    let numero = 7;

    if numero % 2 == 0 {
        println!("El número {} es par.", numero);
    } else {
        println!("El número {} es impar.", numero);
    }

    // Uso de `else if` para evaluar múltiples condiciones
    let temperatura = 15;

    if temperatura > 30 {
        println!("Hace calor.");
    } else if temperatura > 20 {
        println!("Está templado.");
    } else if temperatura > 10 {
        println!("Hace fresco.");
    } else {
        println!("Hace frío.");
    }

    // Uso de `if` como una expresión
    let es_mayor_de_edad = true;
    let mensaje = if es_mayor_de_edad {
        "Puedes votar."
    } else {
        "No puedes votar."
    };
    println!("{}", mensaje);
}