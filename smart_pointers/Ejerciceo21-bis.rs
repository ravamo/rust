struct MiCaja {
    valor: i32,
}

struct CajaCaja(MiCaja);

impl Deref for CajaCaja {
    type Target = MiCaja;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CajaCaja {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CajaCaja {
    fn new(valor: i32) -> CajaCaja {
        CajaCaja(MiCaja { valor })
    }
}

fn main() {
    let mut caja = CajaCaja::new(42);
    
    // Accediendo al valor interno de forma inmutable
    println!("Valor interno: {}", *caja); // Imprime: Valor interno: 42
    
    // Modificando el valor interno a través de la desreferenciación mutable
    *caja = MiCaja { valor: 24 };
    println!("Nuevo valor interno: {}", *caja); // Imprime: Nuevo valor interno: 24
}
