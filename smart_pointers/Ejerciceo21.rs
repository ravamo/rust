struct Archivo {
    nombre: String,
    // Otros campos necesarios para manejar el archivo
}

impl Drop for Archivo {
    fn drop(&mut self) {
        println!("Cerrando archivo: {}", self.nombre);
        // Lógica para cerrar el archivo
    }
}

fn main() {
    let archivo = Archivo { nombre: String::from("data.txt") };
    println!("Archivo abierto");
}
