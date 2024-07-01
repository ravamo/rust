use tokio::time::{sleep, Duration};


async fn wait_for(seconds: u64) {
    println!("Esperando {} segundos...", seconds);
    sleep(Duration::from_secs(seconds)).await;
    println!("¡Tiempo terminado!");
}


#[tokio::main]
async fn main() {
    let seconds = 5;
    wait_for(seconds).await;
}
