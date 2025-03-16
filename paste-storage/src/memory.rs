use super::Storage as StorageTrait;
use async_trait::async_trait;
use std::{collections::HashMap, sync::RwLock};

pub struct Storage {
    map: RwLock<HashMap<String, String>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            map: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl StorageTrait for Storage {
    async fn get(&self, key: String) -> Option<String> {
        self.map.read().unwrap().get(&key).cloned()
    }

    async fn set(&self, key: String, value: String) {
        self.map.write().unwrap().insert(key, value);
    }
}
