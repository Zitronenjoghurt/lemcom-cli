use mongodb::{Client, options::ClientOptions, Database};
use tokio::sync::RwLock;

pub async fn connect(uri: &str) -> Result<RwLock<Database>, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(RwLock::new(client.database("LemCom")))
}