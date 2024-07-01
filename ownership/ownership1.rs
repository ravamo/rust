#[warn(unused_variables)]
/*fn main() {
 // tercera regla
 // x no es valida aqui
 let x = 10; // es valida aqui
}// sale del scope aqui ya no podemos usar x 
*/

fn main(){
 // paso 1
 /* let x = 10;
  let y = 20;
  println!("{},{}",x,y);
  println!("{:p},{:p}",&x,&y);*/
  
  
  // paso 2
  /*let x = 10;
  let y = x;
  println!("{},{}",x,y);
  println!("{:p},{:p}",&x,&y);
  */
  // funciona por que clona el valor, lo guarda en la pila y se lo asigna a y con el valor de X (clonado)
  // por eso tiene referencia distintas.
  
  // paso 3
  /*let x = String::from("Hola Mundo");
  let y = String::from("Hola Mundo");
  println!("{},{}",x,y);
  println!("{:p},{:p}",&x,&y);*/
  
  // paso 3.1
  /*let x = String::from("Hola Mundo");
  let y = x
  println!("{},{}",x,y);
  println!("{:p},{:p}",&x,&y);*/
  
  
  // paso 3.2
  let x = String::from("Hola Mundo");
  let y = x.clone();
  println!("{},{}",x,y);
  println!("{:p},{:p}",&x,&y);
  
}
}