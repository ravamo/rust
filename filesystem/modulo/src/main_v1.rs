// Define un módulo `a`
/*
Por que nos dice por que aunque la funciones 
es publica el modulo no lo es, como nosotros estamos
definiendo este modulo en el misom fichero el main lo podemos usar aunque no sea publico

Si tuvieramos que usarlo como libreia tendiramos que hacerlo como pub

*/

pub mod a {
    // Estructura pública `Sa`
     struct Sa {
        pub x: u8,
        pub y: u8,
    }

    // Función pública `info_a`
     fn info_a() {
        let s = Sa {
            x: 13,
            y: 14,
        };
        println!("x: {}, y: {}", s.x, s.y);
    }
}

fn main() {
    // Importa la estructura `Sa` del módulo `a`
    use a::info_a;
    
    // Crea una instancia de `Sa`
    let s = Sa {
        x: 13,
        y: 14,
    };

    // Imprime los valores de `x` y `y`
    println!("x: {}, y: {}", s.x, s.y);

    // Llama a la función `info_a` usando la ruta completa
    a::info_a();
}
