pub mod a {
    // Estructura pública `Sa`
    pub struct Sa {
        pub x: u8,
        pub y: u8,
    }

    // Función pública `info_a`
    pub fn info_a() {
        let s = Sa {
            x: 13,
            y: 14,
        };
        println!("x: {}, y: {}", s.x, s.y);
    }

    pub mod b {
        // Importa la estructura `Sa` del módulo padre `a`
        //use super::Sa;
        /* Se recomienda usar esto por si luego necesitamos mover  los fichero */
        use crate::a::Sa;

        pub struct Sb {
            pub x: u8,
            pub y: u8,
        }
    
        // Función pública `info_b`
        pub fn info_b() {
            let sb = Sb {
                x: 13,
                y: 14,
            };
            let sa = Sa {
                x: 13,
                y: 14,
            };
            println!("Sa -> x: {}, y: {}", sa.x, sa.y);
            println!("Sb -> x: {}, y: {}", sb.x, sb.y);
        }
    }
}

fn main() {
    // Llama a la función `info_a` del módulo `a`
    a::info_a();

    // Llama a la función `info_b` del módulo `b` dentro de `a`
    a::b::info_b();
}

