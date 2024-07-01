use std::ops::{Deref, DerefMut};

struct MiPuntero<T> {
    valor: T,
}

impl<T> MiPuntero<T> {
    fn new(valor: T) -> MiPuntero<T> {
        MiPuntero { valor }
    }
}

impl<T> Deref for MiPuntero<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.valor
    }
}

impl<T> DerefMut for MiPuntero<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.valor
    }
}

fn main() {
    let mut puntero = MiPuntero::new(42);
    
    // Accediendo al valor interno de forma inmutable
    println!("Valor interno: {}", *puntero); // Imprime: Valor interno: 42
    
    // Modificando el valor interno a través de la desreferenciación mutable
    *puntero = 24;
    println!("Nuevo valor interno: {}", *puntero); // Imprime: Nuevo valor interno: 24
}
