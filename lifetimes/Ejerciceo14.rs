struct Persona<'a> {
    nombre: &'a str,
}

impl<'a> Persona<'a> {
    fn obtener_nombre(&self) -> &'a str {
        self.nombre
    }
}

fn main() {
    // Creamos una instancia de Persona con un nombre válido
    let nombre = "Alice";
    let persona = Persona { nombre };

    // Llamamos a la función obtener_nombre para obtener la referencia al nombre
    let nombre_persona = persona.obtener_nombre();

    // Imprimimos el nombre obtenido
    println!("El nombre de la persona es: {}", nombre_persona);
}
