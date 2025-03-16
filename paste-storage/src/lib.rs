use async_trait::async_trait;
use std::sync::Arc;

mod memory;
mod mongo;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn get(&self, key: String) -> Option<String>;
    async fn set(&self, key: String, value: String);
}

pub enum StorageType {
    InMemory,
    MongoDB(String),
}

pub async fn get_storage(t: StorageType) -> Arc<dyn Storage> {
    match t {
        StorageType::InMemory => Arc::new(memory::Storage::new()),
        StorageType::MongoDB(db_uri) => Arc::new(mongo::Storage::new(db_uri).await),
    }
}
