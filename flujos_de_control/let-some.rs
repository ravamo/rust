fn main(){
    let algun_numero: Option<i32> = Some(100);
    // forma 1
    match algun_numero {
        Some(numero) => println!("numero valido {}",numero),
        None => println!("numero no valido"),
    }
    
    // forma 2
    if let Some(numero)= algun_numero {
       println!("numero valido {}",numero);
    } else {
        println!("numero no valido");
    }
    
    // rust 1.65
    let Some(numero) = algun_numero else {
        panic!("numero no es valido");
    };
    
    println!("numero valido {}",numero);
}