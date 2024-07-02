#[warn(unused_variables)]
/*fn main() {
 // tercera regla
 // x no es valida aqui
 let x = 10; // es valida aqui
}// sale del scope aqui ya no podemos usar x 
*/

fn main(){
 // paso 1
 let hola = String::from("hola mundo");
 saluda(hola);
 // paso 1.1  saluda(&hola);
 // paso 1.1 si pongo  println!("{}",hola); sale error
 let x= 10;
 let y = 20;
 
 suma (x,y); // se hace una copia de la variable x e y , 
 //ya vimos que contienen diferencte direccion de memoria
 println!("{},{}",y,x);
 // paso 1.2  println!("{:p},{:p}",&y,&x);
  
} 
// la variable x e y son eliminadas . 
//Con la variable hola no pasa nada  devido a que fue movida a la funciona saludar

// paso 1.1 fn saluda(hola:&str){
fn saluda(hola:String){
    //
    println!("{}",hola);
} // variable hola es eliminada

fn suma(x:i32,y:i32){
    println!("{}",x+y);
    // paso 1.2 println!("{:p},{:p}",&y,&x);
} // la variable x e y son eliminadas dentro de scope