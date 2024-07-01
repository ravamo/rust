// Parte 1
fn main() {
    // Asignamos un valor Some(42) a la variable `numero`
    let numero = Some(42);

    // Usamos `if let` para imprimir el valor si está presente
    if let Some(valor) = numero {
        println!("El valor es: {}", valor);
    }
}

// Parte 2
fn main() {
    // Creamos una variable `resultado` con valor Ok("¡Éxito!")
    let resultado: Result<&str, &str> = Ok("¡Éxito!");

    // Usamos `if let` para imprimir el mensaje si es Ok
    if let Ok(mensaje) = resultado {
        println!("Mensaje de éxito: {}", mensaje);
    }
}

// Parte 3
fn main() {
    // Creamos una lista de palabras
    let palabras = vec!["hello", "world", "rust"];

    // Usamos `while let` para imprimir cada palabra hasta que la lista esté vacía
    let mut iterador = palabras.iter();
    while let Some(palabra) = iterador.next() {
        println!("Palabra: {}", palabra);
    }
}

// Parte 4
fn main() {
    // Creamos un vector con números del 1 al 5
    let numeros = vec![1, 2, 3, 4, 5];

    // Usamos `while let` para sumar todos los números del vector hasta que esté vacío
    let mut iterador = numeros.iter();
    let mut suma = 0;
    while let Some(&numero) = iterador.next() {
        suma += numero;
    }

    println!("La suma de los números es: {}", suma);
}
