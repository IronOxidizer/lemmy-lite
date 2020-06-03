use actix_web::client::Client;
use actix_web::{Result};

pub async fn get_post_list() -> Result<()> {
    let client = Client::default();

    println!("Building and sending request");
    let response = client.get("http://httpbin.org/get")
       .header("User-Agent", "Actix-web")
       .send().await;
    println!("Response: {:?}", response);
    Ok(())
}