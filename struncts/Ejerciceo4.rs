/* Tarea 1 */
struct Estudiante {
    nombre: String,
    edad: u8,
    nota: f32,
}

fn main() {
    // Instanciar la struct con datos de un estudiante
    let estudiante = Estudiante {
        nombre: String::from("Juan"),
        edad: 20,
        nota: 8.5,
    };

    // Acceder a sus campos e imprimir los valores
    println!("Nombre: {}", estudiante.nombre);
    println!("Edad: {}", estudiante.edad);
    println!("Nota: {}", estudiante.nota);
}

/* Tarea 2 */

struct Producto {
    nombre: String,
    precio: f64,
    cantidad: u32,
}

fn main() {
    // Crear una instancia de Producto
    let producto = Producto {
        nombre: String::from("Manzana"),
        precio: 0.99,
        cantidad: 10,
    };

    // Desestructurar la instancia y modificar los valores
    let Producto { nombre, precio, mut cantidad } = producto;

    // Modificar la cantidad
    cantidad = 20;

    // Imprimir los valores modificados
    println!("Nombre: {}", nombre);
    println!("Precio: {}", precio);
    println!("Cantidad: {}", cantidad);
}

/* Tarea 3 */

struct Direccion {
    calle: String,
    ciudad: String,
    pais: String,
}

struct Persona {
    nombre: String,
    edad: u8,
    direccion: Direccion,
}

fn main() {
    // Crear una instancia de Direccion
    let direccion = Direccion {
        calle: String::from("Calle Falsa 123"),
        ciudad: String::from("Ciudad Ejemplo"),
        pais: String::from("País Ejemplo"),
    };

    // Crear una instancia de Persona que incluya una instancia de Direccion
    let persona = Persona {
        nombre: String::from("María"),
        edad: 30,
        direccion: direccion,
    };

    // Imprimir los valores de la instancia de Persona, incluyendo la dirección
    println!("Nombre: {}", persona.nombre);
    println!("Edad: {}", persona.edad);
    println!("Calle: {}", persona.direccion.calle);
    println!("Ciudad: {}", persona.direccion.ciudad);
    println!("País: {}", persona.direccion.pais);
}