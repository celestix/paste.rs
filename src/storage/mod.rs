use std::sync::Arc;

pub mod memory;

pub trait Storage: Send + Sync {
    fn get(&self, key: String) -> Option<String>;
    fn set(&self, key: String, value: String);
}

pub fn get_storage() -> Arc<dyn Storage> {
    Arc::new(memory::Storage::new())
}