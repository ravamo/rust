pub mod a ;
fn main() {
    // Llama a la función `info_a` del módulo `a`
    a::info_a();

    // Llama a la función `info_b` del módulo `b` dentro de `a`
    a::b::info_b();
    a::info_b();
}

