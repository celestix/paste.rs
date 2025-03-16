use std::{collections::HashMap, sync::RwLock};

use super::Storage as StorageTrait;

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

impl StorageTrait for Storage {
    fn get(&self, key: String) -> Option<String> {
        self.map.read().unwrap().get(&key).cloned()
    }

    fn set(&self, key: String, value: String) {
        self.map.write().unwrap().insert(key, value);
    }
}
