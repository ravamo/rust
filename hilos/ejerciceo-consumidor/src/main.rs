use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let productor = thread::spawn(move || {
        for i in 0..5 {
            println!("Produciendo: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let consumidor = thread::spawn(move || {
        for recibido in rx {
            println!("Consumiendo: {}", recibido);
        }
    });

    productor.join().unwrap();
    consumidor.join().unwrap();
}
