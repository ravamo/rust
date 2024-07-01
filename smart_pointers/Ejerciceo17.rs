struct Conexion {
    id: u32,
    // Otros campos necesarios para manejar la conexión
}

impl Drop for Conexion {
    fn drop(&mut self) {
        println!("Cerrando conexión: {}", self.id);
        // Lógica para cerrar la conexión
    }
}

fn main() {
    let conexion = Conexion { id: 1 };
    println!("Conexión establecida");
}