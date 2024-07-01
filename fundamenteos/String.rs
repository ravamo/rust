// String vs str
// stack (tamaño fijo) durante la ejecucion .
// heap informacion que crece o no .
fn main(){
    let mut saludo = "Hola mundo";
    let mut saludo1 = String::from("Hola mundo");
    
    saludo1.push_str(" Desde Rust ");
    //saludo.push_str(" Desde Rust ");
    
    // son iguales pero no Rust los genstiona de forma distinta.
    
    println!("{}", saludo1);
    println!("{}", saludo);
    
    println!("{}", saludo1.len());
    println!("{}", saludo.len());
    
    // ahora miramos la capacidad
    
    println!("{}", saludo1.capacity());
    // da error, No puede mutar no puede cambiar 
    // en tipo de compilacion sabe que guarda este texto y lo guarda al contratio que el otro que si puede crecer 
    // se guarda en el heap y eso hace que sea dinamico 
    // println!("{}", saludo.capacity());
    
    saludar(&saludo1);
    saludar2(saludo1);
    
    
    saludar(saludo);
}

// por referencia
fn saludar(saludo: &str){
    println!("{}",saludo )
}