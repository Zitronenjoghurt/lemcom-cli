use std::{collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use mongodb::{bson::doc, error::Error, Database};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub key: String,
    pub name: String,
    pub display_name: String,
    pub created_stamp: u64,
    pub last_access_stamp: u64,
    pub endpoint_usage: HashMap<String, u64>,
    pub join_date_public: bool,
    pub online_date_public: bool
}

pub async fn execute(database: RwLock<Database>,username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let database = database.read().await;
    let collection = database.collection::<User>("users");
    
    let key = Uuid::new_v4().simple().to_string();
    let name = username.to_string().to_lowercase();

    let name_exists = collection
        .count_documents(doc! { "name": &name }, None)
        .await?;
    let key_exists = collection
        .count_documents(doc! { "key": &key }, None)
        .await?;

    if name_exists > 0 || key_exists > 0 {
        return Err(Box::new(Error::custom("User with same key or name already exists")));
    }

    let user = User {
        key: key.clone(),
        name: name.clone(),
        display_name: name,
        created_stamp: timestamp_now_nanos(),
        last_access_stamp: 0,
        endpoint_usage: HashMap::new(),
        join_date_public: true,
        online_date_public: true
    };

    collection.insert_one(user, None).await?;
    println!("Created user with API Key: {}", key);
    Ok(())
}

pub fn timestamp_now_nanos() -> u64 {
    let start_time = SystemTime::now();
    let since_unix = start_time.duration_since(UNIX_EPOCH).expect("Somehow the time went backwards...");
    since_unix.as_nanos() as u64
}