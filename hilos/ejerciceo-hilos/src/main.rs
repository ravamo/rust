use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Función que realiza un cálculo
fn realizar_calculo(data: Arc<Mutex<Vec<i32>>>) {
    for i in 0..10 {
        let mut datos = data.lock().unwrap();
        datos.push(i * 2); // Realiza un cálculo (doble del índice)
        println!("Calculado: {}", i * 2);
        thread::sleep(Duration::from_millis(100)); // Simula una operación costosa
    }
}

// Función que almacena resultados
fn almacenar_resultados(data: Arc<Mutex<Vec<i32>>>) {
    loop {
        let datos = data.lock().unwrap();
        if !datos.is_empty() {
            println!("Resultados almacenados: {:?}", *datos);
            break;
        }
        thread::sleep(Duration::from_millis(100)); // Espera antes de intentar de nuevo
    }
}

fn main() {
    // Datos compartidos
    let datos_compartidos = Arc::new(Mutex::new(Vec::new()));

    // Clonamos los datos compartidos para pasarlos a los hilos
    let datos_para_calculo = Arc::clone(&datos_compartidos);
    let datos_para_almacenamiento = Arc::clone(&datos_compartidos);

    // Crear hilo para realizar cálculos
    let handle_calculo = thread::spawn(move || {
        realizar_calculo(datos_para_calculo);
    });

    // Crear hilo para almacenar resultados
    let handle_almacenamiento = thread::spawn(move || {
        almacenar_resultados(datos_para_almacenamiento);
    });

    // Esperar a que los hilos terminen su ejecución
    handle_calculo.join().unwrap();
    handle_almacenamiento.join().unwrap();

    println!("Programa finalizado.");
}
