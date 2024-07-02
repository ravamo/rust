Vamos a implementar un programa en Rust que utilice hilos para realizar operaciones concurrentes de cálculo y almacenamiento de resultados. Usaremos `Arc` y `Mutex` para sincronizar el acceso a los datos compartidos.

### Paso 1: Crear una función que realice un cálculo y otra que almacene resultados

Primero, definimos dos funciones: una para realizar un cálculo y otra para almacenar los resultados.

### Paso 2: Crear hilos para cada función

Luego, creamos los hilos correspondientes.

### Paso 3: Usar `Arc` y `Mutex` para sincronizar el acceso a los datos compartidos

Para asegurar la sincronización, utilizamos `Arc` y `Mutex`.

Aquí está el código completo:

```rust
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
```
1. **Introducción al programa:**
   - Este programa demuestra cómo usar hilos en Rust para realizar operaciones concurrentes.
   - Usamos `Arc` y `Mutex` para sincronizar el acceso a los datos compartidos entre los hilos.

2. **Explicación del código:**
   - `realizar_calculo`: Esta función simula un cálculo que toma tiempo, por ejemplo, multiplicando el índice por 2 y luego durmiendo por 100 ms.
   - `almacenar_resultados`: Esta función verifica si hay resultados calculados y los imprime, simulando el almacenamiento de resultados.
   - `datos_compartidos`: Es un `Arc` que contiene un `Mutex` protegiendo un vector de enteros, que será utilizado por ambos hilos.
   - Se crean dos hilos, uno para `realizar_calculo` y otro para `almacenar_resultados`, cada uno operando sobre el mismo conjunto de datos compartidos.

3. **Ejemplo de ejecución:**
   - Cuando el programa se ejecuta, verás en la consola la salida de los cálculos realizados y finalmente los resultados almacenados.

4. **Conclusión:**
   - Usar `Arc` y `Mutex` en Rust permite manejar la concurrencia de manera segura, evitando condiciones de carrera y asegurando el acceso sincronizado a los datos compartidos.
   - Este ejemplo básico puede ser extendido y adaptado a casos de uso más complejos en aplicaciones concurrentes y paralelas.


