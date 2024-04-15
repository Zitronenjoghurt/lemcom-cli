extern crate dotenv;

use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;
use tokio::sync::RwLock;

pub async fn connect() -> Result<RwLock<Database>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let url = env::var("DB_URL").expect("Database URL not set.");

    let client_options = ClientOptions::parse(url).await?;
    let client = Client::with_options(client_options)?;
    Ok(RwLock::new(client.database("LemCom")))
}
