//Define un trait `Describible` con un método `descripcion` que devuelva una cadena (`&str`). Implementa este trait para la struct `Producto`.
struct Producto {
    nombre: String,
    precio: f64,
    cantidad: u32,
}

trait Describible {
    fn descripcion(&self) -> String;
}

impl Describible for Producto {
    fn descripcion(&self) -> String {
        format!("Producto: {}, Precio: ${}, Cantidad: {}", self.nombre, self.precio, self.cantidad)
    }
}

fn main() {
    let producto = Producto {
        nombre: String::from("Manzana"),
        precio: 0.99,
        cantidad: 10,
    };

    println!("{}", producto.descripcion());
}

//Define un trait `EstadoInfo` con un método `estado_actual` que devuelva una cadena (`&str`). Implementa este trait para el enum `Estado`.

enum Estado {
    Activo,
    Inactivo,
    Pendiente,
}

trait EstadoInfo {
    fn estado_actual(&self) -> &'static str;
}

impl EstadoInfo for Estado {
    fn estado_actual(&self) -> &'static str {
        match self {
            Estado::Activo => "Activo",
            Estado::Inactivo => "Inactivo",
            Estado::Pendiente => "Pendiente",
        }
    }
}

fn main() {
    let estado = Estado::Activo;

    println!("El estado actual es: {}", estado.estado_actual());
}

//Define un trait `DetallesPedido` con un método `detalles` que devuelva una cadena (`String`). Implementa este trait para el enum `Pedido`, que usa la struct `Producto`.
struct Producto {
    nombre: String,
    precio: f64,
    cantidad: u32,
}

enum Pedido {
    Nuevo(Producto),
    Enviado(Producto),
    Cancelado(Producto, String),
}

trait DetallesPedido {
    fn detalles(&self) -> String;
}

impl DetallesPedido for Pedido {
    fn detalles(&self) -> String {
        match self {
            Pedido::Nuevo(producto) => format!("Nuevo pedido: {} - ${} x{}", producto.nombre, producto.precio, producto.cantidad),
            Pedido::Enviado(producto) => format!("Pedido enviado: {} - ${} x{}", producto.nombre, producto.precio, producto.cantidad),
            Pedido::Cancelado(producto, motivo) => format!("Pedido cancelado: {} - ${} x{}, Motivo: {}", producto.nombre, producto.precio, producto.cantidad, motivo),
        }
    }
}

fn main() {
    let producto = Producto {
        nombre: String::from("Manzana"),
        precio: 0.99,
        cantidad: 10,
    };

    let pedido = Pedido::Nuevo(producto);

    println!("{}", pedido.detalles());
}