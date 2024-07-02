/* DSL 
funciones vs macros
tu puedes implementar un trait de un tipo y en una funcion no puede que 
es llamada en tiempo de ejecucion y el trait necesita ser invocado en tiempo de compilacion 
es mucho mas flexible
*/

macro_rules! tres  {
    ($x:expr) => {
        $x+1
    };
}

macro_rules! mi_vector {
    ($($x:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}



macro_rules! nueva_funcion {
    ($name:ident) => {
        fn$name(){
            println!("hola soy {:?}()",stringify!($name));
        }
    };
}
fn main() {
    println!("{}", tres!(100));
    let vector = mi_vector![1,2,3,4];
    print!("Vector: {:?}",vector);

    /*
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    */

    nueva_funcion!(saludar);
    saludar();
    nueva_funcion!(rust);
    rust();
    nueva_funcion!(2);
}