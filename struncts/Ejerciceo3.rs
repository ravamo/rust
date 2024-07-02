// Definir la struct Vehiculo con los campos marca, modelo y anio
struct Vehiculo {
    marca: String,
    modelo: String,
    anio: u16,
}

fn main() {
    // Crear una instancia de Vehiculo y asignar valores a sus campos
    let mi_vehiculo = Vehiculo {
        marca: String::from("Toyota"),
        modelo: String::from("Corolla"),
        anio: 2020,
    };

    // Imprimir los valores de los campos del Vehiculo
    println!("Marca: {}", mi_vehiculo.marca);
    println!("Modelo: {}", mi_vehiculo.modelo);
    println!("Año: {}", mi_vehiculo.anio);
}