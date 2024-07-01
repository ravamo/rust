use std::ops::Deref;

struct CajaDoble<T> {
    valor1: T,
    valor2: T,
}

impl<T> CajaDoble<T> {
    fn new(valor1: T, valor2: T) -> CajaDoble<T> {
        CajaDoble { valor1, valor2 }
    }
}

impl<T> Deref for CajaDoble<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.valor1
    }
}

fn main() {
    let caja = CajaDoble::new(42, 24);
    
    // Accediendo al primer valor a través de Deref
    println!("Primer valor: {}", *caja); // Imprime: Primer valor: 42
}
