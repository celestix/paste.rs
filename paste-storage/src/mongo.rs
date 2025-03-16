use super::Storage as StorageTrait;
use async_trait::async_trait;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

const DB_NAME: &str = "paste_db";
const COL_NAME: &str = "pastes";

#[derive(Clone)]
pub struct Storage {
    client: mongodb::Client,
}

#[derive(Serialize, Deserialize)]
struct Paste {
    name: String,
    value: String,
}

impl Storage {
    pub async fn new(db_uri: String) -> Self {
        Storage {
            client: mongodb::Client::with_uri_str(&db_uri).await.unwrap(),
        }
    }
}

#[async_trait]
impl StorageTrait for Storage {
    async fn get(&self, key: String) -> Option<String> {
        let col: mongodb::Collection<Paste> = self.client.database(DB_NAME).collection(COL_NAME);
        match col.find_one(doc! { "name": key }).await {
            Ok(Some(p)) => Some(p.value),
            _ => None,
        }
    }

    async fn set(&self, key: String, value: String) {
        let col: mongodb::Collection<Paste> = self.client.database(DB_NAME).collection(COL_NAME);
        col.insert_one(Paste { name: key, value }).await.unwrap();
    }
}
