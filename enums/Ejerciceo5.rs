// Parte 1
enum Estado {
    Activo,
    Inactivo,
    Pendiente,
}

fn main() {
    // Crear una instancia del enum
    let estado = Estado::Activo;

    // Usar el enum en una sentencia match
    match estado {
        Estado::Activo => println!("El estado es Activo."),
        Estado::Inactivo => println!("El estado es Inactivo."),
        Estado::Pendiente => println!("El estado está Pendiente."),
    }
}


// Parte 2
enum Mensaje {
    Texto(String),
    Numero(i32),
    Estado(Estado),
}

fn main() {
    // Crear instancias del enum con diferentes variantes
    let mensaje1 = Mensaje::Texto(String::from("Hola"));
    let mensaje2 = Mensaje::Numero(42);
    let mensaje3 = Mensaje::Estado(Estado::Inactivo);

    // Usar el enum en una sentencia match
    match mensaje1 {
        Mensaje::Texto(texto) => println!("Mensaje de texto: {}", texto),
        Mensaje::Numero(numero) => println!("Mensaje numérico: {}", numero),
        Mensaje::Estado(estado) => match estado {
            Estado::Activo => println!("Estado: Activo"),
            Estado::Inactivo => println!("Estado: Inactivo"),
            Estado::Pendiente => println!("Estado: Pendiente"),
        },
    }
}

// Parte 3
struct Estudiante {
    nombre: String,
    edad: u8,
    nota: f32,
}

enum Resultado {
    Exito(Estudiante),
    Error(String),
}

fn main() {
    // Crear instancias de Estudiante
    let estudiante1 = Estudiante {
        nombre: String::from("Juan"),
        edad: 20,
        nota: 8.5,
    };

    // Crear una instancia del enum con la variante Exito
    let resultado1 = Resultado::Exito(estudiante1);

    // Crear una instancia del enum con la variante Error
    let resultado2 = Resultado::Error(String::from("Fallo en la inscripción"));

    // Usar el enum en una sentencia match
    match resultado1 {
        Resultado::Exito(estudiante) => {
            println!("Estudiante inscrito:");
            println!("Nombre: {}", estudiante.nombre);
            println!("Edad: {}", estudiante.edad);
            println!("Nota: {}", estudiante.nota);
        },
        Resultado::Error(mensaje) => println!("Error: {}", mensaje),
    }

    match resultado2 {
        Resultado::Exito(estudiante) => {
            println!("Estudiante inscrito:");
            println!("Nombre: {}", estudiante.nombre);
            println!("Edad: {}", estudiante.edad);
            println!("Nota: {}", estudiante.nota);
        },
        Resultado::Error(mensaje) => println!("Error: {}", mensaje),
    }
}