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

    pub mod b ;
    // se podri hacer eso 
    pub use b::info_b;
    

