use reqwest::Error;


async fn fetch_url(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    match fetch_url(url).await {
        Ok(content) => println!("Contenido: {}", content),
        Err(e) => println!("Error al obtener la URL: {}", e),
    }
}

