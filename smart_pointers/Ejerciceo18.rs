// Parte 1
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

// Parte 2
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

// Parte 3
use std::ops::{Deref, DerefMut};

struct MiCaja<T> {
    valor: T,
}

struct CajaCaja<T> {
    caja_interna: MiCaja<T>,
}

impl<T> MiCaja<T> {
    fn new(valor: T) -> MiCaja<T> {
        MiCaja { valor }
    }
}

impl<T> CajaCaja<T> {
    fn new(valor: T) -> CajaCaja<T> {
        CajaCaja {
            caja_interna: MiCaja::new(valor),
        }
    }
}

impl<T> Deref for CajaCaja<T> {
    type Target = MiCaja<T>;

    fn deref(&self) -> &Self::Target {
        &self.caja_interna
    }
}

impl<T> DerefMut for CajaCaja<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.caja_interna
    }
}

fn main() {
    let mut caja_caja = CajaCaja::new(42);
    
    // Accediendo al valor interno de forma inmutable
    println!("Valor interno: {}", caja_caja.valor); // Imprime: Valor interno: 42
    
    // Modificando el valor interno a través de la desreferenciación mutable
    caja_caja.valor = 24;
    println!("Nuevo valor interno: {}", caja_caja.valor); // Imprime: Nuevo valor interno: 24
}
